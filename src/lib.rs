//! # Useful notes
//!
//! - https://seancode.com/terrafirma/world.html

mod ser;
mod de;
mod error;

pub use ser::Serializer;
pub use ser::to_writer;
pub use de::Deserializer;
pub use de::from_reader;
pub use error::Error;
pub use error::Result;