use std::fmt::Formatter;
use serde::de::SeqAccess;
use crate::VecI16Flags;
use crate::VecULEB128;
use crate::VecI16;
use crate::VecI32;

/// Visitor for [VecI16Flags], containing `bool`s.
pub struct VecI16FlagsVisitor;
/// Visitor for [VecULEB128], containing `T`s.
pub struct VecULEB128Visitor<T> (pub std::marker::PhantomData<T>);
/// Visitor for [VecI16], containing `T`s.
pub struct VecI16Visitor<T> (pub std::marker::PhantomData<T>);
/// Visitor for [VecULEB128], containing `T`s.
pub struct VecI32Visitor<T> (pub std::marker::PhantomData<T>);


/// Custom visitor trait with support for the weird Terraria array serialization.
pub trait Visitor<'de> : serde::de::Visitor<'de> {
    /// The input contains a [VecI16Flags].
    ///
    /// The default implementation fails with a type error.
    fn visit_vec_i16flags<S: serde::de::SeqAccess<'de>>(self, seq: S) -> Result<Self::Value, S::Error> {
        let _ = seq;
        Err(serde::de::Error::invalid_type(serde::de::Unexpected::Seq, &self))
    }

    /// The input contains a [VecULEB128].
    ///
    /// The default implementation fails with a type error.
    fn visit_vec_uleb128<S: serde::de::SeqAccess<'de>>(self, seq: S) -> Result<Self::Value, S::Error> {
        let _ = seq;
        Err(serde::de::Error::invalid_type(serde::de::Unexpected::Seq, &self))
    }

    /// The input contains a [VecI16].
    ///
    /// The default implementation fails with a type error.
    fn visit_vec_i16<S: serde::de::SeqAccess<'de>>(self, seq: S) -> Result<Self::Value, S::Error> {
        let _ = seq;
        Err(serde::de::Error::invalid_type(serde::de::Unexpected::Seq, &self))
    }

    /// The input contains a [VecI32].
    ///
    /// The default implementation fails with a type error.
    fn visit_vec_i32<S: serde::de::SeqAccess<'de>>(self, seq: S) -> Result<Self::Value, S::Error> {
        let _ = seq;
        Err(serde::de::Error::invalid_type(serde::de::Unexpected::Seq, &self))
    }
}

impl<'de> serde::de::Visitor<'de> for VecI16FlagsVisitor {
    type Value = VecI16Flags;

    fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
        formatter.write_str("a u16-sized list of bools")
    }
}

impl<'de> Visitor<'de> for VecI16FlagsVisitor {
    fn visit_vec_i16flags<S: SeqAccess<'de>>(self, mut seq: S) -> Result<Self::Value, S::Error> {
        let mut inner_vec: Vec<bool> = vec![];
        while let Some(element) = seq.next_element::<u8>()? {
            let mut bits = vec![
                (element & 0b0000_0001) != 0,
                (element & 0b0000_0010) != 0,
                (element & 0b0000_0100) != 0,
                (element & 0b0000_1000) != 0,
                (element & 0b0001_0000) != 0,
                (element & 0b0010_0000) != 0,
                (element & 0b0100_0000) != 0,
                (element & 0b1000_0000) != 0,
            ];
            inner_vec.append(&mut bits);
        }
        Ok(VecI16Flags(inner_vec))
    }
}

impl<'de, T> serde::de::Visitor<'de> for VecI16Visitor<T> {
    type Value = VecI16<T>;

    fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
        formatter.write_str("a u16-sized list")
    }
}

impl<'de, T> Visitor<'de> for VecI16Visitor<T> where T: crate::de::Deserialize<'de, T> {
    fn visit_vec_i16<S: SeqAccess<'de>>(self, mut seq: S) -> Result<Self::Value, S::Error> {
        let mut inner_vec: Vec<T> = vec![];
        while let Some(element) = seq.next_element()? {
            inner_vec.push(element);
        }
        Ok(VecI16(inner_vec))
    }
}

impl<'de, T> serde::de::Visitor<'de> for VecI32Visitor<T> {
    type Value = VecI32<T>;

    fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
        formatter.write_str("a u32-sized list")
    }
}

impl<'de, T> Visitor<'de> for VecI32Visitor<T> where T: crate::de::Deserialize<'de, T> {
    fn visit_vec_i32<S: SeqAccess<'de>>(self, mut seq: S) -> Result<Self::Value, S::Error> {
        let mut inner_vec: Vec<T> = vec![];
        while let Some(element) = seq.next_element()? {
            inner_vec.push(element);
        }
        Ok(VecI32(inner_vec))
    }
}

impl<'de, T> serde::de::Visitor<'de> for VecULEB128Visitor<T> {
    type Value = VecULEB128<T>;

    fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
        formatter.write_str("a uleb128-sized list")
    }
}

impl<'de, T> Visitor<'de> for VecULEB128Visitor<T> where T: crate::de::Deserialize<'de, T> {
    fn visit_vec_uleb128<S: SeqAccess<'de>>(self, mut seq: S) -> Result<Self::Value, S::Error> {
        let mut inner_vec: Vec<T> = vec![];
        while let Some(element) = seq.next_element()? {
            inner_vec.push(element);
        }
        Ok(VecULEB128(inner_vec))
    }
}
