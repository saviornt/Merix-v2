use crate::Db;
use crate::schemas::{VectorSearchResult, VectorQuery, HasEmbedding, RecordId};
use merix_core::MerixError;
use serde::{Serialize, de::DeserializeOwned};
use serde_json::json;

/// Upsert vector-enabled documents (hybrid create-or-update)
/// If `ids` is Some, it must match the length of `items` and will be injected as the record `id`.
pub async fn upsert<T: Serialize + HasEmbedding>(
    db: &Db,
    collection: &str,
    items: Vec<T>,
    ids: Option<Vec<RecordId>>,
) -> Result<(), MerixError> {
    let mut bound_items = Vec::with_capacity(items.len());

    for (i, item) in items.into_iter().enumerate() {
        let mut value = serde_json::to_value(&item)
            .map_err(|e| MerixError::Db(e.to_string()))?;

        if let Some(ref id_list) = ids {
            if let Some(rid) = id_list.get(i) {
                value["id"] = json!(rid.as_surreal());
            }
        }

        bound_items.push(value);
    }

    let query = format!("UPSERT {} CONTENT $items", collection);

    db.query(&query)
        .bind(("items", json!(bound_items)))
        .await
        .map_err(|e| MerixError::Db(e.to_string()))?;

    Ok(())
}

/// Vector similarity search (hybrid with optional RecordId filter)
pub async fn search<T: DeserializeOwned>(
    db: &Db,
    collection: &str,
    query: VectorQuery,
    filter_id: Option<RecordId>,
) -> Result<Vec<VectorSearchResult<T>>, MerixError> {
    let mut where_clause = String::from("embedding <5> $query");

    if filter_id.is_some() {
        where_clause.push_str(" AND id = $filter_id");
    }

    let raw_query = format!(r#"
        SELECT *,
               1 - vector::distance::cosine(embedding, $query) AS score
        FROM {}
        WHERE {}
        ORDER BY score DESC
        LIMIT $limit
    "#, collection, where_clause);

    let mut q = db.query(&raw_query)
        .bind(("query", query.embedding))
        .bind(("limit", query.limit));

    if let Some(ref fid) = filter_id {
        q = q.bind(("filter_id", fid.as_surreal()));
    }

    let raw: Vec<serde_json::Value> = q
        .await
        .map_err(|e| MerixError::Db(e.to_string()))?
        .take(0)
        .map_err(|e| MerixError::Db(e.to_string()))?;

    let mut results = Vec::new();
    for mut v in raw {
        let score = v.get("score").and_then(|s| s.as_f64()).unwrap_or(0.0) as f32;
        if let Some(obj) = v.as_object_mut() {
            obj.remove("score");
        }
        let record: T = serde_json::from_value(v)
            .map_err(|e| MerixError::Db(e.to_string()))?;
        results.push(VectorSearchResult { record, score });
    }

    Ok(results)
}