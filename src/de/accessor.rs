pub struct ValueSized<'a, 'de: 'a, R> where R: std::io::Read {
    pub de: &'a mut crate::de::ReadDeserializer<'de, R>,
    pub size: usize,
}

impl<'a, 'de, R> serde::de::SeqAccess<'de> for ValueSized<'a, 'de, R> where R: std::io::Read {
    type Error = crate::Error;

    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>, Self::Error> where T: serde::de::DeserializeSeed<'de> {
        match self.size {
            0 => Ok(None),
            _ => seed.deserialize(&mut *self.de).map(Some),
        }
    }

    fn size_hint(&self) -> Option<usize> {
        Some(self.size)
    }
}
