use std::borrow::BorrowMut;
use std::io::Write;
use serde::Serialize;
use crate::ULEB128;

/// The main serializer struct, into which objects can be "inserted".
pub struct Serializer<W: Write> {
    pub writer: W,
}

impl<W: Write> Serializer<W> {

    /// Certain integers are stored as `ULEB128`, so they can have infinite length.
    fn serialize_uleb128(&mut self, v: crate::types::ULEB128) -> Result<(), crate::Error> {
        self.writer.write_all(&v.as_slice()).map_err(|_err| crate::Error::IO)
    }

}

impl<W: Write> serde::ser::Serializer for &mut Serializer<W> {
    /// The result of a successful serialization.
    /// Since we write in a buffer, we don't have any output.
    type Ok = ();

    /// The result of a failed serialization.
    type Error = crate::Error;

    type SerializeSeq = Self;
    type SerializeTuple = Self;
    type SerializeTupleStruct = Self;
    type SerializeTupleVariant = Self;
    type SerializeMap = Self;
    type SerializeStruct = Self;
    type SerializeStructVariant = Self;

    /// `bool`s ("Bool") are stored as a single `u8` containing either `0` or `1`.
    fn serialize_bool(self, v: bool) -> Result<Self::Ok, Self::Error> {
        self.serialize_u8(match v {
            false => 0_u8,
            true => 1_u8,
        })
    }

    /// `i8`s do not exist in Terraria save files.
    fn serialize_i8(self, _v: i8) -> Result<Self::Ok, Self::Error> {
        Err(crate::Error::Unsupported)
    }

    /// `i16`s ("Int16") are stored in little-endian byte order.
    fn serialize_i16(self, v: i16) -> Result<Self::Ok, Self::Error> {
        self.writer.write_all(&v.to_le_bytes()).map_err(|_err| crate::Error::IO)
    }

    /// `i32`s ("Int32") are stored in little-endian byte order.
    fn serialize_i32(self, v: i32) -> Result<Self::Ok, Self::Error> {
        self.writer.write_all(&v.to_le_bytes()).map_err(|_err| crate::Error::IO)
    }

    /// `i64`s do not exist in Terraria save files.
    fn serialize_i64(self, _v: i64) -> Result<Self::Ok, Self::Error> {
        Err(crate::Error::Unsupported)
    }

    /// `u8`s ("Byte") are stored in little-endian byte order.
    fn serialize_u8(self, v: u8) -> Result<Self::Ok, Self::Error> {
        self.writer.write_all(&[v]).map_err(|_err| crate::Error::IO)
    }

    /// `u16`s don't exist in Terraria save files.
    fn serialize_u16(self, _v: u16) -> Result<Self::Ok, Self::Error> {
        Err(crate::Error::Unsupported)
    }

    /// `u32`s don't exist in Terraria save files.
    fn serialize_u32(self, _v: u32) -> Result<Self::Ok, Self::Error> {
        Err(crate::Error::Unsupported)
    }

    /// `u64`s don't exist in Terraria save files.
    fn serialize_u64(self, _v: u64) -> Result<Self::Ok, Self::Error> {
        Err(crate::Error::Unsupported)
    }

    /// `f32`s ("Single") are stored in little-endian byte order.
    fn serialize_f32(self, v: f32) -> Result<Self::Ok, Self::Error> {
        self.writer.write_all(&v.to_le_bytes()).map_err(|_err| crate::Error::IO)
    }

    /// `f64`s ("Double") are stored in little-endian byte order.
    fn serialize_f64(self, v: f64) -> Result<Self::Ok, Self::Error> {
        self.writer.write_all(&v.to_le_bytes()).map_err(|_err| crate::Error::IO)
    }

    /// `char`s don't exist in Terraria save files.
    fn serialize_char(self, v: char) -> Result<Self::Ok, Self::Error> {
        Err(crate::Error::Unsupported)
    }

    /// `str`s ("String") are stored in `UTF8`, with the byte length prepended as a `ULEB128`
    /// number.
    fn serialize_str(self, v: &str) -> Result<Self::Ok, Self::Error> {
        ULEB128::from(v.len()).serialize(self)?;
        self.writer.write_all(v.as_bytes()).map_err(|_err| crate::Error::IO)?;
        Ok(())
    }

    /// Bytes are interpreted literally.
    fn serialize_bytes(self, v: &[u8]) -> Result<Self::Ok, Self::Error> {
        self.writer.write_all(v).map_err(|_err| crate::Error::IO)
    }

    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_some<T: ?Sized>(self, value: &T) -> Result<Self::Ok, Self::Error> where T: serde::ser::Serialize {
        todo!()
    }

    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_unit_struct(self, name: &'static str) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_unit_variant(self, name: &'static str, variant_index: u32, variant: &'static str) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_newtype_struct<T: ?Sized>(self, name: &'static str, value: &T) -> Result<Self::Ok, Self::Error> where T: serde::ser::Serialize {
        todo!()
    }

    fn serialize_newtype_variant<T: ?Sized>(self, name: &'static str, variant_index: u32, variant: &'static str, value: &T) -> Result<Self::Ok, Self::Error> where T: serde::ser::Serialize {
        todo!()
    }

    fn serialize_seq(self, len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        todo!()
    }

    fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple, Self::Error> {
        todo!()
    }

    fn serialize_tuple_struct(self, name: &'static str, len: usize) -> Result<Self::SerializeTupleStruct, Self::Error> {
        todo!()
    }

    fn serialize_tuple_variant(self, name: &'static str, variant_index: u32, variant: &'static str, len: usize) -> Result<Self::SerializeTupleVariant, Self::Error> {
        todo!()
    }

    fn serialize_map(self, len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        todo!()
    }

    fn serialize_struct(self, name: &'static str, len: usize) -> Result<Self::SerializeStruct, Self::Error> {
        todo!()
    }

    fn serialize_struct_variant(self, name: &'static str, variant_index: u32, variant: &'static str, len: usize) -> Result<Self::SerializeStructVariant, Self::Error> {
        todo!()
    }
}