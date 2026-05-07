use crate::Db;
use crate::schemas::{RecordId, QueryFilter};
use merix_core::MerixError;
use serde::Serialize;
use surrealdb_types::{Value, SurrealValue};

/// Production-ready insert using high-level .create() API (SurrealDB v3)
pub async fn insert<T>(db: &Db, collection: &str, data: T) -> Result<RecordId, MerixError>
where
    T: Serialize + SurrealValue,
{
    let created: Option<Value> = db
        .create(collection)
        .content(data)
        .await
        .map_err(|e| MerixError::Db(format!("Create failed: {}", e)))?;

    let value = created.ok_or_else(|| MerixError::Db("No record created".to_string()))?;

    let id_str = extract_record_id_string(&value)?;
    let parts: Vec<&str> = id_str.split(':').collect();
    if parts.len() != 2 {
        return Err(MerixError::Db("Malformed SurrealDB id".to_string()));
    }

    Ok(RecordId::new(parts[0].to_string(), parts[1].to_string()))
}

/// Batch insert using high-level .insert() (optimal for heavy workloads)
pub async fn insert_many<T>(db: &Db, collection: &str, data: Vec<T>) -> Result<Vec<RecordId>, MerixError>
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
        let id_str = extract_record_id_string(&v)?;
        let parts: Vec<&str> = id_str.split(':').collect();
        if parts.len() == 2 {
            ids.push(RecordId::new(parts[0].to_string(), parts[1].to_string()));
        } else {
            ids.push(RecordId::random(collection));
        }
    }
    Ok(ids)
}

pub async fn update(db: &Db, id: RecordId, updates: Value) -> Result<(), MerixError> {
    let _: Vec<Value> = db
        .update(id.as_surreal())
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

pub async fn find_by_id<T: SurrealValue>(db: &Db, id: RecordId) -> Result<Option<T>, MerixError> {
    let records: Vec<T> = db.select(id.as_surreal())
        .await
        .map_err(|e| MerixError::Db(format!("Select by id failed: {}", e)))?;
    Ok(records.into_iter().next())
}

pub async fn delete(db: &Db, id: RecordId) -> Result<(), MerixError> {
    let _: Vec<Value> = db
        .delete(id.as_surreal())
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
            let id_list: Vec<String> = ids.into_iter().map(|r| r.as_surreal()).collect();
            db.query(&format!("DELETE {} WHERE id IN $ids", collection))
                .bind(("ids", id_list))
                .await
                .map_err(|e| MerixError::Db(e.to_string()))?;
        }
        QueryFilter::Raw(sql) => {
            db.query(&sql).await.map_err(|e| MerixError::Db(e.to_string()))?;
        }
    }
    Ok(())
}

/// Extract full record ID as "table:key" string from surrealdb_types::Value
fn extract_record_id_string(value: &Value) -> Result<String, MerixError> {
    match value {
        Value::Object(obj) => {
            if let Some(id_val) = obj.get("id") {
                match id_val {
                    Value::RecordId(rid) => {
                        // RecordIdKey does not implement Display in v3; use Debug
                        Ok(format!("{}:{:?}", rid.table, rid.key))
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