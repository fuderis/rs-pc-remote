pub mod prelude;
pub mod error;     pub use error::{ Error, StdResult, Result };

pub mod input;     pub use input::{ Keyboard, Key, Mouse };
pub mod media;     pub use media::{ Media, Device, DeviceKind };
