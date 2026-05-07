// backend/crates/cache/tests/integration.rs
//! Integration tests for the merix-cache crate.
//!
//! These tests treat the crate as an external dependency (exactly how the rest
//! of the application will use it). They focus on:
//! - Using multiple generic `Cache` instances together (e.g. sessions + contexts)
//! - Concurrent access patterns
//! - TTL expiration across caches
//!
//! The tests are written against the **public** API only.

use merix_cache::Cache;
use merix_core::{Message, Session};
use std::sync::Arc;
use std::thread;
use std::time::Duration;
use uuid::Uuid;
use chrono::Utc;

#[test]
fn integration_both_caches_work_together() {
    // Two separate generic caches with different TTLs
    let session_cache: Cache<String, Session> = Cache::new(Some(Duration::from_millis(300)));
    let context_cache: Cache<String, Vec<Message>> = Cache::new(Some(Duration::from_millis(300)));

    // Session usage
    let session = Session {
        id: Uuid::new_v4(),
        model: "gpt-4o-mini".to_string(),
        created_at: Utc::now(),
    };
    session_cache.insert("user-123".to_string(), session.clone());
    assert!(session_cache.contains_key(&"user-123".to_string()));
    assert_eq!(session_cache.get(&"user-123".to_string()).unwrap().id, session.id);

    // Context usage (Vec<Message>)
    let msg1 = Message {
        role: "user".to_string(),
        content: "Hello".to_string(),
    };
    let msg2 = Message {
        role: "assistant".to_string(),
        content: "Hi there!".to_string(),
    };
    let mut history = vec![msg1];
    history.push(msg2);
    context_cache.insert("conv-abc".to_string(), history.clone());

    let retrieved = context_cache.get(&"conv-abc".to_string()).unwrap();
    assert_eq!(retrieved.len(), 2);
    assert_eq!(retrieved[0].content, "Hello");

    // Both caches alive at the same time
    assert_eq!(session_cache.len(), 1);
    assert_eq!(context_cache.len(), 1);
}

#[test]
fn integration_concurrent_access() {
    let session_cache: Cache<String, Session> = Cache::new(None); // no TTL
    let context_cache: Cache<String, Vec<Message>> = Cache::new(None);

    let session_cache = Arc::new(session_cache);
    let context_cache = Arc::new(context_cache);

    let mut handles = vec![];

    // 8 threads hammering both caches simultaneously
    for i in 0..8 {
        let sc = Arc::clone(&session_cache);
        let cc = Arc::clone(&context_cache);
        let handle = thread::spawn(move || {
            let session = Session {
                id: Uuid::new_v4(),
                model: format!("model-{}", i),
                created_at: Utc::now(),
            };
            let key = format!("sess-{}", i);
            sc.insert(key.clone(), session);

            let msg1 = Message {
                role: "user".to_string(),
                content: format!("msg from thread {}", i),
            };
            let msg2 = Message {
                role: "assistant".to_string(),
                content: "reply".to_string(),
            };
            let history = vec![msg1, msg2];
            cc.insert(format!("conv-{}", i), history);

            assert!(sc.contains_key(&key));
            assert!(cc.get(&format!("conv-{}", i)).is_some());
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    assert_eq!(session_cache.len(), 8);
    assert_eq!(context_cache.len(), 8);
}

#[test]
fn integration_ttl_expiration_across_caches() {
    let session_cache: Cache<String, Session> = Cache::new(Some(Duration::from_millis(100)));
    let context_cache: Cache<String, Vec<Message>> = Cache::new(Some(Duration::from_millis(100)));

    // Insert data
    let session = Session {
        id: Uuid::new_v4(),
        model: "test".to_string(),
        created_at: Utc::now(),
    };
    session_cache.insert("exp-session".to_string(), session);

    let msg = Message {
        role: "system".to_string(),
        content: "temp".to_string(),
    };
    context_cache.insert("exp-conv".to_string(), vec![msg]);

    // Wait for expiration
    thread::sleep(Duration::from_millis(150));

    assert!(session_cache.get(&"exp-session".to_string()).is_none());
    assert!(context_cache.get(&"exp-conv".to_string()).is_none());
}