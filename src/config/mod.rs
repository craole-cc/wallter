mod default;
pub use default::*;

mod color;
pub use color::{Config as Color, Mode as ColorMode};

mod search;
pub use search::Config as Search;

pub mod monitor;
pub use monitor::Config as Monitor;

mod path;
pub use path::{Config as Path, Type as PathType};

mod slideshow;
pub use slideshow::Config as Slideshow;
