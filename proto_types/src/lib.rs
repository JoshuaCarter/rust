#![allow(clippy::needless_return)]
#![allow(clippy::module_inception)]
#![allow(clippy::new_without_default)]

pub mod common { tonic::include_proto!("common"); }

mod trading_proto { tonic::include_proto!("trading"); }
pub mod trading {
    pub use super::trading_proto::trading_server::Trading as ProtoService;
    pub use super::trading_proto::trading_server::TradingServer as ProtoServer;
    pub use super::trading_proto::trading_client::TradingClient as ProtoClient;
    pub use super::trading_proto::{
        NewReply,
        NewRequest,
        CxlReply,
        CxlRequest
    };
}
