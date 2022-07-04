pub mod venue;

// declared like this so that sibling packages can be found via generated code `super::package`
pub mod common { tonic::include_proto!("common"); pub use super::common_proto::*; }
pub mod trading { tonic::include_proto!("trading"); pub use super::trading_proto::*; }
pub mod market { tonic::include_proto!("market"); pub use super::market_proto::*; }

// contains implementations for types imported from proto files
mod common_proto;
mod trading_proto;
mod market_proto;

