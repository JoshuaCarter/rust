// clippy lint rules
#![deny(clippy::implicit_return)]
#![allow(clippy::needless_return)]
#![allow(clippy::module_inception)]

extern crate simple_error;

pub mod cli;
pub mod model;
