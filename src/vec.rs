/// A [i16]-sized [Vec] serialized as a sequence of bits.
pub struct VecI16Flags (pub Vec<bool>);

/// A ULEB128-sized [Vec] serialized as a sequence of `T`.
pub struct VecULEB128<T> (pub Vec<T>);

/// A [i16]-sized [Vec] serialized as a sequence of `T`.
pub struct VecI16<T> (pub Vec<T>);

/// A [i32]-sized [Vec] serialized as a sequence of `T`.
pub struct VecI32<T> (pub Vec<T>);
