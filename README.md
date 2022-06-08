# Configured

Utility to load configuration from three well defined layers into any type which can be deserialized by [Serde](https://serde.rs/).

First, values from the mandatory default configuration file at `<CONFIG_DIR>/default.toml` are loaded.

Then, if if the environment variable `CONFIG_ENVIRONMENT` is defined, values from the environment (e.g. "prod") specific configuration file at `<CONFIG_DIR>/<CONFIG_ENVIRONMENT>.toml` are loaded as an overlay, i.e. adding or overwriting already existing values.

Finally environment variables prefixed with `<CONFIG_ENV_PREFIX>__` and separated by `__` (double underscores are used as separators because of snake_cased keys) are used as additional overlay.

## Example

```rust
#[derive(Deserialize)]
struct Config {
    foo: Foo,
    qux: Qux,
}

#[derive(Deserialize)]
struct Foo {
    bar: String,
    baz: String,
}

#[derive(Deserialize)]
struct Qux {
    quux: String,
}

#[test]
fn test_load() {
    env::set_var(CONFIG_DIR, "test-config");
    env::set_var(CONFIG_ENVIRONMENT, "dev");
    env::set_var("APP__QUX__QUUX", "Quux2");

    let config = Config::load();
    assert!(config.is_ok());
    let config = config.unwrap();

    assert_eq!(config.foo.bar.as_str(), "Bar");
    assert_eq!(config.foo.baz.as_str(), "Baz2");
    assert_eq!(config.qux.quux.as_str(), "Quux2");
}
```

## Attribution

This utility is built on top of the fantastic [Config](https://crates.io/crates/config) library.

## License ##

This code is open source software licensed under the [Apache 2.0 License](http://www.apache.org/licenses/LICENSE-2.0.html).
