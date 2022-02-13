use std::io::Read;
use serde::de::{DeserializeSeed, Visitor};

pub struct Deserializer<'de, R: Read> {
    pub reader: &'de mut R
}

impl<'de, R: Read> Deserializer<'de, R> {

}

impl<'de, R: Read> serde::de::Deserializer<'de> for &mut Deserializer<'de, R> {
    /// The result of a failed deserialization.
    type Error = crate::Error;

    /// With no info on what the next value is going to be, there's no way to determine it in Terraria world files.
    fn deserialize_any<V>(self, _visitor: V) -> Result<V::Value, Self::Error> where V: Visitor<'de> {
        Err(crate::Error::Unsupported)
    }

    /// `bool`s ("Bool") are stored as a single `u8` containing either `0` or `1`.
    fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value, Self::Error> where V: Visitor<'de> {
        let mut buf: [u8; 1] = [0; 1];
        self.reader.read(&mut buf).map_err(|_err| crate::Error::IO)?;
        match buf[0] {
            0_u8 => visitor.visit_bool(false),
            1_u8 => visitor.visit_bool(true),
            _ => Err(crate::Error::Overflow),
        }
    }

    /// `i8`s do not exist in Terraria save files.
    fn deserialize_i8<V>(self, _visitor: V) -> Result<V::Value, Self::Error> where V: Visitor<'de> {
        Err(crate::Error::Unsupported)
    }

    /// `i16`s ("Int16") are stored in little-endian byte order.
    fn deserialize_i16<V>(self, visitor: V) -> Result<V::Value, Self::Error> where V: Visitor<'de> {
        let mut buf: [u8; 2] = [0; 2];
        self.reader.read(&mut buf).map_err(|_err| crate::Error::IO)?;
        visitor.visit_i16(i16::from_le_bytes(buf))
    }

    /// `i32`s ("Int32") are stored in little-endian byte order.
    fn deserialize_i32<V>(self, visitor: V) -> Result<V::Value, Self::Error> where V: Visitor<'de> {
        let mut buf: [u8; 4] = [0; 4];
        self.reader.read(&mut buf).map_err(|_err| crate::Error::IO)?;
        visitor.visit_i32(i32::from_le_bytes(buf))
    }

    /// `i64`s do not exist in Terraria save files.
    fn deserialize_i64<V>(self, _visitor: V) -> Result<V::Value, Self::Error> where V: Visitor<'de> {
        Err(crate::Error::Unsupported)
    }

    /// `u8`s ("Byte") are stored in little-endian byte order.
    fn deserialize_u8<V>(self, visitor: V) -> Result<V::Value, Self::Error> where V: Visitor<'de> {
        let mut buf: [u8; 1] = [0; 1];
        self.reader.read(&mut buf).map_err(|_err| crate::Error::IO)?;
        visitor.visit_u8(buf[0])
    }

    /// `u16`s don't exist in Terraria save files.
    fn deserialize_u16<V>(self, _visitor: V) -> Result<V::Value, Self::Error> where V: Visitor<'de> {
        Err(crate::Error::Unsupported)
    }

    /// `u32`s don't exist in Terraria save files.
    fn deserialize_u32<V>(self, _visitor: V) -> Result<V::Value, Self::Error> where V: Visitor<'de> {
        Err(crate::Error::Unsupported)
    }

    /// `u64`s don't exist in Terraria save files.
    fn deserialize_u64<V>(self, _visitor: V) -> Result<V::Value, Self::Error> where V: Visitor<'de> {
        Err(crate::Error::Unsupported)
    }

    /// `f32`s ("Single") are stored in little-endian byte order.
    fn deserialize_f32<V>(self, visitor: V) -> Result<V::Value, Self::Error> where V: Visitor<'de> {
        let mut buf: [u8; 4] = [0; 4];
        self.reader.read(&mut buf).map_err(|_err| crate::Error::IO)?;
        visitor.visit_f32(f32::from_le_bytes(buf))
    }

    /// `f64`s ("Double") are stored in little-endian byte order.
    fn deserialize_f64<V>(self, visitor: V) -> Result<V::Value, Self::Error> where V: Visitor<'de> {
        let mut buf: [u8; 8] = [0; 8];
        self.reader.read(&mut buf).map_err(|_err| crate::Error::IO)?;
        visitor.visit_f64(f64::from_le_bytes(buf))
    }

    /// `char`s don't exist in Terraria save files.
    fn deserialize_char<V>(self, _visitor: V) -> Result<V::Value, Self::Error> where V: Visitor<'de> {
        Err(crate::Error::Unsupported)
    }

    /// `str`s ("String") are stored as sequences of bytes.
    fn deserialize_str<V>(self, visitor: V) -> Result<V::Value, Self::Error> where V: Visitor<'de> {
        // This works?
        self.deserialize_seq(visitor)
    }

    /// `str`s ("String") are stored as sequences of bytes.
    fn deserialize_string<V>(self, visitor: V) -> Result<V::Value, Self::Error> where V: Visitor<'de> {
        // This works?
        self.deserialize_seq(visitor)
    }

    /// Bytes should not exist in Terraria save files.
    fn deserialize_bytes<V>(self, _visitor: V) -> Result<V::Value, Self::Error> where V: Visitor<'de> {
        Err(crate::Error::Unsupported)
    }

    /// Bytes should not exist in Terraria save files.
    fn deserialize_byte_buf<V>(self, _visitor: V) -> Result<V::Value, Self::Error> where V: Visitor<'de> {
        Err(crate::Error::Unsupported)
    }

    /// `None`s don't exist in Terraria save files, and consequently neither do `Some`s.
    fn deserialize_option<V>(self, _visitor: V) -> Result<V::Value, Self::Error> where V: Visitor<'de> {
        Err(crate::Error::Unsupported)
    }

    /// Units (`()`) don't exist in Terraria save files.
    fn deserialize_unit<V>(self, _visitor: V) -> Result<V::Value, Self::Error> where V: Visitor<'de> {
        Err(crate::Error::Unsupported)
    }

    /// Named units don't exist in Terraria save files.
    fn deserialize_unit_struct<V>(self, _name: &'static str, _visitor: V) -> Result<V::Value, Self::Error> where V: Visitor<'de> {
        Err(crate::Error::Unsupported)
    }

    /// `struct`s should be handled by serializing their fields in order.
    fn deserialize_newtype_struct<V>(self, _name: &'static str, visitor: V) -> Result<V::Value, Self::Error> where V: Visitor<'de> {
        visitor.visit_newtype_struct(self)
    }

    /// Sequences start with a ULEB128 representation of their length, followed by their contents.
    fn deserialize_seq<V>(self, visitor: V) -> Result<V::Value, Self::Error> where V: Visitor<'de> {
        let size = leb128::read::unsigned(&mut self.reader).map_err(|_err| crate::Error::IO)?;
        let size = usize::try_from(size).map_err(|_err| crate::Error::Overflow)?;
        visitor.visit_seq(ByteSized { de: self, size })
    }

    /// Tuples are stored as simple sequences of values.
    fn deserialize_tuple<V>(self, len: usize, visitor: V) -> Result<V::Value, Self::Error> where V: Visitor<'de> {
        todo!()
    }

    /// Tuple `struct`s are stored exactly in the same way as tuples.
    fn deserialize_tuple_struct<V>(self, _name: &'static str, len: usize, visitor: V) -> Result<V::Value, Self::Error> where V: Visitor<'de> {
        self.deserialize_tuple(len, visitor)
    }

    /// Maps don't exist in Terraria save files.
    fn deserialize_map<V>(self, _visitor: V) -> Result<V::Value, Self::Error> where V: Visitor<'de> {
        Err(crate::Error::Unsupported)
    }

    /// `struct`s are handled like tuples; keys are ignored.
    fn deserialize_struct<V>(self, _name: &'static str, fields: &'static [&'static str], visitor: V) -> Result<V::Value, Self::Error> where V: Visitor<'de> {
        self.deserialize_tuple(fields.len(), visitor)
    }

    fn deserialize_enum<V>(self, name: &'static str, variants: &'static [&'static str], visitor: V) -> Result<V::Value, Self::Error> where V: Visitor<'de> {
        todo!()
    }

    fn deserialize_identifier<V>(self, visitor: V) -> Result<V::Value, Self::Error> where V: Visitor<'de> {
        todo!()
    }

    fn deserialize_ignored_any<V>(self, visitor: V) -> Result<V::Value, Self::Error> where V: Visitor<'de> {
        todo!()
    }
}


struct ByteSized<'a, 'de: 'a, R: Read> {
    de: &'a mut Deserializer<'de, R>,
    size: usize,
}

impl<'a, 'de, R: Read> serde::de::SeqAccess<'de> for ByteSized<'a, 'de, R> {
    type Error = crate::Error;

    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>, Self::Error> where T: DeserializeSeed<'de> {
        match self.size {
            0 => Ok(None),
            _ => seed.deserialize(&mut *self.de).map(Some),
        }
    }

    fn size_hint(&self) -> Option<usize> {
        Some(self.size)
    }
}
