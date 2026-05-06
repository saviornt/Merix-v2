use dashmap::DashMap;
use std::sync::Arc;

pub type SessionCache = Arc<DashMap<String, merix_core::Session>>;
pub type ContextCache = Arc<DashMap<String, Vec<merix_core::Message>>>;

pub fn init_caches() -> (SessionCache, ContextCache) {
    (Arc::new(DashMap::new()), Arc::new(DashMap::new()))
}
pub fn placeholder() { unimplemented!("TODO: implement module") }