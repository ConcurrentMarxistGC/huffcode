#[macro_use]
extern crate serde_derive;
extern crate serde;

pub use error::HuffcodeError;
pub use model::HuffmanTree;

mod model;
mod error;

