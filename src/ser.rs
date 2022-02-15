use std::io::Write;
use serde::Serialize;

/// `Write`-based serializer for Terraria world files.
pub struct Serializer<W: Write> {
    pub writer: W,
}

impl<W: Write> Serializer<W> {
    /// Write a ULEB128 value.
    pub fn write_uleb128<T: Into<u64>>(&mut self, val: T) -> crate::Result<()> {
        leb128::write::unsigned(&mut self.writer, val.into()).map_err(|_err| crate::Error::IO)?;
        Ok(())
    }
}

impl<W: Write> serde::ser::Serializer for &mut Serializer<W> {
    /// The result of a successful serialization.
    /// Since we write in a buffer, we don't have any output.
    type Ok = ();

    /// The result of a failed serialization.
    type Error = crate::Error;

    /// The type used to handle serialization of sequences' contents.
    type SerializeSeq = Self;

    /// The type used to handle serialization of tuples' contents.
    type SerializeTuple = Self;

    /// The type used to handle serialization of tuple `struct`s' contents.
    type SerializeTupleStruct = Self;

    /// The type used to handle serialization of tuple variants' contents.
    type SerializeTupleVariant = Self;

    /// The type used to handle serialization of maps' contents.
    type SerializeMap = Self;

    /// The type used to handle serialization of structs' contents.
    type SerializeStruct = Self;

    /// The type used to handle serialization of struct variants' contents.
    type SerializeStructVariant = Self;

    /// `bool`s ("Bool") are stored as a single `u8` containing either `0` or `1`.
    fn serialize_bool(self, v: bool) -> Result<Self::Ok, Self::Error> {
        self.serialize_u8(match v {
            false => 0_u8,
            true => 1_u8,
        })
    }

    /// `i8`s do not exist in Terraria save files.
    fn serialize_i8(self, v: i8) -> Result<Self::Ok, Self::Error> {
        self.writer.write_all(&v.to_le_bytes()).map_err(|_err| crate::Error::IO)
    }

    /// `i16`s ("Int16") are stored in little-endian byte order.
    fn serialize_i16(self, v: i16) -> Result<Self::Ok, Self::Error> {
        self.writer.write_all(&v.to_le_bytes()).map_err(|_err| crate::Error::IO)
    }

    /// `i32`s ("Int32") are stored in little-endian byte order.
    fn serialize_i32(self, v: i32) -> Result<Self::Ok, Self::Error> {
        self.writer.write_all(&v.to_le_bytes()).map_err(|_err| crate::Error::IO)
    }

    /// `i64`s are stored in little-endian byte order.
    fn serialize_i64(self, v: i64) -> Result<Self::Ok, Self::Error> {
        self.writer.write_all(&v.to_le_bytes()).map_err(|_err| crate::Error::IO)
    }

    /// `u8`s ("Byte") are stored in little-endian byte order.
    fn serialize_u8(self, v: u8) -> Result<Self::Ok, Self::Error> {
        self.writer.write_all(&v.to_le_bytes()).map_err(|_err| crate::Error::IO)
    }

    /// `u16`s are stored in little-endian byte order.
    fn serialize_u16(self, v: u16) -> Result<Self::Ok, Self::Error> {
        self.writer.write_all(&v.to_le_bytes()).map_err(|_err| crate::Error::IO)
    }

    /// `u32`s are stored in little-endian byte order.
    fn serialize_u32(self, v: u32) -> Result<Self::Ok, Self::Error> {
        self.writer.write_all(&v.to_le_bytes()).map_err(|_err| crate::Error::IO)
    }

    /// `u64`s are stored in little-endian byte order.
    fn serialize_u64(self, v: u64) -> Result<Self::Ok, Self::Error> {
        self.writer.write_all(&v.to_le_bytes()).map_err(|_err| crate::Error::IO)
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
    fn serialize_char(self, _v: char) -> Result<Self::Ok, Self::Error> {
        Err(crate::Error::Unsupported)
    }

    /// `str`s ("String") are stored as sequences of bytes.
    fn serialize_str(self, v: &str) -> Result<Self::Ok, Self::Error> {
        v.as_bytes().to_vec().serialize(self)
    }

    /// Bytes are stored literally.
    fn serialize_bytes(self, v: &[u8]) -> Result<Self::Ok, Self::Error> {
        self.writer.write_all(v).map_err(|_err| crate::Error::IO)
    }

    /// `None`s don't exist in Terraria save files.
    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        Err(crate::Error::Unsupported)
    }

    /// `None`s don't exist in Terraria save files, and consequently neither do `Some`s.
    fn serialize_some<T: ?Sized>(self, _value: &T) -> Result<Self::Ok, Self::Error> where T: serde::ser::Serialize {
        Err(crate::Error::Unsupported)
    }

    /// Units (`()`) don't exist in Terraria save files.
    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        Err(crate::Error::Unsupported)
    }

    /// Named units don't exist in Terraria save files.
    fn serialize_unit_struct(self, _name: &'static str) -> Result<Self::Ok, Self::Error> {
        Err(crate::Error::Unsupported)
    }

    /// Variant units don't exist in Terraria save files.
    fn serialize_unit_variant(self, _name: &'static str, _variant_index: u32, _variant: &'static str) -> Result<Self::Ok, Self::Error> {
        Err(crate::Error::Unsupported)
    }

    /// `struct`s should be handled by serializing their fields in order.
    fn serialize_newtype_struct<T: ?Sized>(self, _name: &'static str, value: &T) -> Result<Self::Ok, Self::Error> where T: serde::ser::Serialize {
        value.serialize(self)
    }

    /// Generic `struct`s should be handled by serializing their fields in order.
    fn serialize_newtype_variant<T: ?Sized>(self, _name: &'static str, _variant_index: u32, _variant: &'static str, value: &T) -> Result<Self::Ok, Self::Error> where T: serde::ser::Serialize {
        value.serialize(self)
    }

    /// Sequences start with a ULEB128 representation of their length, followed by their contents.
    fn serialize_seq(self, len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        match len {
            Some(v) => {
                self.write_uleb128(v as u64)?;
                Ok(self)
            },
            // If the length of a sequence is not defined, it cannot be represented in a Terraria save file.
            None => Err(crate::Error::Unsupported)?,
        }
    }

    /// Tuples are stored as simple sequences of values.
    fn serialize_tuple(self, _len: usize) -> Result<Self::SerializeTuple, Self::Error> {
        // Nothing is prepended to the tuple on serialization!
        Ok(self)
    }

    /// Tuple `struct`s are stored exactly in the same way as tuples.
    fn serialize_tuple_struct(self, _name: &'static str, len: usize) -> Result<Self::SerializeTupleStruct, Self::Error> {
        self.serialize_tuple(len)
    }

    /// Tuple variants don't exist in Terraria save files.
    fn serialize_tuple_variant(self, _name: &'static str, _variant_index: u32, _variant: &'static str, _len: usize) -> Result<Self::SerializeTupleVariant, Self::Error> {
        Err(crate::Error::Unsupported)
    }

    /// Maps don't exist in Terraria save files.
    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        Err(crate::Error::Unsupported)
    }

    /// `struct`s are handled like tuples; keys are ignored.
    fn serialize_struct(self, _name: &'static str, len: usize) -> Result<Self::SerializeStruct, Self::Error> {
        self.serialize_tuple(len)
    }

    /// `struct` variants don't exist in Terraria save files.
    fn serialize_struct_variant(self, _name: &'static str, _variant_index: u32, _variant: &'static str, _len: usize) -> Result<Self::SerializeStructVariant, Self::Error> {
        Err(crate::Error::Unsupported)
    }
}

impl<W: Write> serde::ser::SerializeSeq for &mut Serializer<W> {
    /// The result of a successful serialization.
    /// Since we write in a buffer, we don't have any output.
    type Ok = ();

    /// The result of a failed serialization.
    type Error = crate::Error;

    /// Sequence elements are stored like regular values.
    fn serialize_element<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error> where T: Serialize {
        // I'm not sure why this is a double pointer?
        value.serialize(&mut **self)
    }

    /// Sequences don't have an end marker in Terraria save files.
    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }
}

impl<W: Write> serde::ser::SerializeTuple for &mut Serializer<W> {
    /// The result of a successful serialization.
    /// Since we write in a buffer, we don't have any output.
    type Ok = ();

    /// The result of a failed serialization.
    type Error = crate::Error;

    /// Tuple elements are stored like regular values.
    fn serialize_element<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error> where T: Serialize {
        // I'm not sure why this is a double pointer?
        value.serialize(&mut **self)
    }

    /// Tuples don't have an end marker in Terraria save files.
    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }
}

impl<W: Write> serde::ser::SerializeTupleStruct for &mut Serializer<W> {
    /// The result of a successful serialization.
    /// Since we write in a buffer, we don't have any output.
    type Ok = ();

    /// The result of a failed serialization.
    type Error = crate::Error;

    /// Tuple `struct`s are stored exactly in the same way as tuples.
    fn serialize_field<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error> where T: Serialize {
        serde::ser::SerializeTuple::serialize_element(self, value)
    }

    /// Tuple `struct`s are stored exactly in the same way as tuples.
    fn end(self) -> Result<Self::Ok, Self::Error> {
        serde::ser::SerializeTuple::end(self)
    }
}

impl<W: Write> serde::ser::SerializeTupleVariant for &mut Serializer<W> {
    /// The result of a successful serialization.
    /// Since we write in a buffer, we don't have any output.
    type Ok = ();

    /// The result of a failed serialization.
    type Error = crate::Error;

    /// Tuple variants don't exist in Terraria save files.
    fn serialize_field<T: ?Sized>(&mut self, _value: &T) -> Result<(), Self::Error> where T: Serialize {
        Err(crate::Error::Unsupported)
    }

    /// Tuple variants don't exist in Terraria save files.
    fn end(self) -> Result<Self::Ok, Self::Error> {
        Err(crate::Error::Unsupported)
    }
}

impl<W: Write> serde::ser::SerializeMap for &mut Serializer<W> {
    /// The result of a successful serialization.
    /// Since we write in a buffer, we don't have any output.
    type Ok = ();

    /// The result of a failed serialization.
    type Error = crate::Error;

    /// Maps don't exist in Terraria save files.
    fn serialize_key<T: ?Sized>(&mut self, _key: &T) -> Result<(), Self::Error> where T: Serialize {
        Err(crate::Error::Unsupported)
    }

    /// Maps don't exist in Terraria save files.
    fn serialize_value<T: ?Sized>(&mut self, _value: &T) -> Result<(), Self::Error> where T: Serialize {
        Err(crate::Error::Unsupported)
    }

    /// Maps don't exist in Terraria save files.
    fn end(self) -> Result<Self::Ok, Self::Error> {
        Err(crate::Error::Unsupported)
    }
}

impl<W: Write> serde::ser::SerializeStruct for &mut Serializer<W> {
    /// The result of a successful serialization.
    /// Since we write in a buffer, we don't have any output.
    type Ok = ();

    /// The result of a failed serialization.
    type Error = crate::Error;

    /// `struct`s are handled like tuples; keys are ignored.
    fn serialize_field<T: ?Sized>(&mut self, _key: &'static str, value: &T) -> Result<(), Self::Error> where T: Serialize {
        serde::ser::SerializeTuple::serialize_element(self, value)
    }

    /// `struct`s are handled like tuples; keys are ignored.
    fn end(self) -> Result<Self::Ok, Self::Error> {
        serde::ser::SerializeTuple::end(self)
    }
}

impl<W: Write> serde::ser::SerializeStructVariant for &mut Serializer<W> {
    /// The result of a successful serialization.
    /// Since we write in a buffer, we don't have any output.
    type Ok = ();

    /// The result of a failed serialization.
    type Error = crate::Error;

    /// `struct` variants don't exist in Terraria save files.
    fn serialize_field<T: ?Sized>(&mut self, _key: &'static str, _value: &T) -> Result<(), Self::Error> where T: Serialize {
        Err(crate::Error::Unsupported)
    }

    /// `struct` variants don't exist in Terraria save files.
    fn end(self) -> Result<Self::Ok, Self::Error> {
        Err(crate::Error::Unsupported)
    }
}
