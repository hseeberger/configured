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

Opinionated utility, built on top of the [config](https://crates.io/crates/config) crate, to load a configuration from well defined layers into any type which can be deserialized by [Serde](https://serde.rs/) using the case chosen by the caller (e.g. `Case::Kebab` or `Case::Snake`).

First, values from the mandatory default configuration file at `<CONFIG_DIR>/default.yaml/toml` are loaded.

Then, if the environment variable `CONFIG_OVERLAYS` is defined, its comma separated overlays (e.g. "prod" or "feat, dev") at `<CONFIG_DIR>/<overlay>.yaml/toml` are loaded from left to right as overlays, i.e. adding or overwriting already existing values.

Finally environment variables prefixed with `<CONFIG_ENV_PREFIX>__` (the prefix defaults to `CFG`) and segments separated by `__` (double underscores are used as segment separators to allow for single underscores in segment names) are used as final overlay. Each segment is converted to the [`Case`](https://docs.rs/convert_case) passed to `load`, so it matches the `#[serde(rename_all = ...)]` of the target type.

File formats are gated behind features: `yaml` (default) and `toml`.

## Example

```rust
use configured::{ Case, Configured };
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

let config = Config::load(Case::Snake)?;
```

## License ##

This code is open source software licensed under the [Apache 2.0 License](http://www.apache.org/licenses/LICENSE-2.0.html).
