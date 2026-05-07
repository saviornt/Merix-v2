use crate::Db;
use merix_core::MerixError;
use surrealdb_types::{Value, SurrealValue};
use tracing;
use chrono::{DateTime, Utc};

/// Production-ready time-series queries for SurrealDB v3.
///
/// Uses timestamp fields for efficient range and latest-N queries.
/// Add a timestamp index once via `define_time_series_index` for best performance.
pub async fn range<T: SurrealValue>(
    db: &Db,
    collection: &str,
    timestamp_field: &str,
    start: DateTime<Utc>,
    end: DateTime<Utc>,
    limit: Option<u32>,
) -> Result<Vec<T>, MerixError> {
    let limit_clause = limit.map_or_else(String::new, |l| format!("LIMIT {}", l));

    let raw_query = format!(
        r#"
        SELECT *
        FROM {}
        WHERE {} >= $start AND {} <= $end
        ORDER BY {} ASC
        {}
    "#,
        collection, timestamp_field, timestamp_field, timestamp_field, limit_clause
    );

    let raw: Vec<Value> = db
        .query(&raw_query)
        .bind(("start", start))
        .bind(("end", end))
        .await
        .map_err(|e| MerixError::Db(format!("Time-series range query failed: {}", e)))?
        .take(0)
        .map_err(|e| MerixError::Db(e.to_string()))?;

    let mut results = Vec::with_capacity(raw.len());
    for v in raw {
        let record = T::from_value(v)
            .map_err(|e| MerixError::Db(format!("Failed to convert record: {}", e)))?;
        results.push(record);
    }

    tracing::debug!(
        "Time-series range on {}.{} from {} to {} returned {} results",
        collection, timestamp_field, start, end, results.len()
    );
    Ok(results)
}

/// Get the most recent N records ordered by timestamp DESC.
pub async fn latest<T: SurrealValue>(
    db: &Db,
    collection: &str,
    timestamp_field: &str,
    limit: u32,
) -> Result<Vec<T>, MerixError> {
    let raw_query = format!(
        r#"
        SELECT *
        FROM {}
        ORDER BY {} DESC
        LIMIT $limit
    "#,
        collection, timestamp_field
    );

    let raw: Vec<Value> = db
        .query(&raw_query)
        .bind(("limit", limit as i64))
        .await
        .map_err(|e| MerixError::Db(format!("Time-series latest query failed: {}", e)))?
        .take(0)
        .map_err(|e| MerixError::Db(e.to_string()))?;

    let mut results = Vec::with_capacity(raw.len());
    for v in raw {
        let record = T::from_value(v)
            .map_err(|e| MerixError::Db(format!("Failed to convert record: {}", e)))?;
        results.push(record);
    }

    tracing::debug!(
        "Time-series latest on {}.{} returned {} results",
        collection, timestamp_field, results.len()
    );
    Ok(results)
}

/// Helper: Define a time-series index on a timestamp field (call once via apply_schemas).
///
/// This creates a standard index optimized for range scans.
pub async fn define_time_series_index(
    db: &Db,
    collection: &str,
    timestamp_field: &str,
) -> Result<(), MerixError> {
    let stmt = format!(
        r#"
        DEFINE INDEX IF NOT EXISTS idx_{timestamp_field}_ts
            ON TABLE {collection}
            FIELDS {timestamp_field};
        "#
    );

    db.query(&stmt)
        .await
        .map_err(|e| MerixError::Db(format!("Failed to define time-series index: {}", e)))?;

    tracing::info!(
        "Time-series index defined on {}.{}",
        collection, timestamp_field
    );
    Ok(())
}