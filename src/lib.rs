//! The [OWAPI](https://github.com/SunDwarf/OWAPI) bindings.
//!
//! Currently only [v3](https://github.com/SunDwarf/OWAPI/blob/8167f61d965cdfd5e9604fcd3605f11fd4425585/api.md) is supported.
//!
//! Only the latest version is reexported at top-level.


#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate reqwest;
extern crate serde;

mod user;
mod error;

pub mod v3;

pub use self::v3::*;
pub use self::error::Error;
pub use self::user::BNetUser;
