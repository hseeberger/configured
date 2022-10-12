# Configured

Opinionated utility to load a configuration from three well defined layers into any type which can
be deserialized by [Serde](https://serde.rs/) using kebab-case.

First, values from the mandatory default configuration file at `<CONFIG_DIR>/default.toml` are
loaded.

Then, if if the environment variable `CONFIG_ENVIRONMENT` is defined, values from the environment
(e.g. "prod") specific configuration file at `<CONFIG_DIR>/<CONFIG_ENVIRONMENT>.toml` are loaded as
an overlay, i.e. adding or overwriting already existing values.

Finally environment variables prefixed with `<CONFIG_ENV_PREFIX>__` and segments separated by `__`
(double underscores are used as segment separators to allow for single underscores in segment names)
are used as additional overlay.

## Example

```rust
#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
struct Config {
    foo: Foo,
    qux: Qux,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
struct Foo {
    bar: String,
    baz: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
struct Qux {
    quux_quux: String,
}

#[test]
fn test_load() {
    env::set_var(CONFIG_DIR, "test-config");
    env::set_var(CONFIG_ENVIRONMENT, "dev");
    env::set_var("APP__QUX__QUUX-QUUX", "Quux2");

    let config = Config::load();
    assert!(config.is_ok());
    let config = config.unwrap();

    assert_eq!(config.foo.bar.as_str(), "Bar");
    assert_eq!(config.foo.baz.as_str(), "Baz2");
    assert_eq!(config.qux.quux_quux.as_str(), "Quux2");
}
```

## Attribution

This utility is built on top of the fantastic [Config](https://crates.io/crates/config) library.

## License ##

This code is open source software licensed under the [Apache 2.0
License](http://www.apache.org/licenses/LICENSE-2.0.html).
