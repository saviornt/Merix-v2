use crate::Db;
use crate::schemas::{VectorSearchResult, VectorQuery, HasEmbedding, RecordId};
use merix_core::MerixError;
use serde::{Serialize, de::DeserializeOwned};
use serde_json::json;

/// Upsert vector-enabled documents
pub async fn upsert<T: Serialize + HasEmbedding>(db: &Db, collection: &str, items: Vec<T>) -> Result<(), MerixError> {
    let query = format!("CREATE {} CONTENT $items", collection);
    db.query(&query)
        .bind(("items", json!(items)))
        .await
        .map_err(|e| MerixError::Db(e.to_string()))?;
    Ok(())
}

/// Vector similarity search
pub async fn search<T: DeserializeOwned>(
    db: &Db,
    collection: &str,
    query: VectorQuery,
) -> Result<Vec<VectorSearchResult<T>>, MerixError> {
    let raw_query = format!(r#"
        SELECT *,
               1 - vector::distance::cosine(embedding, $query) AS score
        FROM {}
        WHERE embedding <5> $query
        ORDER BY score DESC
        LIMIT $limit
    "#, collection);

    let raw: Vec<serde_json::Value> = db.query(&raw_query)
        .bind(("query", query.embedding))
        .bind(("limit", query.limit))
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