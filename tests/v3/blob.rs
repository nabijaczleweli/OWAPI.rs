use owapi::v3::blob::FullData;
use serde_json::from_str;


static TEST_HEROES: &'static str = include_str!("../../test_data/heroes.json");


#[test]
fn deserialisation() {
    assert!(from_str::<FullData>(TEST_HEROES).is_ok());
}
