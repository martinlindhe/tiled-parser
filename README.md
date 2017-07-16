# tiled-parser

[![Travis-CI](https://api.travis-ci.org/martinlindhe/tiled-parser.svg)](https://travis-ci.org/martinlindhe/tiled-parser) [![Crates.io](https://img.shields.io/crates/v/tiled-parser.svg)](https://crates.io/crates/tiled-parser)

Deserializes and serializes [Tiled](http://www.mapeditor.org/) .json files, using serde_json


### Usage

In Cargo.toml:

```tiled-parser = "0.1"```

```rust
extern crate tiled_parser;

let data = include_str!("test-data/levels/super_mario.json");

// deserialize
let mut level = tiled_parser::load_level(data);
println!("{:?}", level);

level.width = 128;

// serialize
let serialized = serde_json::to_string(&level).unwrap();
println!("serialized = {}", serialized);
```


### Documentation

https://docs.rs/tiled-parser/


### License

Under [MIT](LICENSE)
