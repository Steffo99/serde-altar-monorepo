use serde::ser::Serialize;
use serde::{Serializer};

pub struct ULEB128(Vec<u8>);

impl From<&[u8]> for ULEB128 {
    fn from(v: &[u8]) -> Self {
        ULEB128(v.to_vec())
    }
}

impl From<u64> for ULEB128 {
    fn from(value: u64) -> Self {
        let mut vec = vec![];
        leb128::write::unsigned(&mut vec, value).expect("Error while writing ULEB128 data to a Vec<u8>.");
        ULEB128(vec)
    }
}

impl From<usize> for ULEB128 {
    fn from(v: usize) -> Self {
        // FIXME: This might have a performance cost on 32-bit architectures.
        ULEB128::from(v as u64)
    }
}

impl Serialize for ULEB128 {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        serializer.serialize_bytes(self.as_slice())
    }
}


impl ULEB128 {
    pub fn as_slice(&self) -> &[u8] {
        self.0.as_slice()
    }

    pub fn as_u64(&self) -> crate::Result<u64> {
        leb128::read::unsigned(&mut self.clone().as_slice()).map_err(|_err| crate::Error::Overflow)
    }
}


pub struct Rect {
    pub left: i32,
    pub right: i32,
    pub top: i32,
    pub bottom: i32,
}

impl Rect {
    pub fn width(&self) -> i32 {
        (self.left - self.right).abs()
    }

    pub fn height(&self) -> i32 {
        (self.top - self.bottom).abs()
    }

    pub fn area(&self) -> i64 {
        i64::from(self.width()) * i64::from(self.height())
    }
}
