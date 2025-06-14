mod default;
pub use default::*;

mod api;
pub use api::Config as Api;

pub mod monitor;
pub use monitor::Config as Monitor;

mod path;
pub use path::{Config as Path, Type as PathType};

mod slideshow;
pub use slideshow::Config as Slideshow;
