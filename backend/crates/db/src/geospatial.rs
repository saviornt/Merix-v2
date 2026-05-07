use crate::Db;
use merix_core::MerixError;
use surrealdb_types::{Value, SurrealValue, Geometry};
use geo_types::Point;
use tracing;

/// Production-ready geospatial queries for SurrealDB v3.
///
/// Uses `geo::distance()` (meters) for point-based radius searches.
/// Records must have a field of type `geometry::point([lon, lat])`.
pub async fn nearby<T: SurrealValue>(
    db: &Db,
    collection: &str,
    location_field: &str,
    center_lon_lat: (f64, f64), // (longitude, latitude)
    radius_meters: f64,
    limit: u32,
) -> Result<Vec<T>, MerixError> {
    let raw_query = format!(
        r#"
        SELECT *,
               geo::distance({}, $center) AS distance
        FROM {}
        WHERE geo::distance({}, $center) <= $radius
        ORDER BY distance ASC
        LIMIT $limit
    "#,
        location_field, collection, location_field
    );

    // Bind center as proper Geometry::Point (required by geo::distance)
    let center = Geometry::Point(Point::new(center_lon_lat.0, center_lon_lat.1));

    let raw: Vec<Value> = db
        .query(&raw_query)
        .bind(("center", center))
        .bind(("radius", radius_meters))
        .bind(("limit", limit as i64))
        .await
        .map_err(|e| MerixError::Db(format!("Geospatial nearby query failed: {}", e)))?
        .take(0)
        .map_err(|e| MerixError::Db(e.to_string()))?;

    let mut results = Vec::with_capacity(raw.len());
    for v in raw {
        let record = T::from_value(v)
            .map_err(|e| MerixError::Db(format!("Failed to convert record: {}", e)))?;
        results.push(record);
    }

    tracing::debug!(
        "Geospatial nearby query on {}.{} (center: {}, {}) within {}m returned {} results",
        collection, location_field, center_lon_lat.0, center_lon_lat.1, radius_meters, results.len()
    );
    Ok(results)
}

/// Helper: Define a geospatial index (call once via apply_schemas).
pub async fn define_geospatial_index(
    db: &Db,
    collection: &str,
    field: &str,
) -> Result<(), MerixError> {
    let stmt = format!(
        r#"
        DEFINE INDEX IF NOT EXISTS idx_{field}_geo
            ON TABLE {collection}
            FIELDS {field};
        "#
    );

    db.query(&stmt)
        .await
        .map_err(|e| MerixError::Db(format!("Failed to define geospatial index: {}", e)))?;

    tracing::info!(
        "Geospatial index defined on {}.{}",
        collection, field
    );
    Ok(())
}