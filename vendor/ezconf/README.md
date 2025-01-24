ezconf [![crates.io page](http://meritbadge.herokuapp.com/ezconf)](https://crates.io/crates/ezconf) [![Build Status](https://travis-ci.org/Rahix/ezconf.svg?branch=master)](https://travis-ci.org/Rahix/ezconf) [![docs.rs](https://docs.rs/ezconf/badge.svg)](https://docs.rs/ezconf)
======

A library to add configuration options to your project with as little
boilerplate as possible. Uses `toml` as the configuration format.

## Example ##

```rust
extern crate ezconf;

static CONFIG: ezconf::Config = ezconf::INIT;

fn main() {
    CONFIG
        .init([ezconf::Source::File("tests/test.toml")].iter())
        .unwrap();

    let v = CONFIG.get_or::<String>("string.a", "Hello String".into());
    println!("Value: {:?}", v);
}
```

## License ##
ezconf is licensed under either of

 * Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.
