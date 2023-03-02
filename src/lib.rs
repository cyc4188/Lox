pub mod loxer;
pub mod scanner;
pub mod token;
pub mod error;
pub mod utils;

pub use loxer::Loxer;
pub use token::{Literal, Token, TokenType};
pub use error::Error;
pub use utils::*;

pub use log::info;
#[macro_use]
extern crate enum_display_derive;
