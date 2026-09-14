# configured

[![license][license-badge]][license-url]
[![build][build-badge]][build-url]
[![docs][docs-badge]][docs-url]

[license-badge]: https://img.shields.io/github/license/hseeberger/configured
[license-url]: https://github.com/hseeberger/configured/blob/main/LICENSE
[build-badge]: https://img.shields.io/github/actions/workflow/status/hseeberger/configured/ci.yaml
[build-url]: https://github.com/hseeberger/configured/actions/workflows/ci.yaml
[docs-badge]: https://img.shields.io/docsrs/configured/latest
[docs-url]: https://docs.rs/configured/latest/configured/

Opinionated utility, built on top of the [config](https://crates.io/crates/config) crate, to load a configuration from well defined layers into any type which can be deserialized by [Serde](https://serde.rs/), using the `LoadOptions` given by the caller.

First, values from the mandatory default configuration file at `<config_dir>/default.yaml/toml` are loaded.

Then the overlays (e.g. "prod" or "feat" and "dev") at `<config_dir>/<overlay>.yaml/toml` are loaded from left to right as overlays, i.e. adding or overwriting already existing values.

Finally environment variables prefixed with `<config_env_prefix>__` and segments separated by `__` (double underscores are used as segment separators to allow for single underscores in segment names) are used as final overlay. Each segment is converted to the [`Case`](https://docs.rs/convert_case) of the `LoadOptions` (e.g. `Case::Kebab` or `Case::Snake`, the default), so it matches the `#[serde(rename_all = ...)]` of the target type.

The configuration directory, the overlays and the environment variable prefix can each be set on the `LoadOptions`. A value which is not set there is taken from the respective environment variable (`CONFIG_DIR`, `CONFIG_OVERLAYS` or `CONFIG_ENV_PREFIX`) and, if that one is not defined either, from the fallback: `config` for the directory, no overlays and `CFG` for the prefix. Setting an empty list of overlays means no overlays, even if `CONFIG_OVERLAYS` is defined.

File formats are gated behind features: `yaml` (default) and `toml`.

## Example

```rust
use configured::{ Case, Configured, LoadOptions };
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Config {
    contact: Contact,
    address: Address,
}

#[derive(Debug, Deserialize)]
struct Contact {
    first_name: String,
    last_name: String,
}

#[derive(Debug, Deserialize)]
struct Address {
    street_name: String,
    zip_code: String,
}

let config = Config::load(LoadOptions::default())?;

let options = LoadOptions::default().config_dir("test-config").case(Case::Kebab);
let config = Config::load(options)?;
```

## License ##

This code is open source software licensed under the [Apache 2.0 License](http://www.apache.org/licenses/LICENSE-2.0.html).
