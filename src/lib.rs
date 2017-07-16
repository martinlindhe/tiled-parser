extern crate serde;
extern crate serde_json;

#[macro_use]
extern crate serde_derive;

pub mod level;
pub mod layer;
pub mod tileset;

pub fn load_level(s: &str) -> level::Level {
    serde_json::from_str(&s).unwrap()
}

#[test]
fn test_load_level() {
    let data = include_str!("../test-data/levels/super_mario.json");
    let mut level = load_level(data);

    level.width = 128;

    // serialize
    let _ = serde_json::to_string(&level).unwrap();
}
