//! The [OWAPI](https://github.com/SunDwarf/OWAPI)
//! [v3](https://github.com/SunDwarf/OWAPI/blob/8167f61d965cdfd5e9604fcd3605f11fd4425585/api.md)
//! API bindings.


pub mod blob;
pub mod stats;
pub mod achievements;

use reqwest;
use serde_json::from_reader;
use self::super::{BNetUser, Error};


/// Download full information about the specified player.
///
/// # Examples
///
/// ```
/// # use owapi::v3::acquire;
/// println!("nabijaczleweli has played {} hours of Zenyatta in competitive",
///          acquire("наб-2170").unwrap().eu.heroes.unwrap().playtime.competitive.zenyatta);
/// ```
pub fn acquire<U: BNetUser>(player: U) -> Result<self::blob::FullData, Error> {
    Ok(try!(from_reader(try!(reqwest::get(&format!("https://owapi.net/api/v3/u/{}/blob", player.identifier()))))))
}


/// Time dedicated to playing each hero.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HeroPlaytime {
    /// Hours spent playing Junkrat.
    pub junkrat: f64,
    /// Hours spent playing Roadhog.
    pub roadhog: f64,
    /// Hours spent playing OR15A.
    pub orisa: f64,
    /// Hours spent playing Genji.
    pub genji: f64,
    /// Hours spent playing Zenyatta.
    pub zenyatta: f64,
    /// Hours spent playing Tracer.
    pub tracer: f64,
    /// Hours spent playing Symmetra.
    pub symmetra: f64,
    /// Hours spent playing Lúcio.
    pub lucio: f64,
    /// Hours spent playing D.VA.
    pub dva: f64,
    /// Hours spent playing Pharah.
    pub pharah: f64,
    /// Hours spent playing Mei.
    pub mei: f64,
    /// Hours spent playing Soldier: 76.
    pub soldier76: f64,
    /// Hours spent playing Torbjörn.
    pub torbjorn: f64,
    /// Hours spent playing Zarya.
    pub zarya: f64,
    /// Hours spent playing Bastion.
    pub bastion: f64,
    /// Hours spent playing Ana.
    pub ana: f64,
    /// Hours spent playing Reaper.
    pub reaper: f64,
    /// Hours spent playing Winston.
    pub winston: f64,
    /// Hours spent playing Hanzo.
    pub hanzo: f64,
    /// Hours spent playing Sombra.
    pub sombra: f64,
    /// Hours spent playing Widowmaker.
    pub widowmaker: f64,
    /// Hours spent playing Reinhardt.
    pub reinhardt: f64,
    /// Hours spent playing Mercy.
    pub mercy: f64,
    /// Hours spent playing McCree.
    pub mccree: f64,
}
