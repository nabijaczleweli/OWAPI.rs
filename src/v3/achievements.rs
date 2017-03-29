#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub struct Achievements {
    pub defense: DefenseAchievements,
    pub offense: OffenseAchievements,
    pub support: SupportAchievements,
    pub general: GeneralAchievements,
    pub tank: TankAchievements,
    pub maps: MapsAchievements,
}

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub struct DefenseAchievements {
    pub ice_blocked: bool,
    pub triple_threat: bool,
    pub simple_geometry: bool,
    pub the_dragon_is_sated: bool,
    pub did_that_sting: bool,
    pub mine_like_a_steel_trap: bool,
    pub charge: bool,
    pub cold_snap: bool,
    pub raid_wipe: bool,
    pub armor_up: bool,
    pub roadkill: bool,
    pub smooth_as_silk: bool,
}

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub struct OffenseAchievements {
    pub whoa_there: bool,
    pub die_die_die_die: bool,
    pub its_high_noon: bool,
    pub their_own_worst_enemy: bool,
    pub clearing_the_area: bool,
    pub target_rich_environment: bool,
    pub death_from_above: bool,
    pub total_recall: bool,
    pub rocket_man: bool,
    pub slice_and_dice: bool,
    pub special_delivery: bool,
    pub waste_not_want_not: bool,
}

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub struct SupportAchievements {
    pub rapid_discord: bool,
    pub enabler: bool,
    pub huge_rez: bool,
    pub supersonic: bool,
    pub naptime: bool,
    pub huge_success: bool,
    pub the_iris_embraces_you: bool,
    pub the_car_wash: bool,
    pub the_floor_is_lava: bool,
    pub group_health_plan: bool,
}

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub struct GeneralAchievements {
    pub decorated: bool,
    pub blackjack: bool,
    pub centenary: bool,
    pub undying: bool,
    pub level_10: bool,
    pub the_path_is_closed: bool,
    pub level_50: bool,
    pub level_25: bool,
    pub decked_out: bool,
    pub survival_expert: bool,
    pub the_friend_zone: bool,
}

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub struct TankAchievements {
    pub i_am_your_shield: bool,
    pub mine_sweeper: bool,
    pub storm_earth_and_fire: bool,
    pub giving_you_the_hook: bool,
    pub power_overwhelming: bool,
    pub anger_management: bool,
    pub hog_wild: bool,
    pub the_power_of_attraction: bool,
    pub shot_down: bool,
    pub game_over: bool,
}

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub struct MapsAchievements {
    pub world_traveler: bool,
    pub lockdown: bool,
    pub cant_touch_this: bool,
    pub shutout: bool,
    pub escort_duty: bool,
    pub double_cap: bool,
}
