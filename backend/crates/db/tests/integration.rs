use merix_db::{init, operations, vectors, RecordId, VectorQuery, HasEmbedding, VectorSearchResult, QueryFilter};
use serde::{Serialize, Deserialize};
use serde_json::json;

#[derive(Debug, Serialize, Deserialize, Clone)]
struct TestRecord {
    name: String,
    value: i32,
    tags: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct TestEmbeddingRecord {
    title: String,
    content: String,
    embedding: Vec<f32>,
}

impl HasEmbedding for TestEmbeddingRecord {
    fn embedding(&self) -> &[f32] {
        &self.embedding
    }
}

#[tokio::test]
async fn test_merix_db_full_integration() {
    println!("🧪 Starting Merix DB full integration test (with RecordId returns)...");

    let db = match init().await {
        Ok(db) => {
            println!("✅ DB initialized successfully");
            db
        }
        Err(e) => {
            eprintln!("❌ CRITICAL: Failed to initialize DB: {}", e);
            panic!("DB init failed");
        }
    };

    test_basic_crud(&db).await;
    test_vector_operations(&db).await;

    println!("\n🎉 ALL MERIX DB INTEGRATION TESTS PASSED SUCCESSFULLY! 🚀");
}

async fn test_basic_crud(db: &merix_db::Db) {
    println!("\n📋 Testing Basic CRUD...");

    let record = TestRecord {
        name: "Integration Test Record".to_string(),
        value: 42,
        tags: vec!["test".to_string(), "db".to_string()],
    };

    // ✅ insert now returns the real RecordId that SurrealDB assigned
    let rid = operations::insert(db, "test_records", record.clone())
        .await
        .expect("insert failed");
    println!("  ✅ insert() → returned RecordId: {}", rid.as_surreal());

    let batch = vec![
        TestRecord { name: "Batch Item 1".to_string(), value: 100, tags: vec![] },
        TestRecord { name: "Batch Item 2".to_string(), value: 200, tags: vec![] },
    ];
    let batch_ids = operations::insert_many(db, "test_records", batch)
        .await
        .expect("insert_many failed");
    println!("  ✅ insert_many() → {} RecordIds returned", batch_ids.len());

    let all: Vec<TestRecord> = operations::find_all(db, "test_records")
        .await
        .expect("find_all failed");
    println!("  ✅ find_all() - {} records", all.len());

    operations::update(db, rid.clone(), json!({"value": 999}))
        .await
        .expect("update failed");
    println!("  ✅ update()");

    let found: Option<TestRecord> = operations::find_by_id(db, rid.clone())
        .await
        .expect("find_by_id failed");
    assert!(found.is_some(), "find_by_id should return the record");
    println!("  ✅ find_by_id()");

    operations::delete(db, rid.clone()).await.expect("delete failed");
    println!("  ✅ delete()");

    operations::delete_by_filter(db, "test_records", QueryFilter::Where(json!({"value": 200})))
        .await
        .expect("delete_by_filter failed");
    println!("  ✅ delete_by_filter()");

    println!("  ✅ All basic CRUD passed");
}

async fn test_vector_operations(db: &merix_db::Db) {
    println!("\n🔬 Testing Vector operations (hybrid + RecordId)...");

    let embed_docs = vec![
        TestEmbeddingRecord {
            title: "Rust Systems".to_string(),
            content: "Memory safety and performance.".to_string(),
            embedding: vec![0.1f32; 384],
        },
        TestEmbeddingRecord {
            title: "ML Basics".to_string(),
            content: "Statistical models.".to_string(),
            embedding: vec![0.9f32; 384],
        },
    ];

    vectors::upsert(db, "test_embeddings", embed_docs, None)
        .await
        .expect("vector upsert failed");
    println!("  ✅ vectors::upsert()");

    let query = VectorQuery {
        embedding: vec![0.1f32; 384],
        limit: 10,
        threshold: Some(0.0),
    };

    let results: Vec<VectorSearchResult<TestEmbeddingRecord>> =
        vectors::search(db, "test_embeddings", query, None)
            .await
            .expect("vector search failed");

    assert!(!results.is_empty(), "vector search should return results");
    println!("  ✅ vectors::search() - {} results", results.len());

    println!("  ✅ All vector operations passed");
}