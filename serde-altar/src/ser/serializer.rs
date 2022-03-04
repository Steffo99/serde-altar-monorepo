/// Custom serializer trait with support for the weird Terraria array serialization.
pub trait Serializer : serde::ser::Serializer {
    fn serialize_vec_i16flags(self, len: i16) -> Result<Self::SerializeSeq, Self::Error>;
    fn serialize_vec_uleb128(self, len: usize) -> Result<Self::SerializeSeq, Self::Error>;
    fn serialize_vec_i16(self, len: i16) -> Result<Self::SerializeSeq, Self::Error>;
    fn serialize_vec_i32(self, len: i32) -> Result<Self::SerializeSeq, Self::Error>;
}


/// `Write`-based serializer for Terraria world files.
pub struct WriteSerializer<W> where W: std::io::Write {
    pub(crate) writer: W,
}

impl<W> WriteSerializer<W> where W: std::io::Write {
    /// Write a ULEB128 value.
    pub fn write_uleb128<T: Into<u64>>(&mut self, val: T) -> crate::Result<()> {
        leb128::write::unsigned(&mut self.writer, val.into()).map_err(|_err| crate::Error::IO)?;
        Ok(())
    }
}

impl<W> serde::ser::Serializer for &mut WriteSerializer<W> where W: std::io::Write {
    // The result of a successful serialization.
    // Since we write in a buffer, we don't have any output.
    type Ok = ();

    // The result of a failed serialization.
    type Error = crate::Error;

    // The type used to handle serialization of sequences' contents.
    type SerializeSeq = Self;

    // The type used to handle serialization of tuples' contents.
    type SerializeTuple = Self;

    // The type used to handle serialization of tuple `struct`s' contents.
    type SerializeTupleStruct = Self;

    // The type used to handle serialization of tuple variants' contents.
    type SerializeTupleVariant = Self;

    // The type used to handle serialization of maps' contents.
    type SerializeMap = Self;

    // The type used to handle serialization of structs' contents.
    type SerializeStruct = Self;

    // The type used to handle serialization of struct variants' contents.
    type SerializeStructVariant = Self;

    fn serialize_bool(self, v: bool) -> Result<Self::Ok, Self::Error> {
        // `bool`s ("Bool") are stored as a single `u8` containing either `0` or `1`.
        self.serialize_u8(match v {
            false => 0_u8,
            true => 1_u8,
        })
    }

    fn serialize_i8(self, v: i8) -> Result<Self::Ok, Self::Error> {
        // `i8`s are stored in little-endian byte order.
        self.writer.write_all(&v.to_le_bytes()).map_err(|_err| crate::Error::IO)
    }

    fn serialize_i16(self, v: i16) -> Result<Self::Ok, Self::Error> {
        // `i16`s ("Int16") are stored in little-endian byte order.
        self.writer.write_all(&v.to_le_bytes()).map_err(|_err| crate::Error::IO)
    }

    fn serialize_i32(self, v: i32) -> Result<Self::Ok, Self::Error> {
        // `i32`s ("Int32") are stored in little-endian byte order.
        self.writer.write_all(&v.to_le_bytes()).map_err(|_err| crate::Error::IO)
    }

    fn serialize_i64(self, v: i64) -> Result<Self::Ok, Self::Error> {
        // `i64`s are stored in little-endian byte order.
        self.writer.write_all(&v.to_le_bytes()).map_err(|_err| crate::Error::IO)
    }

    fn serialize_u8(self, v: u8) -> Result<Self::Ok, Self::Error> {
        // `u8`s ("Byte") are stored in little-endian byte order.
        self.writer.write_all(&v.to_le_bytes()).map_err(|_err| crate::Error::IO)
    }

    fn serialize_u16(self, v: u16) -> Result<Self::Ok, Self::Error> {
        // `u16`s are stored in little-endian byte order.
        self.writer.write_all(&v.to_le_bytes()).map_err(|_err| crate::Error::IO)
    }

    fn serialize_u32(self, v: u32) -> Result<Self::Ok, Self::Error> {
        // `u32`s are stored in little-endian byte order.
        self.writer.write_all(&v.to_le_bytes()).map_err(|_err| crate::Error::IO)
    }

    fn serialize_u64(self, v: u64) -> Result<Self::Ok, Self::Error> {
        // `u64`s are stored in little-endian byte order.
        self.writer.write_all(&v.to_le_bytes()).map_err(|_err| crate::Error::IO)
    }

    fn serialize_f32(self, v: f32) -> Result<Self::Ok, Self::Error> {
        // `f32`s ("Single") are stored in little-endian byte order.
        self.writer.write_all(&v.to_le_bytes()).map_err(|_err| crate::Error::IO)
    }

    fn serialize_f64(self, v: f64) -> Result<Self::Ok, Self::Error> {
        // `f64`s ("Double") are stored in little-endian byte order.
        self.writer.write_all(&v.to_le_bytes()).map_err(|_err| crate::Error::IO)
    }

    fn serialize_char(self, _v: char) -> Result<Self::Ok, Self::Error> {
        // `char`s don't exist in Terraria save files.
        Err(crate::Error::Unsupported)
    }

    fn serialize_str(self, v: &str) -> Result<Self::Ok, Self::Error> {
        // `str`s ("String") are stored as sequences of bytes.
        let size = v.len() as u64;
        self.write_uleb128(size)?;
        self.writer.write(v.as_bytes()).map_err(|_err| crate::Error::IO)?;
        Ok(())
    }

    fn serialize_bytes(self, _v: &[u8]) -> Result<Self::Ok, Self::Error> {
        // Terraria has no support for terminated byte-strings.
        Err(crate::Error::Unsupported)
    }

    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        // `None`s don't exist in Terraria save files.
        Err(crate::Error::Unsupported)
    }

    fn serialize_some<T: ?Sized>(self, _value: &T) -> Result<Self::Ok, Self::Error> where T: serde::ser::Serialize {
        // `Some`s don't exist in Terraria save files.
        Err(crate::Error::Unsupported)
    }

    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        // Units `()` don't exist in Terraria save files.
        Err(crate::Error::Unsupported)
    }

    fn serialize_unit_struct(self, _name: &'static str) -> Result<Self::Ok, Self::Error> {
        // Named units can't be serialized in Terraria save files.
        Err(crate::Error::Unsupported)
    }

    fn serialize_unit_variant(self, _name: &'static str, _variant_index: u32, _variant: &'static str) -> Result<Self::Ok, Self::Error> {
        // Unit variants don't exist in Terraria save files.
        Err(crate::Error::Unsupported)
    }

    fn serialize_newtype_struct<T: ?Sized>(self, _name: &'static str, value: &T) -> Result<Self::Ok, Self::Error> where T: serde::ser::Serialize {
        // `struct`s are handled by serializing their fields in order.
        value.serialize(self)
    }

    fn serialize_newtype_variant<T: ?Sized>(self, _name: &'static str, _variant_index: u32, _variant: &'static str, value: &T) -> Result<Self::Ok, Self::Error> where T: serde::ser::Serialize {
        // Generic `struct`s are handled by serializing their fields in order.
        value.serialize(self)
    }

    fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        // Generic sequences should not be used in `serde-altar`; sized Vecs are available, though.
        /*
        match len {
            Some(len) => {
                let len = u32::try_from(len).map_err(|_err| crate::Error::Overflow)?;
                self.writer.write(&len.to_le_bytes()).map_err(|_err| crate::Error::IO)?;
                Ok(self)
            },
            // If the length of a sequence is not defined, it cannot be represented in a Terraria save file.
            None => Err(crate::Error::Unsupported)?,
        }
        */
        Err(crate::Error::Unsupported)
    }

    fn serialize_tuple(self, _len: usize) -> Result<Self::SerializeTuple, Self::Error> {
        // Tuples are stored as simple sequences of values.
        Ok(self)
    }

    fn serialize_tuple_struct(self, _name: &'static str, len: usize) -> Result<Self::SerializeTupleStruct, Self::Error> {
        // Tuple `struct`s are stored exactly in the same way as tuples.
        self.serialize_tuple(len)
    }

    fn serialize_tuple_variant(self, _name: &'static str, _variant_index: u32, _variant: &'static str, _len: usize) -> Result<Self::SerializeTupleVariant, Self::Error> {
        // Tuple variants don't exist in Terraria save files.
        Err(crate::Error::Unsupported)
    }

    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        // Maps don't exist in Terraria save files.
        Err(crate::Error::Unsupported)
    }

    fn serialize_struct(self, _name: &'static str, len: usize) -> Result<Self::SerializeStruct, Self::Error> {
        // `struct`s are handled like tuples; keys are ignored.
        self.serialize_tuple(len)
    }

    fn serialize_struct_variant(self, _name: &'static str, _variant_index: u32, _variant: &'static str, _len: usize) -> Result<Self::SerializeStructVariant, Self::Error> {
        // `struct` variants don't exist in Terraria save files.
        Err(crate::Error::Unsupported)
    }
}

impl<W> Serializer for &mut WriteSerializer<W> where W: std::io::Write {
    fn serialize_vec_i16flags(self, len: i16) -> Result<Self::SerializeSeq, Self::Error> {
        self.writer.write(&len.to_le_bytes()).map_err(|_err| crate::Error::IO)?;
        Ok(self)
    }

    fn serialize_vec_uleb128(self, len: usize) -> Result<Self::SerializeSeq, Self::Error> {
        self.writer.write(&len.to_le_bytes()).map_err(|_err| crate::Error::IO)?;
        Ok(self)
    }

    fn serialize_vec_i16(self, len: i16) -> Result<Self::SerializeSeq, Self::Error> {
        self.writer.write(&len.to_le_bytes()).map_err(|_err| crate::Error::IO)?;
        Ok(self)
    }

    fn serialize_vec_i32(self, len: i32) -> Result<Self::SerializeSeq, Self::Error> {
        self.writer.write(&len.to_le_bytes()).map_err(|_err| crate::Error::IO)?;
        Ok(self)
    }
}

impl<W> serde::ser::SerializeSeq for &mut WriteSerializer<W> where W: std::io::Write {
    // The result of a successful serialization.
    // Since we write in a buffer, we don't have any output.
    type Ok = ();

    // The result of a failed serialization.
    type Error = crate::Error;

    fn serialize_element<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error> where T: serde::ser::Serialize {
        // Sequence elements are stored like regular values.
        // I'm not sure why this is a double pointer?
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        // Sequences don't have an end marker in Terraria save files.
        Ok(())
    }
}

impl<W> serde::ser::SerializeTuple for &mut WriteSerializer<W> where W: std::io::Write {
    // The result of a successful serialization.
    // Since we write in a buffer, we don't have any output.
    type Ok = ();

    // The result of a failed serialization.
    type Error = crate::Error;

    // Tuple elements are stored like regular values.
    fn serialize_element<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error> where T: serde::ser::Serialize {
        // I'm not sure why this is a double pointer?
        value.serialize(&mut **self)
    }

    // Tuples don't have an end marker in Terraria save files.
    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }
}

impl<W> serde::ser::SerializeTupleStruct for &mut WriteSerializer<W> where W: std::io::Write {
    // The result of a successful serialization.
    // Since we write in a buffer, we don't have any output.
    type Ok = ();

    // The result of a failed serialization.
    type Error = crate::Error;

    // Tuple `struct`s are stored exactly in the same way as tuples.
    fn serialize_field<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error> where T: serde::ser::Serialize {
        serde::ser::SerializeTuple::serialize_element(self, value)
    }

    // Tuple `struct`s are stored exactly in the same way as tuples.
    fn end(self) -> Result<Self::Ok, Self::Error> {
        serde::ser::SerializeTuple::end(self)
    }
}

impl<W> serde::ser::SerializeTupleVariant for &mut WriteSerializer<W> where W: std::io::Write {
    // The result of a successful serialization.
    // Since we write in a buffer, we don't have any output.
    type Ok = ();

    // The result of a failed serialization.
    type Error = crate::Error;

    fn serialize_field<T: ?Sized>(&mut self, _value: &T) -> Result<(), Self::Error> where T: serde::ser::Serialize {
        // Tuple variants don't exist in Terraria save files.
        Err(crate::Error::Unsupported)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        // Tuple variants don't exist in Terraria save files.
        Err(crate::Error::Unsupported)
    }
}

impl<W> serde::ser::SerializeMap for &mut WriteSerializer<W> where W: std::io::Write {
    // The result of a successful serialization.
    // Since we write in a buffer, we don't have any output.
    type Ok = ();

    // The result of a failed serialization.
    type Error = crate::Error;

    fn serialize_key<T: ?Sized>(&mut self, _key: &T) -> Result<(), Self::Error> where T: serde::ser::Serialize {
        // Maps don't exist in Terraria save files.
        Err(crate::Error::Unsupported)
    }

    fn serialize_value<T: ?Sized>(&mut self, _value: &T) -> Result<(), Self::Error> where T: serde::ser::Serialize {
        // Maps don't exist in Terraria save files.
        Err(crate::Error::Unsupported)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        // Maps don't exist in Terraria save files.
        Err(crate::Error::Unsupported)
    }
}

impl<W> serde::ser::SerializeStruct for &mut WriteSerializer<W> where W: std::io::Write {
    // The result of a successful serialization.
    // Since we write in a buffer, we don't have any output.
    type Ok = ();

    // The result of a failed serialization.
    type Error = crate::Error;

    // `struct`s are handled like tuples; keys are ignored.
    fn serialize_field<T: ?Sized>(&mut self, _key: &'static str, value: &T) -> Result<(), Self::Error> where T: serde::ser::Serialize {
        serde::ser::SerializeTuple::serialize_element(self, value)
    }

    // `struct`s are handled like tuples; keys are ignored.
    fn end(self) -> Result<Self::Ok, Self::Error> {
        serde::ser::SerializeTuple::end(self)
    }
}

impl<W> serde::ser::SerializeStructVariant for &mut WriteSerializer<W> where W: std::io::Write {
    // The result of a successful serialization.
    // Since we write in a buffer, we don't have any output.
    type Ok = ();

    // The result of a failed serialization.
    type Error = crate::Error;

    fn serialize_field<T: ?Sized>(&mut self, _key: &'static str, _value: &T) -> Result<(), Self::Error> where T: serde::ser::Serialize {
        // `struct` variants don't exist in Terraria save files.
        Err(crate::Error::Unsupported)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        // `struct` variants don't exist in Terraria save files.
        Err(crate::Error::Unsupported)
    }
}
