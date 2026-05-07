use merix_db::{init, document, vector_search, apply_schemas, QueryFilter, VectorQuery, VectorSearchResult, HasEmbedding};
use serde::{Serialize, Deserialize};
use surrealdb_types::{SurrealValue, Value, object};
use serde_json::json;

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

impl HasEmbedding for TestEmbeddingRecord {
    fn embedding(&self) -> Vec<f32> {
        self.embedding.clone()
    }
}

#[tokio::test]
async fn test_merix_db_full_integration() {
    println!("🚀 Starting Merix DB full integration test (SurrealDB v3, production API)...");

    let db = match init().await {
        Ok(db) => {
            println!("✅ DB layer initialized successfully");
            db
        }
        Err(e) => {
            eprintln!("❌ CRITICAL: Failed to initialize DB: {}", e);
            panic!("DB init failed");
        }
    };

    let test_schemas = vec![
        "DEFINE TABLE test_records SCHEMAFULL IF NOT EXISTS;",
        "DEFINE INDEX test_records_idx ON test_records FIELDS name IF NOT EXISTS;",
        "DEFINE TABLE test_embeddings SCHEMAFULL IF NOT EXISTS;",
        "DEFINE INDEX vec_idx ON test_embeddings FIELDS embedding HNSW DIMENSION 1536 DIST COSINE TYPE F32 IF NOT EXISTS;",
    ];
    apply_schemas(&db, &test_schemas).await.expect("test schema application failed");
    println!("✅ Test schemas applied");

    test_basic_crud(&db).await;
    test_vector_operations(&db).await;

    println!("\n🎉 ALL MERIX DB INTEGRATION TESTS PASSED SUCCESSFULLY!");
}

async fn test_basic_crud(db: &merix_db::Db) {
    println!("\n📋 Testing Basic CRUD operations...");

    let record = TestRecord {
        name: "Integration Test Record".to_string(),
        value: 42,
        tags: vec!["test".to_string(), "db".to_string()],
    };

    let rid: String = document::insert(db, "test_records", record.clone())
        .await
        .expect("insert failed");
    println!("  ✅ insert() → ID: {}", rid);

    let batch = vec![
        TestRecord { name: "Batch Item 1".to_string(), value: 100, tags: vec![] },
        TestRecord { name: "Batch Item 2".to_string(), value: 200, tags: vec![] },
    ];
    let batch_ids: Vec<String> = document::insert_many(db, "test_records", batch)
        .await
        .expect("insert_many failed");
    println!("  ✅ insert_many() → {} IDs", batch_ids.len());

    let all: Vec<TestRecord> = document::find_all(db, "test_records")
        .await
        .expect("find_all failed");
    println!("  ✅ find_all() → {} records", all.len());

    let test_rid = batch_ids[0].clone();
    let updates = Value::Object(object! { value: 999i32 });
    document::update(db, &test_rid, updates).await.expect("update failed");
    println!("  ✅ update()");

    let found: Option<TestRecord> = document::find_by_id(db, &test_rid)
        .await
        .expect("find_by_id failed");
    assert!(found.is_some());
    println!("  ✅ find_by_id()");

    document::delete(db, &test_rid).await.expect("delete failed");
    println!("  ✅ delete()");

    document::delete_by_filter(db, "test_records", QueryFilter::Where(json!({"value": 200})))
        .await
        .expect("delete_by_filter failed");
    println!("  ✅ delete_by_filter()");

    println!("  ✅ All basic CRUD operations passed");
}

async fn test_vector_operations(db: &merix_db::Db) {
    println!("\n🔍 Testing Vector operations...");

    let embed_docs = vec![
        TestEmbeddingRecord { title: "Rust Systems".to_string(), content: "Memory safety.".to_string(), embedding: vec![0.1f32; 384] },
        TestEmbeddingRecord { title: "ML Basics".to_string(), content: "Statistical models.".to_string(), embedding: vec![0.9f32; 384] },
    ];

    vector_search::upsert(db, "test_embeddings", embed_docs, None)
        .await
        .expect("vector upsert failed");
    println!("  ✅ vectors::upsert()");

    let query = VectorQuery { embedding: vec![0.1f32; 384], limit: 5 };
    let results: Vec<VectorSearchResult<TestEmbeddingRecord>> = vector_search::search(
        db, "test_embeddings", query, None,
    ).await.expect("vector search failed");

    assert!(!results.is_empty());
    println!("  ✅ vectors::search() → {} results", results.len());

    println!("  ✅ All vector operations passed");
}