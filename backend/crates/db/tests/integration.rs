use merix_db::*;
use serde_json::json;
use uuid::Uuid;
use chrono::Utc;

#[tokio::test]
async fn test_full_integration_normal_and_vector_operations() {
    let db = merix_db::init().await.expect("Failed to initialize test DB");

    println!("🚀 Starting minimal integration test...");

    // =============================================================
    // 1. Normal CRUD operations
    // =============================================================
    let profile = json!({
        "id": Uuid::new_v4().to_string(),
        "username": "testuser",
        "email": "test@example.com",
        "preferences": {"theme": "dark", "lang": "en"},
        "created_at": Utc::now(),
        "updated_at": Utc::now()
    });

    insert(&db, "user_profiles", &profile).await.expect("insert failed");
    println!("✅ insert() passed");

    let all = find_all(&db, "user_profiles").await.unwrap();
    assert!(!all.is_empty());

    let id_str = format!("user_profiles:{}", profile["id"].as_str().unwrap());
    let found = find_by_id(&db, id_str.clone()).await.unwrap();
    assert!(found.is_some());

    update(&db, id_str.clone(), json!({"username": "updateduser"})).await.unwrap();
    delete(&db, id_str).await.unwrap();

    println!("✅ Normal CRUD passed");

    // =============================================================
    // 2. Vector operations
    // =============================================================
    let memory = json!({
        "id": format!("memory:{}", Uuid::new_v4()),
        "content": "Test memory for vector search",
        "embedding": vec![0.1; 1536],
        "timestamp": Utc::now(),
        "tags": ["test"]
    });

    upsert(&db, "memory", json!(memory)).await.expect("upsert failed");
    println!("✅ upsert() passed");

    let query_vector = vec![0.1; 1536];
    let results = search(&db, "memory", query_vector, 5).await.unwrap();
    assert!(!results.is_empty(), "vector search should return results");

    println!("✅ Vector operations passed");

    println!("🎉 Full merix-db integration test passed successfully!");
}
