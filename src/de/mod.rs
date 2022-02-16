mod deserialize;
mod deserializer;
mod visitor;
mod accessor;

pub use deserialize::Deserialize;
pub use deserializer::Deserializer;
pub use visitor::Visitor;

pub use deserializer::ReadDeserializer;


/// Deserialize any [Deserialize]able struct using a [Read]er as a source.
pub fn from_reader<'de, R, T>(reader: &'de mut R) -> crate::Result<T> where T: Deserialize<'de, T>, R: std::io::Read {
    let mut de = ReadDeserializer { reader };
    let t = Deserialize::deserialize(&mut de)?;
    Ok(t)
}
