use crate::Db;
use merix_core::MerixError;
use serde::Serialize;
use surrealdb_types::{Value, SurrealValue};
use tracing;

/// Production-ready graph operations for SurrealDB v3.
///
/// Edges are first-class records (can carry rich metadata).
/// All functions remain domain-agnostic — you pass record IDs and edge table names.
pub async fn create_edge<T>(
    db: &Db,
    from_id: impl AsRef<str>,
    edge_table: &str,
    to_id: impl AsRef<str>,
    metadata: Option<T>,
) -> Result<String, MerixError>
where
    T: Serialize + SurrealValue,
{
    let from = from_id.as_ref();
    let to = to_id.as_ref();

    let edge: Option<Value> = if let Some(meta) = metadata {
        db.query(&format!("RELATE {from} -> {edge_table} -> {to} CONTENT $meta"))
            .bind(("meta", meta))
            .await
            .map_err(|e| MerixError::Db(format!("Create edge failed: {}", e)))?
            .take(0)
            .map_err(|e| MerixError::Db(e.to_string()))?
    } else {
        db.query(&format!("RELATE {from} -> {edge_table} -> {to}"))
            .await
            .map_err(|e| MerixError::Db(format!("Create edge failed: {}", e)))?
            .take(0)
            .map_err(|e| MerixError::Db(e.to_string()))?
    };

    let edge = edge.ok_or_else(|| MerixError::Db("No edge record created".to_string()))?;
    let edge_id = extract_id_string(&edge)?;

    tracing::debug!("Created graph edge {} ->[{}]→ {}", from, edge_table, to);
    Ok(edge_id)
}

pub async fn traverse<T: SurrealValue>(
    db: &Db,
    start_id: impl AsRef<str>,
    direction: GraphDirection,
    edge_table: &str,
    depth: Option<u32>,
    limit: Option<u32>,
) -> Result<Vec<T>, MerixError> {
    let start = start_id.as_ref();
    let depth_str = depth.map_or_else(|| "".to_string(), |d| format!("[..{}]", d));
    let limit_str = limit.map_or_else(|| "".to_string(), |l| format!("LIMIT {}", l));

    let query = match direction {
        GraphDirection::Out => format!("SELECT ->{}->{} {} FROM {}", edge_table, depth_str, limit_str, start),
        GraphDirection::In => format!("SELECT <-{}<-{} {} FROM {}", edge_table, depth_str, limit_str, start),
        GraphDirection::Both => format!("SELECT ->{}->{} OR <-{}<-{} {} FROM {}", edge_table, depth_str, edge_table, depth_str, limit_str, start),
    };

    let records: Vec<T> = db
        .query(&query)
        .await
        .map_err(|e| MerixError::Db(format!("Graph traversal failed: {}", e)))?
        .take(0)
        .map_err(|e| MerixError::Db(e.to_string()))?;

    tracing::debug!("Traversed {} edges from {} ({} results)", edge_table, start, records.len());
    Ok(records)
}

pub async fn delete_edge(db: &Db, edge_id: impl AsRef<str>) -> Result<(), MerixError> {
    let (table, key) = split_record_id(edge_id.as_ref())?;
    let _: Option<Value> = db
        .delete((table, key))
        .await
        .map_err(|e| MerixError::Db(format!("Delete edge failed: {}", e)))?;
    Ok(())
}

/// Direction for graph traversal
#[derive(Debug, Clone, Copy)]
pub enum GraphDirection {
    Out,   // -> edge_table ->
    In,    // <- edge_table <-
    Both,  // bidirectional
}

/// Helper: extract clean "table:key" string from returned record
fn extract_id_string(value: &Value) -> Result<String, MerixError> {
    match value {
        Value::Object(obj) => {
            if let Some(id_val) = obj.get("id") {
                match id_val {
                    Value::RecordId(rid) => {
                        let key_str = match &rid.key {
                            surrealdb_types::RecordIdKey::String(s) => s.clone(),
                            _ => format!("{:?}", rid.key),
                        };
                        Ok(format!("{}:{}", rid.table, key_str))
                    }
                    Value::String(s) => Ok(s.clone()),
                    _ => Ok(format!("{:?}", id_val)),
                }
            } else {
                Err(MerixError::Db("No 'id' field in returned record".to_string()))
            }
        }
        _ => Err(MerixError::Db("Expected Object from database".to_string())),
    }
}

/// Helper: split "table:key" into tuple that the v3 SDK requires
fn split_record_id(id: &str) -> Result<(&str, &str), MerixError> {
    let parts: Vec<&str> = id.splitn(2, ':').collect();
    if parts.len() != 2 {
        return Err(MerixError::Db(format!("Invalid record ID format: {}", id)));
    }
    Ok((parts[0], parts[1]))
}