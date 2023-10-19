//! Opinionated utility to load a configuration from well defined layers into any type which can be
//! deserialized by [Serde](https://serde.rs/) using kebab-case.
//!
//! First, values from the mandatory default configuration file at `<CONFIG_DIR>/default.yaml` are
//! loaded.
//!
//! Then, if the environment variable `CONFIG_OVERLAYS` is defined, its comma separated overlays
//! (e.g. "prod" or "feat, dev") at `<CONFIG_DIR>/<overlay>.yaml` are loaded from left to right as
//! overlays, i.e. adding or overwriting already existing values.
//!
//! Finally environment variables prefixed with `<CONFIG_ENV_PREFIX>__` and segments separated by
//! `__` (double underscores are used as segment separators to allow for single underscores in
//! segment names) are used as final overlay.
//!
//! ## Example
//!
//! ```rust
//! use configured::Configured;
//!
//! #[derive(Debug, Deserialize)]
//! #[serde(rename_all = "kebab-case")]
//! struct Config {
//!     foo: Foo,
//!     qux: Qux,
//! }
//!
//! #[derive(Debug, Deserialize)]
//! #[serde(rename_all = "kebab-case")]
//! struct Foo {
//!     bar: String,
//!     baz: String,
//! }
//!
//! #[derive(Debug, Deserialize)]
//! #[serde(rename_all = "kebab-case")]
//! struct Qux {
//!     quux: String,
//!     corge_grault: String,
//! }
//!
//! fn test_load() -> Result<(), Error> {
//!     env::set_var(CONFIG_DIR, "test-config");
//!     env::set_var(CONFIG_OVERLAYS, "feat, dev");
//!     env::set_var("APP__QUX__CORGE_GRAULT", "corge-grault-env");
//!
//!     let config = Config::load()?;
//!
//!     assert_eq!(config.foo.bar.as_str(), "bar");
//!     assert_eq!(config.foo.baz.as_str(), "baz-dev");
//!     assert_eq!(config.qux.quux.as_str(), "quux-feat");
//!     assert_eq!(config.qux.corge_grault.as_str(), "corge-grault-env");
//!
//!     Ok(())
//! }
//! ```

use config::{Config, ConfigError, Environment, File, Map, Source, Value};
use serde::Deserialize;
use std::env;
use thiserror::Error;

/// Environment variable for the configuration directory; default is `"config"`.
pub const CONFIG_DIR: &str = "CONFIG_DIR";

/// Environment variable for the optional overlays (e.g. "prod" or "dev").
pub const CONFIG_OVERLAYS: &str = "CONFIG_OVERLAYS";

/// Environment variable for the prefix of environment variable overrides.
pub const CONFIG_ENV_PREFIX: &str = "CONIFG_ENV_PREFIX";

/// Use (import) this trait and all types that implement `Deserialize` are extended with the `load`
/// associated function.
pub trait Configured: Sized {
    /// Load this configuration.
    fn load() -> Result<Self, Error>;
}

impl<'de, T> Configured for T
where
    T: Deserialize<'de>,
{
    fn load() -> Result<Self, Error> {
        let config_dir = env::var(CONFIG_DIR).unwrap_or_else(|_| "config".to_string());
        let config_env_prefix = env::var(CONFIG_ENV_PREFIX).unwrap_or_else(|_| "APP".to_string());
        env::var(CONFIG_OVERLAYS)
            .into_iter()
            .flat_map(|overlays| {
                overlays
                    .split(',')
                    .map(|s| s.trim().to_owned())
                    .collect::<Vec<_>>()
            })
            .fold(
                Config::builder().add_source(File::with_name(&format!("{config_dir}/default"))),
                |config, overlay| {
                    config.add_source(File::with_name(&format!("{config_dir}/{overlay}")))
                },
            )
            .add_source(KebabEnvironment(
                Environment::with_prefix(&config_env_prefix).separator("__"),
            ))
            .build()
            .map_err(Error::Load)?
            .try_deserialize()
            .map_err(Error::Deserialize)
    }
}

/// Possible errors when loading the configuration.
#[derive(Debug, Error)]
pub enum Error {
    /// Cannot load the configuration, e.g. because file not found.
    #[error("cannot load configuration")]
    Load(#[source] ConfigError),

    /// Cannot deserialzie the configuration, e.g. because fields are missing.
    #[error("cannot deserialize configuration")]
    Deserialize(#[source] ConfigError),
}

#[derive(Debug, Clone)]
struct KebabEnvironment<S>(S);

impl<S> Source for KebabEnvironment<S>
where
    S: Source + Send + Sync + Clone + 'static,
{
    fn clone_into_box(&self) -> Box<dyn Source + Send + Sync> {
        Box::new((*self).clone())
    }

    fn collect(&self) -> Result<Map<String, Value>, ConfigError> {
        let collected = self.0.collect();
        collected.map(|collected| {
            collected
                .into_iter()
                .map(|(k, v)| (k.replace('_', "-"), v))
                .collect()
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

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
}
