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
