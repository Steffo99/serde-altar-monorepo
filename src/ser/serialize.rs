use serde::ser::SerializeSeq;
use crate::VecI16Flags;
use crate::VecULEB128;
use crate::VecI16;
use crate::VecI32;

pub trait Serialize : serde::ser::Serialize {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::ser::Serializer;
}

impl serde::ser::Serialize for VecI16Flags {
    fn serialize<S>(&self, _serializer: S) -> Result<S::Ok, S::Error> where S: serde::ser::Serializer {
        Err(serde::ser::Error::custom("Cannot serialize VecI16Flags with the serde Serializer"))
    }
}

impl Serialize for VecI16Flags {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::ser::Serializer {
        let bit_len = i16::try_from(self.0.len()).map_err(|_err| serde::ser::Error::custom("Vec length does not fit in a i16"))?;
        let should_add_one = (bit_len % 8) != 0;
        let len = (bit_len / 8) + if should_add_one { 1 } else { 0 };
        let mut seq = serializer.serialize_vec_i16flags(len)?;
        for element in &self.0 {
            seq.serialize_element(&element)?;
        };
        seq.end()
    }
}

impl<T> serde::ser::Serialize for VecULEB128<T> {
    fn serialize<S>(&self, _serializer: S) -> Result<S::Ok, S::Error> where S: serde::ser::Serializer {
        Err(serde::ser::Error::custom("Cannot serialize VecULEB128 with the serde Serializer"))
    }
}

impl<T> Serialize for VecULEB128<T> where T: serde::ser::Serialize {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::ser::Serializer {
        let len = self.0.len();
        let mut seq = serializer.serialize_vec_uleb128(len)?;
        for element in &self.0 {
            seq.serialize_element(&element)?;
        };
        seq.end()
    }
}

impl<T> serde::ser::Serialize for VecI16<T> {
    fn serialize<S>(&self, _serializer: S) -> Result<S::Ok, S::Error> where S: serde::ser::Serializer {
        Err(serde::ser::Error::custom("Cannot serialize VecI16 with the serde Serializer"))
    }
}

impl<T> Serialize for VecI16<T> where T: serde::ser::Serialize {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::ser::Serializer {
        let len = i16::try_from(self.0.len()).map_err(|_err| serde::ser::Error::custom("Vec length does not fit in a i16"))?;
        let mut seq = serializer.serialize_vec_i16(len)?;
        for element in &self.0 {
            seq.serialize_element(&element)?;
        };
        seq.end()
    }
}

impl<T> serde::ser::Serialize for VecI32<T> {
    fn serialize<S>(&self, _serializer: S) -> Result<S::Ok, S::Error> where S: serde::ser::Serializer {
        Err(serde::ser::Error::custom("Cannot serialize VecI32 with the serde Serializer"))
    }
}

impl<T> Serialize for VecI32<T> where T: serde::ser::Serialize {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: crate::ser::Serializer {
        let len = i32::try_from(self.0.len()).map_err(|_err| serde::ser::Error::custom("Vec length does not fit in a i32"))?;
        let mut seq = serializer.serialize_vec_i32(len)?;
        for element in &self.0 {
            seq.serialize_element(&element)?;
        };
        seq.end()
    }
}

