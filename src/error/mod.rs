mod default;
pub use default::Error;
pub type Result<T> = std::result::Result<T, Error>;
