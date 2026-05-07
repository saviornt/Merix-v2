use crate::Db;
use crate::schemas::QueryFilter;
use merix_core::MerixError;
use serde::Serialize;
use surrealdb_types::{Value, SurrealValue};

/// Production-ready insert using high-level .create() API (SurrealDB v3)
pub async fn insert<T>(db: &Db, collection: &str, data: T) -> Result<String, MerixError>
where
    T: Serialize + SurrealValue,
{
    let created: Option<Value> = db
        .create(collection)
        .content(data)
        .await
        .map_err(|e| MerixError::Db(format!("Create failed: {}", e)))?;

    let value = created.ok_or_else(|| MerixError::Db("No record created".to_string()))?;

    let id_str = extract_id_string(&value)?;
    Ok(id_str)
}

/// Batch insert using high-level .insert() (optimal for heavy workloads)
pub async fn insert_many<T>(db: &Db, collection: &str, data: Vec<T>) -> Result<Vec<String>, MerixError>
where
    T: Serialize + SurrealValue,
{
    let created: Vec<Value> = db
        .insert(collection)
        .content(data)
        .await
        .map_err(|e| MerixError::Db(format!("Batch insert failed: {}", e)))?;

    let mut ids = Vec::with_capacity(created.len());
    for v in created {
        ids.push(extract_id_string(&v)?);
    }
    Ok(ids)
}

pub async fn update(db: &Db, id: impl AsRef<str>, updates: Value) -> Result<(), MerixError> {
    let (table, key) = split_record_id(id.as_ref())?;
    let _: Option<Value> = db
        .update((table, key))
        .merge(updates)
        .await
        .map_err(|e| MerixError::Db(format!("Update failed: {}", e)))?;
    Ok(())
}

pub async fn find_all<T: SurrealValue>(db: &Db, collection: &str) -> Result<Vec<T>, MerixError> {
    db.select(collection)
        .await
        .map_err(|e| MerixError::Db(format!("Select failed: {}", e)))
}

pub async fn find_by_id<T: SurrealValue>(db: &Db, id: impl AsRef<str>) -> Result<Option<T>, MerixError> {
    let (table, key) = split_record_id(id.as_ref())?;
    let record: Option<T> = db.select((table, key))
        .await
        .map_err(|e| MerixError::Db(format!("Select by id failed: {}", e)))?;
    Ok(record)
}

pub async fn delete(db: &Db, id: impl AsRef<str>) -> Result<(), MerixError> {
    let (table, key) = split_record_id(id.as_ref())?;
    let _: Option<Value> = db
        .delete((table, key))
        .await
        .map_err(|e| MerixError::Db(format!("Delete failed: {}", e)))?;
    Ok(())
}

pub async fn delete_by_filter(db: &Db, collection: &str, filter: QueryFilter) -> Result<(), MerixError> {
    match filter {
        QueryFilter::Where(v) => {
            db.query(&format!("DELETE {} WHERE $filter", collection))
                .bind(("filter", v))
                .await
                .map_err(|e| MerixError::Db(e.to_string()))?;
        }
        QueryFilter::Ids(ids) => {
            db.query(&format!("DELETE {} WHERE id IN $ids", collection))
                .bind(("ids", ids))
                .await
                .map_err(|e| MerixError::Db(e.to_string()))?;
        }
        QueryFilter::Raw(sql) => {
            db.query(&sql).await.map_err(|e| MerixError::Db(e.to_string()))?;
        }
    }
    Ok(())
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