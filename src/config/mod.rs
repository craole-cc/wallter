pub mod default;
pub use default::*;

pub mod color;
pub use color::{Config as Color, Mode as ColorMode};

pub mod search;
pub use search::Config as Search;

pub mod monitor;
pub use monitor::Config as Monitor;

pub mod path;
pub use path::{Config as Path, Type as PathType};

pub mod slideshow;
pub use slideshow::Config as Slideshow;
