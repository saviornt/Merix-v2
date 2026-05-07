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

/// General NoSQL-style CRUD using schema wrappers
pub async fn insert<T: Serialize>(
    db: &Db,
    collection: &str,
    data: T,
    id: Option<RecordId>,
) -> Result<(), MerixError> {
    let mut value = serde_json::to_value(data)
        .map_err(|e| MerixError::Db(e.to_string()))?;

    if let Some(ref rid) = id {
        value["id"] = json!(rid.as_surreal());
    }

    let query = if id.is_some() {
        format!("UPSERT {} CONTENT $data", collection)
    } else {
        format!("CREATE {} CONTENT $data", collection)
    };

    db.query(&query)
        .bind(("data", value))
        .await
        .map_err(|e| MerixError::Db(e.to_string()))?;
    Ok(())
}

pub async fn insert_many<T: Serialize>(
    db: &Db,
    collection: &str,
    data: Vec<T>,
    ids: Option<Vec<RecordId>>,
) -> Result<(), MerixError> {
    let mut bound_items = Vec::with_capacity(data.len());

    for (i, item) in data.into_iter().enumerate() {
        let mut value = serde_json::to_value(item)
            .map_err(|e| MerixError::Db(e.to_string()))?;

        if let Some(ref id_list) = ids {
            if let Some(rid) = id_list.get(i) {
                value["id"] = json!(rid.as_surreal());
            }
        }
        bound_items.push(value);
    }

    let query = if ids.is_some() {
        format!("UPSERT {} CONTENT $data", collection)
    } else {
        format!("CREATE {} CONTENT $data", collection)
    };

    db.query(&query)
        .bind(("data", json!(bound_items)))
        .await
        .map_err(|e| MerixError::Db(e.to_string()))?;
    Ok(())
}

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
        // Defensive handling: Surreal sometimes returns just the ID string for non-existent records
        if let Some(s) = v.as_str() {
            if s.contains(':') {  // looks like a Surreal ID
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