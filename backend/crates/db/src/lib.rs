pub mod connection;
pub mod schemas;
pub mod document;
pub mod vector_search;
pub mod graph;
pub mod full_text_search;
pub mod geospatial;
pub mod time_series;

// Public API — everything other crates will import (kept stable)
pub use connection::{connect, Db};
pub use schemas::*;
pub use document::*;
pub use vector_search::*;
pub use graph::*;
pub use full_text_search::*;
pub use geospatial::*;
pub use time_series::*;

pub type MerixDb = Db;