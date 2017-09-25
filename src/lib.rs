// DOCS

#[macro_use]
extern crate error_chain;

pub mod component;
pub mod error;
mod parser;
pub mod property;

pub use component::Component;
pub use component::parse_component;
pub use component::write_component;
pub use property::Property;
pub use property::escape_chars;
pub use property::unescape_chars;

