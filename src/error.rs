/// Base error of this library.
#[derive(Clone, Debug, PartialEq)]
pub enum Error {

    /// Error raised by a consumer of this library.
    Message(String),

    /// Tried to serialize a type that is not supported by the "altar" file type.
    Unsupported,

    /// An IO error occurred while serializing a value.
    IO,

    /// An overflow of some kind occurred while serializing a value.
    Overflow,

}

/// `serde-altar` errors are regular `std::error::Error`.
impl std::error::Error for Error {}

/// `serde-altar` errors also are `serde::ser::Error`.
impl serde::ser::Error for Error {

    /// Allow a consumer of the library to create their own custom serialization error.
    fn custom<T: std::fmt::Display>(msg: T) -> Self {
        Error::Message(msg.to_string())
    }

}

/// `serde-altar` errors also are `serde::de::Error`.
impl serde::de::Error for Error {

    /// Allow a consumer of the library to create their own custom deserialization error.
    fn custom<T: std::fmt::Display>(msg: T) -> Self {
        Error::Message(msg.to_string())
    }

}

/// Allow displaying a message for `Error`.
impl std::fmt::Display for Error {

    /// Format the error appropriately. 
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            // Custom errors should display their own message.
            Error::Message(msg) => f.write_str(msg),
        }
    }

}

/// Base result type of this library.
pub type Result<T> = std::result::Result<T, Error>;
