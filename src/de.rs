use std::io::Read;
use serde::de::{DeserializeSeed, Visitor};

/// `Read`-based deserializer for Terraria world files.
pub struct Deserializer<'de, R: Read> {
    pub reader: &'de mut R
}

impl<'de, R: Read> Deserializer<'de, R> {
    /// Read a ULEB128 value.
    pub fn read_uleb128(&mut self) -> crate::Result<usize> {
        let size = leb128::read::unsigned(&mut self.reader).map_err(|_err| crate::Error::IO)?;
        let size = usize::try_from(size).map_err(|_err| crate::Error::Overflow)?;
        Ok(size)
    }

    /// Read `N` bytes from the `reader`.
    pub fn read_bytes<const N: usize>(&mut self) -> crate::Result<[u8; N]> {
        let mut buf = [0; N];
        self.reader.read(&mut buf).map_err(|_err| crate::Error::IO)?;
        Ok(buf)
    }

    /// Read a ULEB128-sized `Vec` from the `reader`.
    pub fn read_uleb128_vec(&mut self) -> crate::Result<Vec<u8>> {
        let size = self.read_uleb128()?;
        let mut buf = vec![0; size];
        self.reader.read(&mut buf).map_err(|_err| crate::Error::IO)?;
        Ok(buf)
    }
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
        let buf = self.read_bytes::<1>()?;
        match buf[0] {
            0_u8 => visitor.visit_bool(false),
            1_u8 => visitor.visit_bool(true),
            _ => Err(crate::Error::Overflow),
        }
    }

    /// `i8`s are stored in little-endian byte order.
    fn deserialize_i8<V>(self, visitor: V) -> Result<V::Value, Self::Error> where V: Visitor<'de> {
        let buf = self.read_bytes::<1>()?;
        visitor.visit_i8(i8::from_le_bytes(buf))
    }

    /// `i16`s ("Int16") are stored in little-endian byte order.
    fn deserialize_i16<V>(self, visitor: V) -> Result<V::Value, Self::Error> where V: Visitor<'de> {
        let buf = self.read_bytes::<2>()?;
        visitor.visit_i16(i16::from_le_bytes(buf))
    }

    /// `i32`s ("Int32") are stored in little-endian byte order.
    fn deserialize_i32<V>(self, visitor: V) -> Result<V::Value, Self::Error> where V: Visitor<'de> {
        let buf = self.read_bytes::<4>()?;
        visitor.visit_i32(i32::from_le_bytes(buf))
    }

    /// `i64`s are stored in little-endian byte order.
    fn deserialize_i64<V>(self, visitor: V) -> Result<V::Value, Self::Error> where V: Visitor<'de> {
        let buf = self.read_bytes::<8>()?;
        visitor.visit_i64(i64::from_le_bytes(buf))
    }

    /// `u8`s ("Byte") are stored in little-endian byte order.
    fn deserialize_u8<V>(self, visitor: V) -> Result<V::Value, Self::Error> where V: Visitor<'de> {
        let buf = self.read_bytes::<1>()?;
        visitor.visit_u8(u8::from_le_bytes(buf))
    }

    /// `u16`s are stored in little-endian byte order.
    fn deserialize_u16<V>(self, visitor: V) -> Result<V::Value, Self::Error> where V: Visitor<'de> {
        let buf = self.read_bytes::<2>()?;
        visitor.visit_u16(u16::from_le_bytes(buf))
    }

    /// `u32`s are stored in little-endian byte order.
    fn deserialize_u32<V>(self, visitor: V) -> Result<V::Value, Self::Error> where V: Visitor<'de> {
        let buf = self.read_bytes::<4>()?;
        visitor.visit_u32(u32::from_le_bytes(buf))
    }

    /// `u64`s are stored in little-endian byte order.
    fn deserialize_u64<V>(self, visitor: V) -> Result<V::Value, Self::Error> where V: Visitor<'de> {
        let buf = self.read_bytes::<8>()?;
        visitor.visit_u64(u64::from_le_bytes(buf))
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
        self.deserialize_string(visitor)
    }

    /// `str`s ("String") are stored as sequences of bytes.
    fn deserialize_string<V>(self, visitor: V) -> Result<V::Value, Self::Error> where V: Visitor<'de> {
        let bytes = self.read_uleb128_vec()?;
        let str = String::from_utf8(bytes).map_err(|_err| crate::Error::Overflow)?;
        visitor.visit_string(str)
    }

    /// Bytes don't exist in Terraria save files.
    fn deserialize_bytes<V>(self, _visitor: V) -> Result<V::Value, Self::Error> where V: Visitor<'de> {
        Err(crate::Error::Unsupported)
    }

    /// Bytes don't exist in Terraria save files.
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
        visitor.visit_seq(ByteSized { size: self.read_uleb128()?, de: self })
    }

    /// Tuples are stored as simple sequences of values.
    fn deserialize_tuple<V>(self, len: usize, visitor: V) -> Result<V::Value, Self::Error> where V: Visitor<'de> {
        visitor.visit_seq(ByteSized { de: self, size: len })
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

    /// `enum`s don't exist in Terraria save files.
    fn deserialize_enum<V>(self, _name: &'static str, _variants: &'static [&'static str], _visitor: V) -> Result<V::Value, Self::Error> where V: Visitor<'de> {
        Err(crate::Error::Unsupported)
    }

    /// Identifiers don't exist in Terraria save files.
    fn deserialize_identifier<V>(self, _visitor: V) -> Result<V::Value, Self::Error> where V: Visitor<'de> {
        Err(crate::Error::Unsupported)
    }

    /// With no info on what the next value is going to be, there's no way to determine it in Terraria world files.
    fn deserialize_ignored_any<V>(self, _visitor: V) -> Result<V::Value, Self::Error> where V: Visitor<'de> {
        Err(crate::Error::Unsupported)
    }

    /// Terraria world files are not human-readable.
    fn is_human_readable(&self) -> bool {
        false
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
