// clippy lint rules
#![deny(clippy::implicit_return)]
#![allow(clippy::needless_return)]

pub mod math2d;
pub use math2d::*;
pub mod render;
pub use render::*;
