use async_trait::async_trait;

// venue super trait
#[async_trait]
pub trait Venue: proto_types::trading::ProtoService {}
