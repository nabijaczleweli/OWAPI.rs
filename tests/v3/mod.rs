use owapi;

mod blob;
mod heroes;
mod statistics;
mod achievements;


#[test]
fn acquire() {
    assert!(owapi::v3::acquire("наб-2170").is_ok());
}
