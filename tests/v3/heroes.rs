use owapi::v3::HeroPlaytime;
use serde_json::from_str;


static TEST_PLAYTIME_DATA: &'static str = include_str!("../../test_data/playtime.json");


#[test]
fn deserialisation() {
    assert_eq!(from_str::<HeroPlaytime>(TEST_PLAYTIME_DATA).unwrap(),
               HeroPlaytime {
                   junkrat: 0.03333333333333333,
                   roadhog: 0.0,
                   orisa: 0.0,
                   genji: 0.0,
                   zenyatta: 0.2833333333333333,
                   tracer: 0.0,
                   symmetra: 0.01222222222222222,
                   lucio: 1.0,
                   dva: 0.0,
                   pharah: 0.0,
                   mei: 0.0,
                   soldier76: 0.016388888888888887,
                   torbjorn: 0.016388888888888887,
                   zarya: 0.0,
                   bastion: 0.0,
                   ana: 0.06666666666666667,
                   reaper: 0.0,
                   winston: 0.0,
                   hanzo: 0.0,
                   sombra: 0.0,
                   widowmaker: 0.0,
                   reinhardt: 0.0,
                   mercy: 0.014722222222222222,
                   mccree: 0.0,
               });
}
