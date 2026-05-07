use crate::Db;
use crate::connection::engine;
use merix_core::MerixError;
use tracing;

/// Named database connections for different workloads in Merix.
pub struct MerixDbPool {
    standard:  Db,
    temporal:  Db,
    ephemeral: Db,
}

impl MerixDbPool {
    pub async fn init() -> Result<Self, MerixError> {
        let standard = engine::open_standard().await?;
        let temporal = engine::open_temporal().await?;
        let ephemeral = engine::open_ephemeral().await?;

        tracing::info!("MerixDbPool initialized with 3 named connections (standard / temporal / ephemeral)");

        Ok(Self {
            standard,
            temporal,
            ephemeral,
        })
    }

    pub fn standard(&self)  -> &Db { &self.standard }
    pub fn temporal(&self)  -> &Db { &self.temporal }
    pub fn ephemeral(&self) -> &Db { &self.ephemeral }
}