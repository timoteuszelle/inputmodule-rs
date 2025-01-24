extern crate env_logger;
extern crate ezconf;

#[test]
fn test_simple() {
    let _ = env_logger::try_init();
    let config = ezconf::INIT;

    config
        .init([ezconf::Source::File("tests/test.toml")].iter())
        .unwrap();
}

#[test]
fn test_nosource() {
    let _ = env_logger::try_init();
    let config = ezconf::INIT;

    let res = config
        .init(
            [
                ezconf::Source::File("tests/no-config.toml"),
                ezconf::Source::File("tests/no-config2.toml"),
                ezconf::Source::File("tests/no-config-again.toml"),
            ].iter(),
        )
        .unwrap();

    assert_eq!(res, false);
}

#[test]
fn test_double_load_fail() {
    let _ = env_logger::try_init();
    let config = ezconf::INIT;

    config
        .init([ezconf::Source::File("tests/no-config.toml")].iter())
        .unwrap();
    config
        .init([ezconf::Source::File("tests/test.toml")].iter())
        .unwrap_err();
}

#[test]
fn test_get() {
    let _ = env_logger::try_init();
    let config = ezconf::INIT;

    config
        .init([ezconf::Source::File("tests/test.toml")].iter())
        .unwrap();

    assert_eq!(config.get::<String>("string.a").unwrap(), "Foo");
    assert_eq!(config.get::<String>("string.non-existing"), None);
    assert_eq!(config.get::<u8>("integer.a").unwrap(), 1);
    assert_eq!(config.get::<i32>("integer.c").unwrap(), -324);
    assert_eq!(config.get::<f32>("float.a").unwrap(), 1.4142135);
    assert_eq!(config.get::<bool>("boolean.available").unwrap(), true);
}

#[test]
fn test_get_or() {
    let _ = env_logger::try_init();
    let config = ezconf::INIT;

    config
        .init([ezconf::Source::File("tests/test.toml")].iter())
        .unwrap();

    assert_eq!(config.get_or::<String>("string.a", "Hello".into()), "Foo");
    assert_eq!(config.get_or::<String>("string.z", "Hello".into()), "Hello");
}

#[test]
fn test_memory_source() {
    let _ = env_logger::try_init();
    let config = ezconf::INIT;

    config
        .init(
            [
                ezconf::Source::File("tests/this-config-doesnt-exist.toml"),
                ezconf::Source::Memory(
                    r#"[foo]
bar = "baz"

[hello]
world = 42"#,
                ),
            ].iter(),
        )
        .unwrap();

    assert_eq!(config.get_or::<String>("foo.bar", "Hello".into()), "baz");
    assert_eq!(config.get_or::<usize>("hello.world", 0), 42);
    assert_eq!(config.get_or::<bool>("non.existing", false), false);
}
