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

Opinionated utility, built on top of the [config](https://crates.io/crates/config) crate, to load a configuration from well defined layers into any type which can be deserialized by [Serde](https://serde.rs/) using kebab-case.

First, values from the mandatory default configuration file at `<CONFIG_DIR>/default.yaml` are loaded.

Then, if the environment variable `CONFIG_OVERLAYS` is defined, its comma separated overlays (e.g. "prod" or "feat, dev") at `<CONFIG_DIR>/<overlay>.yaml` are loaded from left to right as overlays, i.e. adding or overwriting already existing values.

Finally environment variables prefixed with `<CONFIG_ENV_PREFIX>__` and segments separated by `__` (double underscores are used as segment separators to allow for single underscores in segment names) are used as final overlay.

File formats are gated behind features: `yaml` (default) and `toml`.

## Example

```rust
use configured::Configured;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Config {
    foo: Foo,
    qux: Qux,
}

let config = Config::load()?;
```

## License ##

This code is open source software licensed under the [Apache 2.0 License](http://www.apache.org/licenses/LICENSE-2.0.html).
