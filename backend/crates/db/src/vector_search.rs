use crate::Db;
use crate::schemas::{VectorSearchResult, VectorQuery};
use merix_core::MerixError;
use serde::Serialize;
use surrealdb_types::{Value, SurrealValue};
use tracing;

/// Upsert (or insert) vector records. Supports two modes (SurrealDB v3 compatible).
pub async fn upsert<T>(
    db: &Db,
    collection: &str,
    items: Vec<T>,
    ids: Option<Vec<String>>,
) -> Result<(), MerixError>
where
    T: Serialize + SurrealValue,
{
    let item_count = items.len();

    match ids {
        Some(ids) => {
            if items.len() != ids.len() {
                return Err(MerixError::Db(format!(
                    "Number of items ({}) must match number of provided IDs ({})",
                    items.len(),
                    ids.len()
                )));
            }

            for (item, id) in items.into_iter().zip(ids) {
                let (table, key) = split_record_id(&id)?;
                let _: Option<Value> = db
                    .upsert((table, key))
                    .content(item)
                    .await
                    .map_err(|e| MerixError::Db(format!("Vector upsert failed for record {id}: {e}")))?;
            }
        }
        None => {
            let _: Vec<Value> = db
                .insert(collection)
                .content(items)
                .await
                .map_err(|e| MerixError::Db(format!("Vector bulk insert failed: {e}")))?;
        }
    }

    tracing::debug!(
        "Upserted/inserted {} vector record(s) into collection '{}'",
        item_count,
        collection
    );
    Ok(())
}

/// Production-ready vector search using `vector::similarity::cosine` (stable SurrealDB v3 path).
pub async fn search<T: SurrealValue>(
    db: &Db,
    collection: &str,
    query: VectorQuery,
    filter_id: Option<String>,
) -> Result<Vec<VectorSearchResult<T>>, MerixError> {
    let where_clause = if filter_id.is_some() {
        "WHERE id = $filter_id"
    } else {
        ""
    };

    let raw_query = format!(
        r#"
        SELECT *,
               vector::similarity::cosine(embedding, $query) AS score
        FROM {}
        {}
        ORDER BY score DESC
        LIMIT $limit
    "#,
        collection, where_clause
    );

    let mut q = db
        .query(&raw_query)
        .bind(("query", query.embedding))
        .bind(("limit", query.limit as i64));

    if let Some(fid) = filter_id {
        q = q.bind(("filter_id", fid));
    }

    let raw: Vec<Value> = q
        .await
        .map_err(|e| MerixError::Db(format!("Vector search failed: {}", e)))?
        .take(0)
        .map_err(|e| MerixError::Db(e.to_string()))?;

    let mut results = Vec::with_capacity(raw.len());
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

/// Helper: split "table:key" into tuple that the v3 SDK requires for single-record upsert.
fn split_record_id(id: &str) -> Result<(&str, &str), MerixError> {
    let parts: Vec<&str> = id.splitn(2, ':').collect();
    if parts.len() != 2 {
        return Err(MerixError::Db(format!("Invalid record ID format: {}", id)));
    }
    Ok((parts[0], parts[1]))
}