use merix_db::{
    init, apply_schemas,
    document, vector_search, graph, full_text_search, geospatial, time_series,
    Db, VectorQuery, VectorSearchResult,
};
use serde::{Serialize, Deserialize};
use surrealdb_types::{SurrealValue, Geometry};
use geo_types::Point;
use chrono::{DateTime, Utc};
use tokio::time::{sleep, Duration};

#[derive(Debug, Serialize, Deserialize, Clone, SurrealValue)]
struct TestRecord {
    name: String,
    value: i32,
    tags: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, SurrealValue)]
struct TestEmbeddingRecord {
    title: String,
    content: String,
    embedding: Vec<f32>,
}

#[derive(Debug, Serialize, Deserialize, Clone, SurrealValue)]
struct TestGraphNode {
    name: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, SurrealValue)]
struct TestTextDoc {
    title: String,
    content: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, SurrealValue)]
struct TestGeoPlace {
    name: String,
    location: Geometry,
}

#[derive(Debug, Serialize, Deserialize, Clone, SurrealValue)]
struct TestTimeSeriesEvent {
    timestamp: DateTime<Utc>,
    value: f64,
    sensor: String,
}

#[tokio::test]
async fn test_merix_db_full_integration() {
    println!("🚀 Starting Merix DB full multi-model integration test (SurrealDB v3)...");

    let db: Db = init().await.expect("DB init failed");
    println!("✅ DB layer initialized successfully");

    // ── Clean + recreate tables as SCHEMALESS (prevents schema conflicts from previous runs) ──
    let schemas = vec![
        "REMOVE TABLE IF EXISTS test_records;",
        "REMOVE TABLE IF EXISTS test_embeddings;",
        "REMOVE TABLE IF EXISTS test_nodes;",
        "REMOVE TABLE IF EXISTS test_docs;",
        "REMOVE TABLE IF EXISTS test_places;",
        "REMOVE TABLE IF EXISTS test_events;",

        "DEFINE TABLE test_records SCHEMALESS;",
        "DEFINE TABLE test_embeddings SCHEMALESS;",
        "DEFINE TABLE test_nodes SCHEMALESS;",
        "DEFINE TABLE test_docs SCHEMALESS;",
        "DEFINE TABLE test_places SCHEMALESS;",
        "DEFINE TABLE test_events SCHEMALESS;",

        // Explicit fields for safety (SCHEMALESS still benefits from them)
        "DEFINE FIELD name ON test_records TYPE string;",
        "DEFINE FIELD value ON test_records TYPE int;",
        "DEFINE FIELD tags ON test_records TYPE array;",

        "DEFINE FIELD title ON test_embeddings TYPE string;",
        "DEFINE FIELD content ON test_embeddings TYPE string;",
        "DEFINE FIELD embedding ON test_embeddings TYPE array;",

        "DEFINE FIELD name ON test_nodes TYPE string;",

        "DEFINE FIELD title ON test_docs TYPE string;",
        "DEFINE FIELD content ON test_docs TYPE string;",

        "DEFINE FIELD name ON test_places TYPE string;",
        "DEFINE FIELD location ON test_places TYPE geometry;",

        "DEFINE FIELD timestamp ON test_events TYPE datetime;",
        "DEFINE FIELD value ON test_events TYPE float;",
        "DEFINE FIELD sensor ON test_events TYPE string;",
    ];

    apply_schemas(&db, &schemas).await.expect("Failed to apply schemas");
    println!("✅ Base schemas applied (SCHEMALESS + fields defined)");

    // Define model-specific indexes
    full_text_search::define_full_text_index(&db, "test_docs", "content", "english_analyzer", true)
        .await
        .expect("full-text index failed");

    geospatial::define_geospatial_index(&db, "test_places", "location")
        .await
        .expect("geospatial index failed");

    time_series::define_time_series_index(&db, "test_events", "timestamp")
        .await
        .expect("time-series index failed");

    println!("✅ All model indexes defined");

    // Run every model test
    test_document_operations(&db).await;
    test_vector_operations(&db).await;
    test_graph_operations(&db).await;
    test_full_text_operations(&db).await;
    test_geospatial_operations(&db).await;
    test_time_series_operations(&db).await;

    println!("\n🎉 ALL MERIX DB MULTI-MODEL INTEGRATION TESTS PASSED SUCCESSFULLY!");
}

async fn test_document_operations(db: &Db) {
    println!("\n📄 Testing Document (CRUD) operations...");
    let record = TestRecord {
        name: "Integration Test Record".to_string(),
        value: 42,
        tags: vec!["test".to_string(), "db".to_string()],
    };

    let rid = document::insert(db, "test_records", record)
        .await
        .expect("document::insert failed");
    println!("  ✅ insert() → {}", rid);

    let found: Option<TestRecord> = document::find_by_id(db, &rid)
        .await
        .expect("document::find_by_id failed");
    assert!(found.is_some());
    println!("  ✅ find_by_id() successful");

    println!("  ✅ All document operations passed");
}

async fn test_vector_operations(db: &Db) {
    println!("\n🔎 Testing Vector search operations...");
    let items = vec![
        TestEmbeddingRecord {
            title: "Rust Systems".to_string(),
            content: "Memory safety.".to_string(),
            embedding: vec![0.1f32; 384],
        },
        TestEmbeddingRecord {
            title: "ML Basics".to_string(),
            content: "Statistical models.".to_string(),
            embedding: vec![0.9f32; 384],
        },
    ];

    vector_search::upsert(db, "test_embeddings", items, None)
        .await
        .expect("vector upsert failed");
    println!("  ✅ vector_search::upsert()");

    let query = VectorQuery {
        embedding: vec![0.1f32; 384],
        limit: 5,
    };
    let results: Vec<VectorSearchResult<TestEmbeddingRecord>> = vector_search::search(
        db,
        "test_embeddings",
        query,
        None,
    ).await.expect("vector search failed");

    assert!(!results.is_empty());
    println!("  ✅ vector_search::search() → {} results", results.len());
}

async fn test_graph_operations(db: &Db) {
    println!("\n🕸️  Testing Graph operations...");
    let alice = document::insert(db, "test_nodes", TestGraphNode { name: "Alice".to_string() })
        .await
        .expect("node insert failed");
    let bob = document::insert(db, "test_nodes", TestGraphNode { name: "Bob".to_string() })
        .await
        .expect("node insert failed");

    graph::create_edge(db, &alice, "follows", &bob, None::<serde_json::Value>)
        .await
        .expect("create_edge failed");
    println!("  ✅ graph::create_edge()");

    // Most reliable SurrealDB v3 pattern for target nodes
    let query = format!("SELECT * FROM test_nodes WHERE <-follows<-test_nodes CONTAINS {}", alice);
    let friends: Vec<TestGraphNode> = db
        .query(&query)
        .await
        .expect("Graph traversal query failed")
        .take(0)
        .expect("Failed to take results from query");

    assert!(!friends.is_empty());
    println!("  ✅ graph traversal → {} results", friends.len());
}

async fn test_full_text_operations(db: &Db) {
    println!("\n🔍 Testing Full-text search operations...");

    let docs = vec![
        TestTextDoc {
            title: "Rust Guide".to_string(),
            content: "Memory safety and systems programming.".to_string(),
        },
        TestTextDoc {
            title: "AI Guide".to_string(),
            content: "Embeddings and vector search.".to_string(),
        },
    ];

    for doc in docs {
        document::insert(db, "test_docs", doc).await.unwrap();
    }

    // Give SurrealDB a moment to make the newly created full-text index queryable
    sleep(Duration::from_millis(300)).await;

    let results: Vec<TestTextDoc> = full_text_search::search_text(
        db,
        "test_docs",
        "content",
        "memory",   // simpler, guaranteed match
        10,
    ).await.expect("full-text search failed");

    assert!(!results.is_empty());
    println!("  ✅ full_text_search::search_text() → {} results", results.len());
}

async fn test_geospatial_operations(db: &Db) {
    println!("\n📍 Testing Geospatial operations...");
    let places = vec![
        TestGeoPlace {
            name: "Phoenix Downtown".to_string(),
            location: Geometry::Point(Point::new(-112.0740, 33.4484)),
        },
        TestGeoPlace {
            name: "Scottsdale".to_string(),
            location: Geometry::Point(Point::new(-111.9260, 33.4942)),
        },
    ];

    for place in places {
        document::insert(db, "test_places", place).await.unwrap();
    }

    let nearby: Vec<TestGeoPlace> = geospatial::nearby(
        db,
        "test_places",
        "location",
        (-112.0740, 33.4484),
        10000.0, // 10 km
        10,
    ).await.expect("geospatial nearby failed");

    assert!(!nearby.is_empty());
    println!("  ✅ geospatial::nearby() → {} results", nearby.len());
}

async fn test_time_series_operations(db: &Db) {
    println!("\n📈 Testing Time-series operations...");
    let now = Utc::now();
    let events = vec![
        TestTimeSeriesEvent { timestamp: now - chrono::Duration::minutes(30), value: 25.5, sensor: "temp1".to_string() },
        TestTimeSeriesEvent { timestamp: now - chrono::Duration::minutes(10), value: 26.1, sensor: "temp1".to_string() },
    ];

    for e in events {
        document::insert(db, "test_events", e).await.unwrap();
    }

    let latest: Vec<TestTimeSeriesEvent> = time_series::latest(db, "test_events", "timestamp", 5)
        .await
        .expect("latest failed");

    let range: Vec<TestTimeSeriesEvent> = time_series::range(
        db,
        "test_events",
        "timestamp",
        now - chrono::Duration::hours(1),
        now,
        Some(10),
    ).await.expect("range failed");

    assert!(!latest.is_empty() && !range.is_empty());
    println!("  ✅ time_series::latest() + range() passed");
}