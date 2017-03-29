use owapi::v3::achievements::{Achievements, DefenseAchievements, OffenseAchievements, SupportAchievements, GeneralAchievements, TankAchievements,
                              MapsAchievements};
use serde_json::from_str;


/// Solen from
/// https://github.com/SunDwarf/OWAPI/blob/bf731c489ba18abceb281d753c72d14d75b07ad2/api.md#get-apiv3ubattletagachievements
static TEST_ACHIEVEMENTS: &'static str = include_str!("../../test_data/achievements.json");


#[test]
fn deserialisation() {
    assert_eq!(from_str::<Achievements>(TEST_ACHIEVEMENTS).unwrap(),
               Achievements {
                   defense: DefenseAchievements {
                       ice_blocked: true,
                       triple_threat: false,
                       simple_geometry: false,
                       the_dragon_is_sated: false,
                       did_that_sting: true,
                       mine_like_a_steel_trap: true,
                       charge: false,
                       cold_snap: false,
                       raid_wipe: true,
                       armor_up: true,
                       roadkill: true,
                       smooth_as_silk: true,
                   },
                   offense: OffenseAchievements {
                       whoa_there: true,
                       die_die_die_die: false,
                       its_high_noon: false,
                       their_own_worst_enemy: false,
                       clearing_the_area: true,
                       target_rich_environment: true,
                       death_from_above: false,
                       total_recall: true,
                       rocket_man: false,
                       slice_and_dice: false,
                       special_delivery: false,
                       waste_not_want_not: false,
                   },
                   support: SupportAchievements {
                       rapid_discord: false,
                       enabler: false,
                       huge_rez: true,
                       supersonic: true,
                       naptime: false,
                       huge_success: false,
                       the_iris_embraces_you: false,
                       the_car_wash: true,
                       the_floor_is_lava: false,
                       group_health_plan: true,
                   },
                   general: GeneralAchievements {
                       decorated: true,
                       blackjack: true,
                       centenary: true,
                       undying: true,
                       level_10: true,
                       the_path_is_closed: true,
                       level_50: true,
                       level_25: true,
                       decked_out: false,
                       survival_expert: false,
                       the_friend_zone: true,
                   },
                   tank: TankAchievements {
                       i_am_your_shield: true,
                       mine_sweeper: false,
                       storm_earth_and_fire: false,
                       giving_you_the_hook: true,
                       power_overwhelming: true,
                       anger_management: false,
                       hog_wild: true,
                       the_power_of_attraction: true,
                       shot_down: false,
                       game_over: false,
                   },
                   maps: MapsAchievements {
                       world_traveler: true,
                       lockdown: true,
                       cant_touch_this: true,
                       shutout: true,
                       escort_duty: true,
                       double_cap: true,
                   },
               });
}
