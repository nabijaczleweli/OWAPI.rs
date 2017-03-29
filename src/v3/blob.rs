use serde_json::{Value as JsonValue, from_value as from_json_value};
use self::super::achievements::Achievements;
use serde::{Serialize, Deserialize, Deserializer};
use self::super::stats::Statistics;
use std::collections::BTreeMap;
use self::super::HeroPlaytime;
use std::fmt::Debug;


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FullData {
    pub eu: RegionalData,
    pub us: RegionalData,
    pub kr: RegionalData,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct RegionalData {
    pub stats: Option<BiMode<Statistics>>,
    pub achievements: Option<Achievements>,
    pub heroes: Option<HeroesContainer>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BiMode<T: Debug + Clone + PartialEq + Serialize + Deserialize> {
    pub competitive: T,
    pub quickplay: T,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HeroesContainer {
    pub playtime: BiMode<HeroPlaytime>,
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
