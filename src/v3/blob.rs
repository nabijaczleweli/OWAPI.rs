//! Structures for handling the blobbed data.
//!
//! See `FullData` for top-level and work down from there.


use serde_json::{Value as JsonValue, from_value as from_json_value};
use self::super::achievements::Achievements;
use serde::{Serialize, Deserialize, Deserializer};
use self::super::stats::Statistics;
use std::collections::BTreeMap;
use self::super::HeroPlaytime;
use std::fmt::Debug;


/// Data for all regions.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FullData {
    /// Data for Europe.
    pub eu: RegionalData,
    /// Data for the U.S.
    pub us: RegionalData,
    /// Data for Korea.
    pub kr: RegionalData,
}

/// Data for a single region.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct RegionalData {
    /// User statistics.
    pub stats: Option<BiMode<Statistics>>,
    /// Achievements achieved by user.
    pub achievements: Option<Achievements>,
    /// Hero data.
    pub heroes: Option<HeroesContainer>,
}

/// Something that has a competitive and a quickplay variant.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BiMode<T: Debug + Clone + PartialEq + Serialize + Deserialize> {
    /// Variant for competitive play.
    pub competitive: T,
    /// Variant for quickplay.
    pub quickplay: T,
}

/// Hero data.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HeroesContainer {
    /// Time played for each of the heroes.
    pub playtime: BiMode<HeroPlaytime>,
    /// Statistics for various heroes.
    ///
    /// This is just a map because it's a bloody mess all-round.
    pub stats: BiMode<BTreeMap<String, JsonValue>>,
}


impl Deserialize for RegionalData {
    fn deserialize<D: Deserializer>(deserializer: D) -> Result<Self, D::Error> {
        let val: JsonValue = try!(Deserialize::deserialize(deserializer));
        Ok(RegionalData {
            stats: val.get("stats").into_iter().filter(|v| !v.is_null()).flat_map(|v| from_json_value(v.clone())).next(),
            achievements: val.get("achievements").into_iter().filter(|v| !v.is_null()).flat_map(|v| from_json_value(v.clone())).next(),
            heroes: val.get("heroes").into_iter().filter(|v| !v.is_null()).flat_map(|v| from_json_value(v.clone())).next(),
        })
    }
}
