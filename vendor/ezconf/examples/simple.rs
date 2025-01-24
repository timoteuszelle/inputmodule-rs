#[macro_use]
extern crate log;
extern crate env_logger;
extern crate ezconf;

fn main() {
    env_logger::Builder::from_default_env()
        .filter_level(log::LevelFilter::Trace)
        .init();
    let config = ezconf::INIT;

    config
        .init([ezconf::Source::File("tests/test.toml")].iter())
        .unwrap();

    let v = config.get_or::<String>("string.a", "Hello String".into());
    info!("Value: {:?}", v);
}
