use merix_core::MerixError;
use merix_memory::{stm::ShortTermMemory, ltm::LongTermMemory};
use merix_agent_tools::skills::Skill;
use merix_orchestration::swarm::Swarm;
use tokio::sync::Mutex;
use std::sync::Arc;

/// Main Agent runtime
pub struct Agent {
    pub id: String,
    pub stm: ShortTermMemory,
    pub ltm: LongTermMemory,
    pub skills: Vec<Skill>,
    pub swarm: Option<Arc<Mutex<Swarm>>>,
}

impl Agent {
    pub fn new(id: String) -> Self {
        Self {
            id,
            stm: ShortTermMemory::new(),
            ltm: LongTermMemory::new(),
            skills: Vec::new(),
            swarm: None,
        }
    }

    pub async fn run(&self) {
        // TODO: Main agent loop - will use tools, memory, and swarm coordination
        unimplemented!("Agent runtime logic coming in next phase")
    }
}
