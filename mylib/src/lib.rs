// clippy lint rules
#![allow(clippy::needless_return)]
#![allow(clippy::module_inception)]
#![allow(clippy::new_without_default)]

extern crate simple_error;

pub mod cli;
pub mod model;
pub mod venues;
pub mod utils;
pub mod net;
