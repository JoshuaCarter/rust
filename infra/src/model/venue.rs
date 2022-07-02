use super::trading::TradingVenue;

// venue super trait
#[tonic::async_trait]
pub trait Venue: TradingVenue {}
