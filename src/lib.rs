mod error;
mod vec;
mod ser;
mod de;

pub use ser::WriteSerializer;
pub use ser::to_writer;

pub use de::ReadDeserializer;
pub use de::from_reader;

pub use error::Error;
pub use error::Result;

pub use vec::VecI16Flags;
pub use vec::VecULEB128;
pub use vec::VecI16;
pub use vec::VecI32;
