use crate::Db;
use merix_core::MerixError;
use surrealdb_types::{Value, SurrealValue};
use tracing;

/// Full-text search for SurrealDB v3.
///
/// Uses the `@@` (matches) operator with BM25 scoring and optional highlighting.
/// Indexes must be defined once via `define_full_text_index`.
pub async fn search_text<T: SurrealValue>(
    db: &Db,
    collection: &str,
    field: &str,
    query: &str,
    limit: u32,
) -> Result<Vec<T>, MerixError> {
    let raw_query = format!(
        r#"
        SELECT *,
               search::score(0) AS score
        FROM {}
        WHERE {} @0@ $query
        ORDER BY score DESC
        LIMIT $limit
    "#,
        collection, field
    );

    let raw: Vec<Value> = db
        .query(&raw_query)
        .bind(("query", query.to_string()))
        .bind(("limit", limit as i64))
        .await
        .map_err(|e| MerixError::Db(format!("Full-text search failed: {}", e)))?
        .take(0)
        .map_err(|e| MerixError::Db(e.to_string()))?;

    let mut results = Vec::with_capacity(raw.len());
    for v in raw {
        let record = T::from_value(v)
            .map_err(|e| MerixError::Db(format!("Failed to convert record: {}", e)))?;
        results.push(record);
    }

    tracing::debug!(
        "Full-text search on {}.{} for '{}' returned {} results",
        collection, field, query, results.len()
    );
    Ok(results)
}

/// Helper: Define a full-text index + analyzer (call once via apply_schemas).
///
/// Uses correct SurrealDB v3 syntax: `FULLTEXT ANALYZER`.
pub async fn define_full_text_index(
    db: &Db,
    collection: &str,
    field: &str,
    analyzer_name: &str,
    with_highlights: bool,
) -> Result<(), MerixError> {
    let highlights = if with_highlights { " HIGHLIGHTS" } else { "" };

    let stmt = format!(
        r#"
        DEFINE ANALYZER IF NOT EXISTS {analyzer_name}
            TOKENIZERS blank,class,punct,camel
            FILTERS lowercase,snowball(english);

        DEFINE INDEX IF NOT EXISTS idx_{field}_ft
            ON TABLE {collection}
            FIELDS {field}
            FULLTEXT ANALYZER {analyzer_name} BM25{highlights};
        "#
    );

    db.query(&stmt)
        .await
        .map_err(|e| MerixError::Db(format!("Failed to define full-text index: {}", e)))?;

    tracing::info!(
        "Full-text index defined on {}.{} using analyzer '{}'",
        collection, field, analyzer_name
    );
    Ok(())
}