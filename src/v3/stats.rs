//! Statistics data structures.
//!
//! These stats include overall statistics, like winrate, level, SR,
//! average statistics, like healing, and all-game statistics, like kills per life.


use serde_json::Value as JsonValue;
use std::collections::BTreeMap;


/// All user statistics.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Statistics {
    /// Overall statistics, incl. winrate, game count, prestige, level, etc.
    pub overall_stats: OverallStats,
    /// Statistics over all games.
    ///
    /// This is just a map due to being a bloody inconsistent mess.
    pub game_stats: BTreeMap<String, JsonValue>,
    /// Blizzard-calculated average per-game statistics.
    ///
    /// This is just a map due to being a bloody inconsistent mess.
    pub average_stats: BTreeMap<String, JsonValue>,
    /// Whether these statistics are for quickplay or competitive.
    pub competitive: bool,
}

/// Overall statistics.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OverallStats {
    /// % of games won.
    pub win_rate: f64,
    /// User's level, excl. prestige ("stars")
    ///
    /// The user's numerical level can be calculated using the `(prestige * 100) + level` formula.
    pub level: u64,
    /// User's prestige ("stars")
    ///
    /// The user's numerical level can be calculated using the `(prestige * 100) + level` formula.
    pub prestige: u64,
    /// URL to user's avatar (little chooseable image, e.g. a pachimari).
    pub avatar: String,
    /// URL to user's portrait.
    pub rank_image: Option<String>,
    /// User's competitive "tier", like `"gold"` or `"bronze"`.
    pub tier: Option<String>,
    /// Amount of games won overall.
    pub wins: u64,
    /// Amount of games played overall.
    pub games: u64,
    /// User's SR.
    pub comprank: u64,
    /// Amount of games lost overall.
    pub losses: u64,
    /// Amount of games tied overall.
    pub ties: Option<u64>,
}
