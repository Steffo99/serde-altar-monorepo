//! # Useful notes
//!
//! - https://seancode.com/terrafirma/world.html

mod ser;
mod de;
mod error;
mod types;

pub use ser::Serializer;
pub use de::Deserializer;
pub use error::Error;
pub use error::Result;
pub use types::ULEB128;
pub use types::Rect;
