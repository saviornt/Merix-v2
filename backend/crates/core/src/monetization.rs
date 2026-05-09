 //! monetization.rs
 //! Ad-supported revenue layer for Merix-v2.
 //! Provides user-interest profiling via semantic search on memory entries (chats/profile),
 //! personalized ad selection, impression tracking, and revenue calculation.
 //! 
 //! This module integrates with merix-memory embeddings for privacy-first,
 //! local semantic matching. No external ad networks are wired yet.

use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::Mutex;
use uuid::Uuid;
use chrono::{DateTime, Utc};
// TODO: Import from existing memory crate once confirmed in lib.rs
// use merix_memory::MemoryEntry;  // or wherever MemoryEntry lives

/// Revenue model enum (AdSupported is the focus).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RevenueModel {
    AdSupported,
    Subscription,
    OneTime,
}

/// Single ad impression record.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdImpression {
    pub id: Uuid,
    pub slot: String,
    pub provider: String,
    pub timestamp: DateTime<Utc>,
    pub clicked: bool,
    pub estimated_revenue_cents: u64,
}

/// Ad config (CPM/CPC + interval throttling).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdConfig {
    pub cpm_cents: u64,
    pub cpc_cents: u64,
    pub min_ad_interval_seconds: u64,
}

impl Default for AdConfig {
    fn default() -> Self {
        // TODO: Tune defaults based on real ad network benchmarks
        Self { cpm_cents: 500, cpc_cents: 50, min_ad_interval_seconds: 300 }
    }
}

/// TODO: Implement AdRevenueTracker (in-memory + future SurrealDB persistence).
/// Tracks impressions, enforces interval, calculates revenue.
pub struct AdRevenueTracker {
    // TODO: Add fields + methods (record_impression, total_revenue_cents, etc.)
    _inner: Arc<Mutex<()>>, // placeholder
}

/// TODO: Implement UserInterestProfiler.
/// Uses semantic search on recent MemoryEntry embeddings (chats + profile)
/// to compute a single user interest vector via averaging.
pub struct UserInterestProfiler;

impl UserInterestProfiler {
    // TODO: pub fn compute_interest_vector(entries: &[MemoryEntry]) -> Vec<f32>
    //       - Average embeddings from merix-memory
    //       - Handle empty case gracefully
}

/// TODO: Implement AdSelector + cosine_similarity helper.
/// Matches user interest vector against pre-defined ad categories
/// (expand categories with real inventory later).
pub struct AdSelector {
    // TODO: categories: Vec<AdCategory>
}

impl AdSelector {
    // TODO: pub fn new() -> Self
    // TODO: pub fn select_ad(&self, interest_vector: &[f32]) -> Option<PersonalizedAd>
}

/// TODO: Add PersonalizedAd struct for frontend delivery.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersonalizedAd {
    pub id: Uuid,
    pub slot: String,
    pub headline: String,
    pub body: String,
    pub cta_url: String,
    pub category_id: String,
}
