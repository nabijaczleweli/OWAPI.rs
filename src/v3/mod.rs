pub mod blob;
pub mod stats;
pub mod achievements;


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HeroPlaytime {
    pub junkrat: f64,
    pub roadhog: f64,
    pub orisa: f64,
    pub genji: f64,
    pub zenyatta: f64,
    pub tracer: f64,
    pub symmetra: f64,
    pub lucio: f64,
    pub dva: f64,
    pub pharah: f64,
    pub mei: f64,
    pub soldier76: f64,
    pub torbjorn: f64,
    pub zarya: f64,
    pub bastion: f64,
    pub ana: f64,
    pub reaper: f64,
    pub winston: f64,
    pub hanzo: f64,
    pub sombra: f64,
    pub widowmaker: f64,
    pub reinhardt: f64,
    pub mercy: f64,
    pub mccree: f64,
}
