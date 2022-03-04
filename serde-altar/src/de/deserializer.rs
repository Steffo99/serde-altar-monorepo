use crate::de::Visitor;

/// Custom deserializer trait with support for the weird Terraria array serialization.
pub trait Deserializer<'de> : serde::de::Deserializer<'de> {

    /// Hint that the `Deserialize` type is expecting a sequence of bits, prefixed with the bit amount as an [i16].
    fn deserialize_vec_i16flags<V>(self, visitor: V) -> Result<V::Value, Self::Error> where V: crate::de::Visitor<'de>;

    /// Hint that the `Deserialize` type is expecting a sequence of values, prefixed with the sequence size as an [i16].
    fn deserialize_vec_i16<V>(self, visitor: V) -> Result<V::Value, Self::Error> where V: crate::de::Visitor<'de>;

    /// Hint that the `Deserialize` type is expecting a sequence of values, prefixed with the sequence size as an [i32].
    fn deserialize_vec_i32<V>(self, visitor: V) -> Result<V::Value, Self::Error> where V: crate::de::Visitor<'de>;

    /// Hint that the `Deserialize` type is expecting a sequence of values, prefixed with the sequence size as an ULEB128.
    fn deserialize_vec_uleb128<V>(self, visitor: V) -> Result<V::Value, Self::Error> where V: crate::de::Visitor<'de>;
}


/// `Read`-based deserializer for Terraria world files.
pub struct ReadDeserializer<'de, R> where R: std::io::Read {
    pub(crate) reader: &'de mut R
}

impl<'de, R> ReadDeserializer<'de, R> where R: std::io::Read {
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

/// Implementation of the base serde data model.
impl<'de, R> serde::de::Deserializer<'de> for &mut ReadDeserializer<'de, R> where R: std::io::Read {
    /// The result of a failed deserialization.
    type Error = crate::Error;

    fn deserialize_any<V>(self, _visitor: V) -> Result<V::Value, Self::Error> where V: serde::de::Visitor<'de> {
        // With no info on what the next value is going to be, there's no way to determine it in Terraria world files.
        Err(crate::Error::Unsupported)
    }

    fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value, Self::Error> where V: serde::de::Visitor<'de> {
        // `bool`s ("Bool") are stored as a single `u8` containing either `0` or `1`.
        let buf = self.read_bytes::<1>()?;
        match buf[0] {
            0_u8 => visitor.visit_bool(false),
            1_u8 => visitor.visit_bool(true),
            _ => Err(crate::Error::Overflow),
        }
    }

    fn deserialize_i8<V>(self, visitor: V) -> Result<V::Value, Self::Error> where V: serde::de::Visitor<'de> {
        // `i8`s are stored in little-endian byte order.
        let buf = self.read_bytes::<1>()?;
        visitor.visit_i8(i8::from_le_bytes(buf))
    }

    fn deserialize_i16<V>(self, visitor: V) -> Result<V::Value, Self::Error> where V: serde::de::Visitor<'de> {
        // `i16`s ("Int16") are stored in little-endian byte order.
        let buf = self.read_bytes::<2>()?;
        visitor.visit_i16(i16::from_le_bytes(buf))
    }

    fn deserialize_i32<V>(self, visitor: V) -> Result<V::Value, Self::Error> where V: serde::de::Visitor<'de> {
        // `i32`s ("Int32") are stored in little-endian byte order.
        let buf = self.read_bytes::<4>()?;
        visitor.visit_i32(i32::from_le_bytes(buf))
    }

    fn deserialize_i64<V>(self, visitor: V) -> Result<V::Value, Self::Error> where V: serde::de::Visitor<'de> {
        // `i64`s are stored in little-endian byte order.
        let buf = self.read_bytes::<8>()?;
        visitor.visit_i64(i64::from_le_bytes(buf))
    }

    fn deserialize_u8<V>(self, visitor: V) -> Result<V::Value, Self::Error> where V: serde::de::Visitor<'de> {
        // `u8`s ("Byte") are stored in little-endian byte order.
        let buf = self.read_bytes::<1>()?;
        visitor.visit_u8(u8::from_le_bytes(buf))
    }

    fn deserialize_u16<V>(self, visitor: V) -> Result<V::Value, Self::Error> where V: serde::de::Visitor<'de> {
        // `u16`s are stored in little-endian byte order.
        let buf = self.read_bytes::<2>()?;
        visitor.visit_u16(u16::from_le_bytes(buf))
    }

    fn deserialize_u32<V>(self, visitor: V) -> Result<V::Value, Self::Error> where V: serde::de::Visitor<'de> {
        // `u32`s are stored in little-endian byte order.
        let buf = self.read_bytes::<4>()?;
        visitor.visit_u32(u32::from_le_bytes(buf))
    }

    fn deserialize_u64<V>(self, visitor: V) -> Result<V::Value, Self::Error> where V: serde::de::Visitor<'de> {
        // `u64`s are stored in little-endian byte order.
        let buf = self.read_bytes::<8>()?;
        visitor.visit_u64(u64::from_le_bytes(buf))
    }

    fn deserialize_f32<V>(self, visitor: V) -> Result<V::Value, Self::Error> where V: serde::de::Visitor<'de> {
        // `f32`s ("Single") are stored in little-endian byte order.
        let mut buf: [u8; 4] = [0; 4];
        self.reader.read(&mut buf).map_err(|_err| crate::Error::IO)?;
        visitor.visit_f32(f32::from_le_bytes(buf))
    }

    fn deserialize_f64<V>(self, visitor: V) -> Result<V::Value, Self::Error> where V: serde::de::Visitor<'de> {
        // `f64`s ("Double") are stored in little-endian byte order.
        let mut buf: [u8; 8] = [0; 8];
        self.reader.read(&mut buf).map_err(|_err| crate::Error::IO)?;
        visitor.visit_f64(f64::from_le_bytes(buf))
    }

    fn deserialize_char<V>(self, _visitor: V) -> Result<V::Value, Self::Error> where V: serde::de::Visitor<'de> {
        // `char`s don't exist in Terraria save files.
        Err(crate::Error::Unsupported)
    }

    fn deserialize_str<V>(self, visitor: V) -> Result<V::Value, Self::Error> where V: serde::de::Visitor<'de> {
        // Ownership of the string must be taken in Terraria world files.
        self.deserialize_string(visitor)
    }

    fn deserialize_string<V>(self, visitor: V) -> Result<V::Value, Self::Error> where V: serde::de::Visitor<'de> {
        // `str`s ("String") are stored as sequences of bytes.
        let bytes = self.read_uleb128_vec()?;
        let str = String::from_utf8(bytes).map_err(|_err| crate::Error::Overflow)?;
        visitor.visit_string(str)
    }

    fn deserialize_bytes<V>(self, _visitor: V) -> Result<V::Value, Self::Error> where V: serde::de::Visitor<'de> {
        // Terraria has no support for terminated byte-strings.
        Err(crate::Error::Unsupported)
    }

    fn deserialize_byte_buf<V>(self, _visitor: V) -> Result<V::Value, Self::Error> where V: serde::de::Visitor<'de> {
        // Terraria has no support for terminated byte-strings.
        Err(crate::Error::Unsupported)
    }

    fn deserialize_option<V>(self, _visitor: V) -> Result<V::Value, Self::Error> where V: serde::de::Visitor<'de> {
        // `None`s don't exist in Terraria save files.
        Err(crate::Error::Unsupported)
    }

    fn deserialize_unit<V>(self, _visitor: V) -> Result<V::Value, Self::Error> where V: serde::de::Visitor<'de> {
        // Units `()` don't exist in Terraria save files.
        Err(crate::Error::Unsupported)
    }

    fn deserialize_unit_struct<V>(self, _name: &'static str, _visitor: V) -> Result<V::Value, Self::Error> where V: serde::de::Visitor<'de> {
        // Named units can't be serialized in Terraria save files.
        Err(crate::Error::Unsupported)
    }

    fn deserialize_newtype_struct<V>(self, _name: &'static str, visitor: V) -> Result<V::Value, Self::Error> where V: serde::de::Visitor<'de> {
        // `struct`s are handled by serializing their fields in order.
        visitor.visit_newtype_struct(self)
    }

    fn deserialize_seq<V>(self, _visitor: V) -> Result<V::Value, Self::Error> where V: serde::de::Visitor<'de> {
        // Generic sequences should not be used in `serde-altar`; sized Vecs are available, though.
        Err(crate::Error::Unsupported)
    }

    fn deserialize_tuple<V>(self, len: usize, visitor: V) -> Result<V::Value, Self::Error> where V: serde::de::Visitor<'de> {
        // Tuples are stored as simple sequences of values.
        visitor.visit_seq(crate::de::accessor::ValueSized { size: len, de: self })
    }

    fn deserialize_tuple_struct<V>(self, _name: &'static str, len: usize, visitor: V) -> Result<V::Value, Self::Error> where V: serde::de::Visitor<'de> {
        // Tuple `struct`s are stored exactly in the same way as tuples.
        self.deserialize_tuple(len, visitor)
    }

    fn deserialize_map<V>(self, _visitor: V) -> Result<V::Value, Self::Error> where V: serde::de::Visitor<'de> {
        // Maps don't exist in Terraria save files.
        Err(crate::Error::Unsupported)
    }

    fn deserialize_struct<V>(self, _name: &'static str, fields: &'static [&'static str], visitor: V) -> Result<V::Value, Self::Error> where V: serde::de::Visitor<'de> {
        // `struct`s are handled like tuples; keys are ignored.
        self.deserialize_tuple(fields.len(), visitor)
    }

    fn deserialize_enum<V>(self, _name: &'static str, _variants: &'static [&'static str], _visitor: V) -> Result<V::Value, Self::Error> where V: serde::de::Visitor<'de> {
        // `enum`s don't exist in Terraria save files.
        Err(crate::Error::Unsupported)
    }

    fn deserialize_identifier<V>(self, _visitor: V) -> Result<V::Value, Self::Error> where V: serde::de::Visitor<'de> {
        // Identifiers don't exist in Terraria save files.
        Err(crate::Error::Unsupported)
    }

    fn deserialize_ignored_any<V>(self, _visitor: V) -> Result<V::Value, Self::Error> where V: serde::de::Visitor<'de> {
        // With no info on what the next value is going to be, there's no way to determine it in Terraria world files.
        Err(crate::Error::Unsupported)
    }

    fn is_human_readable(&self) -> bool {
        // Terraria world files are not human-readable.
        false
    }
}

impl<'de, R> crate::de::Deserializer<'de> for &mut ReadDeserializer<'de, R> where R: std::io::Read {
    fn deserialize_vec_i16flags<V>(self, visitor: V) -> Result<V::Value, Self::Error> where V: Visitor<'de> {
        let len = i16::from_le_bytes(self.read_bytes::<2>()?) as usize;
        visitor.visit_seq(crate::de::accessor::ValueSized { size: len / 8, de: self })
    }

    fn deserialize_vec_i16<V>(self, visitor: V) -> Result<V::Value, Self::Error> where V: Visitor<'de> {
        let len = i16::from_le_bytes(self.read_bytes::<2>()?) as usize;
        visitor.visit_seq(crate::de::accessor::ValueSized { size: len, de: self })
    }

    fn deserialize_vec_i32<V>(self, visitor: V) -> Result<V::Value, Self::Error> where V: Visitor<'de> {
        let len = i32::from_le_bytes(self.read_bytes::<4>()?) as usize;
        visitor.visit_seq(crate::de::accessor::ValueSized { size: len, de: self })
    }

    fn deserialize_vec_uleb128<V>(self, visitor: V) -> Result<V::Value, Self::Error> where V: Visitor<'de> {
        let len = self.read_uleb128()?;
        visitor.visit_seq(crate::de::accessor::ValueSized { size: len, de: self })
    }
}
