mod serialize;
mod serializer;

pub use serialize::Serialize;
pub use serializer::Serializer;
pub use serializer::WriteSerializer;


/// Serialize any [Serialize]able struct using a [Write]r as a destination.
pub fn to_writer<W, T>(writer: W, value: T) -> crate::Result<W> where W: std::io::Write, T: Serialize {
    let mut ser = WriteSerializer { writer };
    value.serialize(&mut ser)?;
    Ok(ser.writer)
}
