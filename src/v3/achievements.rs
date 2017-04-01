//! Achievement data structures.


/// User's completed achievements, by category.
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub struct Achievements {
    /// Achievements for heroes classified by Blizzard as "defense".
    pub defense: DefenseAchievements,
    /// Achievements for heroes classified by Blizzard as "offense".
    pub offense: OffenseAchievements,
    /// Achievements for heroes classified by Blizzard as "support".
    pub support: SupportAchievements,
    /// General achievements.
    pub general: GeneralAchievements,
    /// Achievements for heroes classified by Blizzard as "tank".
    pub tank: TankAchievements,
    /// Achievements for various maps.
    pub maps: MapsAchievements,
    /// Achievements for various events.
    pub special: Option<SpecialAchievements>,
}

/// Achievements for heroes classified by Blizzard as "defense".
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub struct DefenseAchievements {
    /// Ice Blocked achievement.
    ///
    /// Block 1200 Damage with a single use of Mei's Ice Wall.
    pub ice_blocked: bool,
    /// Triple Threat achievement.
    ///
    /// Kill 2 enemies in each of Bastion's Configurations without dying.
    pub triple_threat: bool,
    /// Simple Geometry achievement.
    ///
    /// Get 2 killing blows with a single use of Hanzo's Scatter Arrow.
    pub simple_geometry: bool,
    /// The Dragon is Sated achievement.
    ///
    /// Kill 4 enemies with one of Hanzo's Dragonstrikes.
    pub the_dragon_is_sated: bool,
    /// Did That Sting? achievement.
    ///
    /// Kill 4 enemies using Widowmaker's Venom Mine during a single game.
    pub did_that_sting: bool,
    /// Mine Like a Steel Trap achievement.
    ///
    /// Knock an enemy into your Steel Trap using Junkrat's Concussion Mine.
    pub mine_like_a_steel_trap: bool,
    /// Charge! achievement.
    ///
    /// Kill 4 enemies with a single use of Bastion's Configuration: Tank.
    pub charge: bool,
    /// Cold Snap achievement.
    ///
    /// Freeze 4 enemies at once with Mei.
    pub cold_snap: bool,
    /// Raid Wipe achievement.
    ///
    /// Kill 4 enemies during a single use of Torbjörn's Molten Core.
    pub raid_wipe: bool,
    /// Armor Up! achievement.
    ///
    /// Have one of Torbjörn's Armor Packs on 5 allies at the same time.
    pub armor_up: bool,
    /// Roadkill achievement.
    ///
    /// Kill 4 enemies with a single use of Junkrat's RIP-Tire.
    pub roadkill: bool,
    /// Smooth As Silk achievement.
    ///
    /// Kill an enemy with a Scoped Headshot while airborne as Widowmaker.
    pub smooth_as_silk: bool,
}

/// Achievements for heroes classified by Blizzard as "offense".
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub struct OffenseAchievements {
    /// Hack The Planet achievement.
    ///
    /// Hack 15 enemies without dying as Sombra in quick or competitive play.
    pub hack_the_planet: bool,
    /// Power Outage achievement.
    ///
    /// Hack 6 enemies at once as Sombra in quick or competitive play.
    pub power_outage: bool,
    /// Whoa There! achievement.
    ///
    /// Interrupt an enemy ultimate ability with McCree's Flashbang.
    pub whoa_there: bool,
    /// Die Die Die... Die achievement.
    ///
    /// Kill 4 enemies with a single use of Reaper's Death Blossom.
    pub die_die_die_die: bool,
    /// It's High Noon achievement.
    ///
    /// Get 4 killing blows with a single use of McCree's Deadeye.
    pub its_high_noon: bool,
    /// Their Own Worst Enemy achievement.
    ///
    /// Kill 2 enemies with a single use of Genji's Deflect.
    pub their_own_worst_enemy: bool,
    /// Clearing the Area achievement.
    ///
    /// Knock an enemy to their death using Pharah's Concussive Blast.
    pub clearing_the_area: bool,
    /// Target Rich Environment achievement.
    ///
    /// Kill 4 enemies with a single use of Soldier: 76's Tactical Visor.
    pub target_rich_environment: bool,
    /// Death From Above achievement.
    ///
    /// Kill 4 enemies in a row without touching the ground as Pharah.
    pub death_from_above: bool,
    /// Total Recall achievement.
    ///
    /// Recover 400 health using Tracer's Recall without dying.
    pub total_recall: bool,
    /// Rocket Man achievement.
    ///
    /// Get 2 killing blows with a single use of Soldier: 76's Helix Rockets.
    pub rocket_man: bool,
    /// Slice and Dice achievement.
    ///
    /// Kill 4 enemies with a single use of Genji's Dragonblade.
    pub slice_and_dice: bool,
    /// Special Delivery achievement.
    ///
    /// Stick 4 of Tracer's Pulse Bombs onto enemies in a single game.
    pub special_delivery: bool,
    /// Waste Not, Want Not achievement.
    ///
    /// Get 3 solo kills with a single clip of Reaper's shotguns.
    pub waste_not_want_not: bool,
}

/// Achievements for heroes classified by Blizzard as "support".
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub struct SupportAchievements {
    /// Rapid Discord achievement.
    ///
    /// Get 4 kills or assists with Zenyatta's Orb of Discord within 6 seconds.
    pub rapid_discord: bool,
    /// Enabler achievement.
    ///
    /// Get 4 kills or assists with a single use of Ana's Nano Boost.
    pub enabler: bool,
    /// Huge Rez achievement.
    ///
    /// Resurrect 4 players at once with Mercy.
    pub huge_rez: bool,
    /// Supersonic achievement.
    ///
    /// Block 1000 damage with a single use of Lúcio's Sound Barrier.
    pub supersonic: bool,
    /// Naptime achievement.
    ///
    /// Interrupt an enemy ultimate ability with Ana's Sleep Dart.
    pub naptime: bool,
    /// Huge Success achievement.
    ///
    /// Teleport 20 players in a single game as Symmetra.
    pub huge_success: bool,
    /// The Iris Embraces You achievement.
    ///
    /// Restore 1500 health with a single use of Zenyatta's Transcendence.
    pub the_iris_embraces_you: bool,
    /// The Car Wash achievement.
    ///
    /// Hit an enemy with 6 beams simultaneously as Symmetra.
    pub the_car_wash: bool,
    /// The Floor is Lava achievement.
    ///
    /// Get 3 killing blows while wall riding as Lúcio without dying.
    pub the_floor_is_lava: bool,
    /// Group Health Plan achievement.
    ///
    /// Restore 200 health for 5 players without dying as Mercy.
    pub group_health_plan: bool,
}

/// General achievements.
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub struct GeneralAchievements {
    /// Decorated achievement.
    ///
    /// Earn 50 postgame medals.
    pub decorated: bool,
    /// Blackjack achievement.
    ///
    /// Earn 21 postgame cards.
    pub blackjack: bool,
    /// Centenary achievement.
    ///
    /// Win 150 games.
    pub centenary: bool,
    /// Undying achievement.
    ///
    /// Get a 20 player kill streak in a game.
    pub undying: bool,
    /// Level 10 achievement.
    ///
    /// Reach level 10.
    pub level_10: bool,
    /// The Path is Closed achievement.
    ///
    /// Destroy 3 of Symmetra's Teleporters in a single game.
    pub the_path_is_closed: bool,
    /// Level 50 achievement.
    ///
    /// Reach level 50.
    pub level_50: bool,
    /// Level 25 achievement.
    ///
    /// Reach level 25.
    pub level_25: bool,
    /// Decked Out achievement.
    ///
    /// Collect 50 unlocks for a single hero.
    pub decked_out: bool,
    /// Survival Expert achievement.
    ///
    /// Use Health Packs to heal 900 health in a single life.
    pub survival_expert: bool,
    /// The Friend Zone achievement.
    ///
    /// Play a game in a group with a friend.
    pub the_friend_zone: bool,
}

/// Achievements for heroes classified by Blizzard as "tank".
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub struct TankAchievements {
    /// Halt State achievement.
    ///
    /// Pull enemies into 1000 damage with a single use of Orisa's Halt! in quick or competitive play.
    pub halt_state: bool,
    /// Overclocked achievement.
    ///
    /// Amplify 900 damage with a single use of Orisa's Supercharger in quick or competitive play.
    pub overclocked: bool,
    /// I Am Your Shield achievement.
    ///
    /// Block 8000 damage with Reinhardt's Barrier Field without dying.
    pub i_am_your_shield: bool,
    /// Mine Sweeper achievement.
    ///
    /// Destroy 10 Turrets or Traps using Winston's Tesla Cannon without dying.
    pub mine_sweeper: bool,
    /// Storm, Earth and Fire achievement.
    ///
    /// Land Reinhardt's Fire Strike and Charge after an Earthshatter Stun.
    pub storm_earth_and_fire: bool,
    /// Giving You The Hook achievement.
    ///
    /// Interrupt an enemy ultimate ability with Roadhog's Chain Hook.
    pub giving_you_the_hook: bool,
    /// Power Overwhelming achievement.
    ///
    /// Keep Zarya's Particle Cannon above 70 Energy for 60 seconds.
    pub power_overwhelming: bool,
    /// Anger Management achievement.
    ///
    /// Damage 6 enemies a single use of Winston's Primal Rage.
    pub anger_management: bool,
    /// Hog Wild achievement.
    ///
    /// Knock 2 enemies to their deaths with one use of Roadhog's Whole Hog.
    pub hog_wild: bool,
    /// The Power of Attraction achievement.
    ///
    /// Capture 5 enemies in a single use of Zarya's Graviton Surge.
    pub the_power_of_attraction: bool,
    /// Shot Down achievement.
    ///
    /// Prevent 1500 damage with a single use of D.Va's Defense Matrix.
    pub shot_down: bool,
    /// Game Over achievement.
    ///
    /// Kill 4 enemies with a single use of D.Va's Self-Destruct.
    pub game_over: bool,
}

/// Achievements for various maps.
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub struct MapsAchievements {
    /// World Traveler achievement.
    ///
    /// Win a quick or competitive play game on 12 different maps.
    pub world_traveler: bool,
    /// Lockdown achievement.
    ///
    /// Win an Assault map on defense without losing the first objective.
    pub lockdown: bool,
    /// Can't Touch This achievement.
    ///
    /// Prevent the attacking team from touching the Payload for 1 minute.
    pub cant_touch_this: bool,
    /// Shutout achievement.
    ///
    /// Win a Control map without the enemy capturing an objective.
    pub shutout: bool,
    /// Escort Duty achievement.
    ///
    /// Push a Payload 100 meters without leaving it.
    pub escort_duty: bool,
    /// Double Cap achievement.
    ///
    /// Capture both objectives on an Assault map without dying.
    pub double_cap: bool,
}

/// Achievements for various events.
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub struct SpecialAchievements {
    /// Survived The Night achievement.
    ///
    /// Win Junkenstein's Revenge on Medium difficulty.
    pub survived_the_night: bool,
    /// Flagbearer achievement.
    ///
    /// Win Capture the Rooster with as score of 3 to 0.
    pub flagbearer: bool,
    /// Cleanup Duty achievement.
    ///
    /// Recover a flag in a game of Capture the Rooster.
    pub cleanup_duty: bool,
    /// Not A Scratch achievement.
    ///
    /// Win Junkenstein's Revenge on Medium difficulty with no damage to the door.
    pub not_a_scratch: bool,
    /// Four They Were achievement.
    ///
    /// Win Junkenstein's Revenge using each of the 4 heroes.
    pub four_they_were: bool,
    /// Whap! achievement.
    ///
    /// Kill an enemy with a snowball from 25 meters away in Mei's Snowball Offensive.
    pub whap: bool,
    /// Cool As Ice achievement.
    ///
    /// Kill 4 enemies without missing in Mei's Snowball Offensive.
    pub cool_as_ice: bool,
    /// Ambush! achievement.
    ///
    /// Kill 3 enemies while they are picking up snow in a game of Mei's Snowball Offensive.
    pub ambush: bool,
    /// Snowed In achievement.
    ///
    /// Win Mei's Snowball Offensive without losing a round.
    pub snowed_in: bool,
    /// Held The Door achievement.
    ///
    /// Win Junkenstein's Revenge on Hard difficulty.
    pub held_the_door: bool,
}
