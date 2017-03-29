#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Statistics {
    pub overall_stats: OverallStats,
    pub game_stats: GameStats,
    pub average_stats: AverageStats,
    pub competitive: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OverallStats {
    pub win_rate: u64,
    pub level: u64,
    pub prestige: u64,
    pub avatar: String,
    pub wins: u64,
    pub games: u64,
    pub comprank: u64,
    pub losses: u64,
}

#[derive(Debug, Copy, Clone, PartialEq, Serialize, Deserialize)]
pub struct GameStats {
    pub objective_kills: f64,
    pub games_won: f64,
    pub kpd: f64,
    pub objective_kills_most_in_game: f64,
    pub time_spent_on_fire_most_in_game: f64,
    pub healing_done: f64,
    pub defensive_assists: f64,
    pub offensive_assists: f64,
    pub final_blows_most_in_game: f64,
    pub objective_time: f64,
    pub melee_final_blows: f64,
    pub medals: f64,
    pub cards: f64,
    pub multikill_best: f64,
    pub multikills: f64,
    pub defensive_assists_most_in_game: f64,
    pub offensive_assists_most_in_game: f64,
    pub melee_final_blow_most_in_game: f64,
    pub damage_done: f64,
    pub medals_silver: f64,
    pub medals_gold: f64,
    pub healing_done_most_in_game: f64,
    pub environmental_kills: f64,
    pub medals_bronze: f64,
    pub solo_kills: f64,
    pub time_spent_on_fire: f64,
    pub eliminations_most_in_game: f64,
    pub final_blows: f64,
    pub time_played: f64,
    pub environmental_deaths: f64,
    pub solo_kills_most_in_game: f64,
    pub damage_done_most_in_game: f64,
    pub games_played: f64,
    pub eliminations: f64,
    pub objective_time_most_in_game: f64,
    pub deaths: f64,
}

#[derive(Debug, Copy, Clone, PartialEq, Serialize, Deserialize)]
pub struct AverageStats {
    pub healing_done_avg: f64,
    pub eliminations_avg: f64,
    pub melee_final_blows_avg: f64,
    pub final_blows_avg: f64,
    pub defensive_assists_avg: f64,
    pub damage_done_avg: f64,
    pub deaths_avg: f64,
    pub objective_time_avg: f64,
    pub offensive_assists_avg: f64,
    pub solo_kills_avg: f64,
    pub time_spent_on_fire_avg: f64,
    pub objective_kills_avg: f64,
}
