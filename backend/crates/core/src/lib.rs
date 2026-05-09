pub mod types;
pub mod error;
pub mod config;

// Clean public API re-exports (used everywhere in the workspace)
pub use types::*;
pub use error::MerixError;
pub use config::Config;

// Convenience type alias for the entire project
pub type MerixResult<T> = Result<T, MerixError>;

/// Initialize the core library (called once at app startup)
pub fn init() -> MerixResult<()> {
    // Future: load config, initialize tracing, etc.
    tracing::info!("Merix core initialized");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_core_serialization_integration() {
        // Test that all major types can be serialized together
        let session = Session {
            id: uuid::Uuid::new_v4(),
            model: "llama3.2:3b".to_string(),
            created_at: chrono::Utc::now(),
        };

        let config = Config::default();
        let error = MerixError::Inference("test error".to_string());

        // All types should serialize without error
        let _ = serde_json::to_string(&session).unwrap();
        let _ = serde_json::to_string(&config).unwrap();
        let _ = serde_json::to_string(&error).unwrap();

        println!("✅ All core types serialize correctly");
    }
}


pub mod monetization;

pub use monetization::{AdConfig, AdImpression, AdRevenueTracker, AdSelector, PersonalizedAd, RevenueModel, UserInterestProfiler};

