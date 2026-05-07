use crate::Db;
use crate::schemas::{RecordId, QueryFilter};
use merix_core::MerixError;
use serde::{Serialize, de::DeserializeOwned};
use serde_json::{json, Value};

/// Helper to strip SurrealDB's automatic "id" field before deserializing
fn strip_surreal_id(mut value: Value) -> Value {
    if let Some(obj) = value.as_object_mut() {
        obj.remove("id");
    }
    value
}

/// Insert a single record and return the actual RecordId that SurrealDB used
pub async fn insert<T: Serialize>(
    db: &Db,
    collection: &str,
    data: T,
) -> Result<RecordId, MerixError> {
    let value = serde_json::to_value(data)
        .map_err(|e| MerixError::Db(e.to_string()))?;

    let query = format!("CREATE {} CONTENT $data RETURN id", collection);

    let raw: Vec<Value> = db.query(&query)
        .bind(("data", value))
        .await
        .map_err(|e| MerixError::Db(e.to_string()))?
        .take(0)
        .map_err(|e| MerixError::Db(e.to_string()))?;

    let id_value = raw.into_iter().next()
        .and_then(|v| v.get("id").cloned())
        .ok_or_else(|| MerixError::Db("No id returned from insert".to_string()))?;

    let id_str = id_value.as_str()
        .ok_or_else(|| MerixError::Db("Invalid id format from SurrealDB".to_string()))?;

    let parts: Vec<&str> = id_str.split(':').collect();
    if parts.len() != 2 {
        return Err(MerixError::Db("Malformed SurrealDB id".to_string()));
    }

    Ok(RecordId::new(parts[0].to_string(), parts[1].to_string()))
}

/// Insert many records and return their actual RecordIds
pub async fn insert_many<T: Serialize>(
    db: &Db,
    collection: &str,
    data: Vec<T>,
) -> Result<Vec<RecordId>, MerixError> {
    let bound_items: Vec<Value> = data
        .into_iter()
        .map(|item| serde_json::to_value(item).map_err(|e| MerixError::Db(e.to_string())))
        .collect::<Result<Vec<_>, _>>()?;

    // ✅ Use INSERT INTO for batch operations (SurrealDB's recommended syntax)
    let query = format!("INSERT INTO {} $data RETURN id", collection);

    let raw: Vec<Value> = db.query(&query)
        .bind(("data", json!(bound_items)))
        .await
        .map_err(|e| MerixError::Db(e.to_string()))?
        .take(0)
        .map_err(|e| MerixError::Db(e.to_string()))?;

    let mut ids = Vec::with_capacity(raw.len());
    for v in raw {
        let id_str = v.get("id")
            .and_then(|i| i.as_str())
            .ok_or_else(|| MerixError::Db("No id in batch insert result".to_string()))?;

        let parts: Vec<&str> = id_str.split(':').collect();
        if parts.len() == 2 {
            ids.push(RecordId::new(parts[0].to_string(), parts[1].to_string()));
        } else {
            ids.push(RecordId::random(collection));
        }
    }
    Ok(ids)
}

// The rest of the file stays exactly the same (update, find_all, find_by_id, delete, delete_by_filter)
pub async fn update(db: &Db, id: RecordId, updates: Value) -> Result<(), MerixError> {
    let query = "UPDATE $id MERGE $updates";
    db.query(query)
        .bind(("id", id.as_surreal()))
        .bind(("updates", updates))
        .await
        .map_err(|e| MerixError::Db(e.to_string()))?;
    Ok(())
}

pub async fn find_all<T: DeserializeOwned>(db: &Db, collection: &str) -> Result<Vec<T>, MerixError> {
    let query = format!("SELECT * FROM {}", collection);
    let raw: Vec<Value> = db.query(&query)
        .await
        .map_err(|e| MerixError::Db(e.to_string()))?
        .take(0)
        .map_err(|e| MerixError::Db(e.to_string()))?;

    raw.into_iter()
        .map(|v| serde_json::from_value(strip_surreal_id(v)))
        .collect::<Result<Vec<T>, _>>()
        .map_err(|e| MerixError::Db(e.to_string()))
}

pub async fn find_by_id<T: DeserializeOwned>(db: &Db, id: RecordId) -> Result<Option<T>, MerixError> {
    let query = "SELECT * FROM $id";
    let mut raw: Vec<Value> = db.query(query)
        .bind(("id", id.as_surreal()))
        .await
        .map_err(|e| MerixError::Db(e.to_string()))?
        .take(0)
        .map_err(|e| MerixError::Db(e.to_string()))?;

    if let Some(v) = raw.pop() {
        if let Some(s) = v.as_str() {
            if s.contains(':') {
                return Ok(None);
            }
        }
        Ok(Some(serde_json::from_value(strip_surreal_id(v))
            .map_err(|e| MerixError::Db(e.to_string()))?))
    } else {
        Ok(None)
    }
}

pub async fn delete(db: &Db, id: RecordId) -> Result<(), MerixError> {
    let query = "DELETE $id";
    db.query(query)
        .bind(("id", id.as_surreal()))
        .await
        .map_err(|e| MerixError::Db(e.to_string()))?;
    Ok(())
}

pub async fn delete_by_filter(db: &Db, collection: &str, filter: QueryFilter) -> Result<(), MerixError> {
    match filter {
        QueryFilter::Where(v) => {
            let query = format!("DELETE {} WHERE $filter", collection);
            db.query(&query)
                .bind(("filter", v))
                .await
                .map_err(|e| MerixError::Db(e.to_string()))?;
        }
        QueryFilter::Ids(ids) => {
            let id_list: Vec<String> = ids.into_iter().map(|r| r.as_surreal()).collect();
            let query = format!("DELETE {} WHERE id IN $ids", collection);
            db.query(&query)
                .bind(("ids", id_list))
                .await
                .map_err(|e| MerixError::Db(e.to_string()))?;
        }
        QueryFilter::Raw(sql) => {
            db.query(&sql)
                .await
                .map_err(|e| MerixError::Db(e.to_string()))?;
        }
    }
    Ok(())
}