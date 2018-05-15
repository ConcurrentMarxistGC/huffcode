#[macro_use]
extern crate serde_derive;

pub use error::HuffcodeError;
pub use model::HuffmanTree;

mod model;
mod error;

