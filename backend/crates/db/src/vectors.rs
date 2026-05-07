use crate::Db;
use crate::schemas::{VectorSearchResult, VectorQuery, HasEmbedding, RecordId};
use merix_core::MerixError;
use serde::Serialize;
use surrealdb_types::{Value, SurrealValue};
use tracing;

/// Upsert with high-level .upsert() for heavy workloads (create-or-update)
pub async fn upsert<T>(db: &Db, collection: &str, items: Vec<T>, ids: Option<Vec<RecordId>>) -> Result<(), MerixError>
where
    T: Serialize + SurrealValue,
{
    let mut bound_items = Vec::with_capacity(items.len());
    let item_count = items.len(); // capture count before the move into the loop

    for (i, item) in items.into_iter().enumerate() {
        let mut value = item.into_value();

        if let Some(ref id_list) = ids {
            if let Some(rid) = id_list.get(i) {
                if let Value::Object(mut obj) = value {
                    obj.insert("id".to_string(), Value::from(rid.as_surreal()));
                    value = Value::Object(obj);
                } else {
                    let mut obj = std::collections::BTreeMap::new();
                    obj.insert("id".to_string(), Value::from(rid.as_surreal()));
                    value = Value::Object(obj.into());
                }
            }
        }

        bound_items.push(value);
    }

    let _: Vec<Value> = db
        .upsert(collection)
        .content(bound_items)
        .await
        .map_err(|e| MerixError::Db(format!("Vector upsert failed: {}", e)))?;

    tracing::debug!("Upserted {} vector records to {}", item_count, collection);
    Ok(())
}

/// Vector search (optimized SurrealQL + cosine distance, production HNSW ready)
pub async fn search<T: SurrealValue>(
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
        .bind(("limit", query.limit as i64));

    if let Some(ref fid) = filter_id {
        q = q.bind(("filter_id", fid.as_surreal()));
    }

    let raw: Vec<Value> = q
        .await
        .map_err(|e| MerixError::Db(format!("Vector search failed: {}", e)))?
        .take(0)
        .map_err(|e| MerixError::Db(e.to_string()))?;

    let mut results = Vec::new();
    for mut v in raw {
        let score = if let Value::Object(ref mut obj) = v {
            if let Some(Value::Number(n)) = obj.remove("score") {
                n.to_f64().unwrap_or(0.0) as f32
            } else {
                0.0
            }
        } else {
            0.0
        };

        let record = T::from_value(v)
            .map_err(|e| MerixError::Db(format!("Failed to convert record: {}", e)))?;

        results.push(VectorSearchResult { record, score });
    }

    Ok(results)
}