use owapi::v3::stats::{Statistics, OverallStats, GameStats, AverageStats};
use serde_json::from_str;


/// Solen from https://github.com/SunDwarf/OWAPI/blob/bf731c489ba18abceb281d753c72d14d75b07ad2/api.md#get-apiv3ubattletagstats
static TEST_STATS: &'static str = include_str!("../../test_data/stats.json");


#[test]
fn deserialisation() {
    assert_eq!(from_str::<Statistics>(TEST_STATS).unwrap(),
               Statistics {
                   overall_stats: OverallStats {
                       win_rate: 52,
                       level: 20,
                       prestige: 1,
                       avatar: "https://blzgdapipro-a.akamaihd.net/game/unlocks/0x0250000000000BBA.png".to_string(),
                       wins: 9,
                       games: 17,
                       comprank: 2395,
                       losses: 8,
                   },
                   game_stats: GameStats {
                       objective_kills: 121.0,
                       games_won: 9.0,
                       kpd: 1.92,
                       objective_kills_most_in_game: 26.0,
                       time_spent_on_fire_most_in_game: 0.075,
                       healing_done: 15798.0,
                       defensive_assists: 20.0,
                       offensive_assists: 4.0,
                       final_blows_most_in_game: 22.0,
                       objective_time: 0.37027777777777776,
                       melee_final_blows: 3.0,
                       medals: 37.0,
                       cards: 4.0,
                       multikill_best: 4.0,
                       multikills: 4.0,
                       defensive_assists_most_in_game: 11.0,
                       offensive_assists_most_in_game: 2.0,
                       melee_final_blow_most_in_game: 1.0,
                       damage_done: 201576.0,
                       medals_silver: 12.0,
                       medals_gold: 12.0,
                       healing_done_most_in_game: 2597.0,
                       environmental_kills: 5.0,
                       medals_bronze: 13.0,
                       solo_kills: 29.0,
                       time_spent_on_fire: 0.33999999999999997,
                       eliminations_most_in_game: 44.0,
                       final_blows: 152.0,
                       time_played: 3.0,
                       environmental_deaths: 6.0,
                       solo_kills_most_in_game: 22.0,
                       damage_done_most_in_game: 22230.0,
                       games_played: 17.0,
                       eliminations: 315.0,
                       objective_time_most_in_game: 0.060000000000000005,
                       deaths: 164.0,
                   },
                   average_stats: AverageStats {
                       healing_done_avg: 929.0,
                       eliminations_avg: 18.52,
                       melee_final_blows_avg: 0.17,
                       final_blows_avg: 8.94,
                       defensive_assists_avg: 1.0,
                       damage_done_avg: 11857.0,
                       deaths_avg: 9.64,
                       objective_time_avg: 0.021666666666666667,
                       offensive_assists_avg: 0.0,
                       solo_kills_avg: 1.7,
                       time_spent_on_fire_avg: 0.02,
                       objective_kills_avg: 7.11,
                   },
                   competitive: true,
               });
}
