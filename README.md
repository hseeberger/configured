# Configured

[![Crates.io][crates-badge]][crates-url]
[![license][license-badge]][license-url]
[![build][build-badge]][build-url]

[crates-badge]: https://img.shields.io/crates/v/configured
[crates-url]: https://crates.io/crates/configured
[license-badge]: https://img.shields.io/github/license/hseeberger/configured
[license-url]: https://github.com/hseeberger/configured/blob/main/LICENSE
[build-badge]: https://img.shields.io/github/actions/workflow/status/hseeberger/configured/ci.yaml
[build-url]: https://github.com/hseeberger/configured/actions/workflows/ci.yaml

Opinionated utility to load a configuration from three well defined layers into any type which can be deserialized by [Serde](https://serde.rs/) using kebab-case.

First, values from the mandatory default configuration file at `<CONFIG_DIR>/default.yaml` are loaded.

Then, if the environment variable `CONFIG_OVERLAYS` is defined, its comma separated overlays (e.g. "prod" or "feat, dev") at `<CONFIG_DIR>/<overlay>.yaml` are loaded from left to right as overlays, i.e. adding or overwriting already existing values.

Finally environment variables prefixed with `<CONFIG_ENV_PREFIX>__` and segments separated by `__` (double underscores are used as segment separators to allow for single underscores in segment names) are used as final overlay.

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
    quux: String,
    corge_grault: String,
}

#[test]
fn test_load() -> Result<(), Error> {
    env::set_var(CONFIG_DIR, "test-config");
    env::set_var(CONFIG_OVERLAYS, "feat, dev");
    env::set_var("APP__QUX__CORGE_GRAULT", "corge-grault-env");

    let config = Config::load()?;

    assert_eq!(config.foo.bar.as_str(), "bar");
    assert_eq!(config.foo.baz.as_str(), "baz-dev");
    assert_eq!(config.qux.quux.as_str(), "quux-feat");
    assert_eq!(config.qux.corge_grault.as_str(), "corge-grault-env");

    Ok(())
}
```

## Attribution

This utility is built on top of the fantastic [Config](https://crates.io/crates/config) library.

## License ##

This code is open source software licensed under the [Apache 2.0 License](http://www.apache.org/licenses/LICENSE-2.0.html).
