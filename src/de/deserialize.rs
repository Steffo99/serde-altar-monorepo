use std::marker::PhantomData;
use serde::de::Error;
use crate::VecI16Flags;
use crate::VecULEB128;
use crate::VecI16;
use crate::VecI32;


/// Custom deserialize trait with support for the weird Terraria array serialization.
pub trait Deserialize<'de, T> : serde::de::Deserialize<'de> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::de::Deserializer<'de>, T: crate::de::Deserialize<'de, T>;
}

impl<'de> serde::Deserialize<'de> for VecI16Flags {
    fn deserialize<D>(_deserializer: D) -> Result<Self, D::Error> where D: serde::de::Deserializer<'de> {
        Err(D::Error::custom("Cannot deserialize VecI16Flags with the serde Deserializer"))
    }
}

impl<'de> Deserialize<'de, bool> for VecI16Flags {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::de::Deserializer<'de> {
        deserializer.deserialize_vec_i16flags(crate::de::visitor::VecI16FlagsVisitor)
    }
}

impl<'de, T> serde::Deserialize<'de> for VecULEB128<T> {
    fn deserialize<D>(_deserializer: D) -> Result<Self, D::Error> where D: serde::de::Deserializer<'de> {
        Err(D::Error::custom("Cannot deserialize VecULEB128 with the serde Deserializer"))
    }
}

impl<'de, T> Deserialize<'de, T> for VecULEB128<T> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::de::Deserializer<'de>, T: crate::de::Deserialize<'de, T> {
        deserializer.deserialize_vec_uleb128(crate::de::visitor::VecULEB128Visitor::<T>(PhantomData))
    }
}

impl<'de, T> serde::Deserialize<'de> for VecI16<T> {
    fn deserialize<D>(_deserializer: D) -> Result<Self, D::Error> where D: serde::de::Deserializer<'de> {
        Err(D::Error::custom("Cannot deserialize VecI16 with the serde Deserializer"))
    }
}

impl<'de, T> Deserialize<'de, T> for VecI16<T> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::de::Deserializer<'de>, T: crate::de::Deserialize<'de, T> {
        deserializer.deserialize_vec_i16(crate::de::visitor::VecI16Visitor::<T>(PhantomData))
    }
}

impl<'de, T> serde::Deserialize<'de> for VecI32<T> {
    fn deserialize<D>(_deserializer: D) -> Result<Self, D::Error> where D: serde::de::Deserializer<'de> {
        Err(D::Error::custom("Cannot deserialize VecI32 with the serde Deserializer"))
    }
}

impl<'de, T> Deserialize<'de, T> for VecI32<T> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: crate::de::Deserializer<'de>, T: crate::de::Deserialize<'de, T> {
        deserializer.deserialize_vec_i32(crate::de::visitor::VecI32Visitor::<T>(PhantomData))
    }
}
