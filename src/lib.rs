#![cfg_attr(docsrs, feature(doc_cfg))]

//! Opinionated utility to load a configuration from well defined layers into any type which can be
//! deserialized by [Serde](https://serde.rs/), using the [Case] chosen by the caller.
//!
//! First, values from the mandatory default configuration file at `<CONFIG_DIR>/default.yaml/toml`
//! are loaded.
//!
//! Then, if the environment variable `CONFIG_OVERLAYS` is defined, its comma separated overlays
//! (e.g. "prod" or "feat, dev") at `<CONFIG_DIR>/<overlay>.yaml/toml` are loaded from left to right
//! as overlays, i.e. adding or overwriting already existing values.
//!
//! Finally environment variables prefixed with `<CONFIG_ENV_PREFIX>__` and segments separated by
//! `__` (double underscores are used as segment separators to allow for single underscores in
//! segment names) are used as final overlay. Each segment is converted to the [Case] passed to
//! [Configured::load], so it matches the `#[serde(rename_all = ...)]` of the target type.
//!
//! ## Example
//!
//! ```rust
//! use configured::{ CONFIG_DIR, CONFIG_OVERLAYS, Case, Configured, Error };
//! use serde::Deserialize;
//! use std::env;
//!
//! #[derive(Debug, Deserialize)]
//! struct Config {
//!     contact: Contact,
//!     address: Address,
//! }
//!
//! #[derive(Debug, Deserialize)]
//! struct Contact {
//!     first_name: String,
//!     last_name: String,
//! }
//!
//! #[derive(Debug, Deserialize)]
//! struct Address {
//!     street_name: String,
//!     zip_code: String,
//! }
//!
//! fn load_config() -> Result<(), Error> {
//!     unsafe {
//!         env::set_var(CONFIG_DIR, "test-config");
//!         env::set_var(CONFIG_OVERLAYS, "feat, dev");
//!         env::set_var("CFG__CONTACT__FIRST_NAME", "Jane-env");
//!     }
//!
//!     let config = Config::load(Case::Snake)?;
//!
//!     assert_eq!(config.contact.first_name.as_str(), "Jane-env");
//!     assert_eq!(config.contact.last_name.as_str(), "Doe-dev");
//!     assert_eq!(config.address.street_name.as_str(), "Main Street feat");
//!     assert_eq!(config.address.zip_code.as_str(), "12345");
//!
//!     Ok(())
//! }
//! ```

use config::{Config, ConfigError, Environment, File};
use serde::Deserialize;
use std::env;
use thiserror::Error;

pub use config::Case;

/// Environment variable for the configuration directory; default is `"config"`.
pub const CONFIG_DIR: &str = "CONFIG_DIR";

/// Environment variable for the optional overlays (e.g. "prod" or "dev").
pub const CONFIG_OVERLAYS: &str = "CONFIG_OVERLAYS";

/// Environment variable for the prefix of environment variable overrides; default is `"CFG"`.
pub const CONFIG_ENV_PREFIX: &str = "CONFIG_ENV_PREFIX";

/// Use (import) this trait and all types that implement `Deserialize` are extended with the `load`
/// associated function.
pub trait Configured: Sized {
    /// Load this configuration, converting environment variable overlay keys to the given [Case].
    fn load(case: Case) -> Result<Self, Error>;
}

impl<'de, T> Configured for T
where
    T: Deserialize<'de>,
{
    fn load(case: Case) -> Result<Self, Error> {
        let config_dir = env::var(CONFIG_DIR).unwrap_or_else(|_| "config".to_string());
        let config_env_prefix = env::var(CONFIG_ENV_PREFIX).unwrap_or_else(|_| "CFG".to_string());
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
            .add_source(
                Environment::with_prefix(&config_env_prefix)
                    .separator("__")
                    .convert_case(case),
            )
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::{env, sync::Mutex};

    static ENV_LOCK: Mutex<()> = Mutex::new(());

    #[derive(Debug, Deserialize)]
    struct Config {
        contact: Contact,
        address: Address,
    }

    #[derive(Debug, Deserialize)]
    struct Contact {
        first_name: String,
        last_name: String,
        favorite: bool,
    }

    #[derive(Debug, Deserialize)]
    struct Address {
        street_name: String,
        street_number: u16,
        zip_code: String,
    }

    #[derive(Debug, Deserialize)]
    #[serde(rename_all = "kebab-case")]
    struct KebabConfig {
        contact: KebabContact,
        address: KebabAddress,
    }

    #[derive(Debug, Deserialize)]
    #[serde(rename_all = "kebab-case")]
    struct KebabContact {
        first_name: String,
        last_name: String,
        favorite: bool,
    }

    #[derive(Debug, Deserialize)]
    #[serde(rename_all = "kebab-case")]
    struct KebabAddress {
        street_name: String,
        street_number: u16,
        zip_code: String,
    }

    #[test]
    fn test_load_snake() -> Result<(), Error> {
        let _guard = ENV_LOCK.lock().unwrap();

        unsafe {
            env::set_var("CONFIG_DIR", "test-config");
            env::set_var("CONFIG_OVERLAYS", "feat, dev");
            env::set_var("CFG__CONTACT__FIRST_NAME", "Jane-env");
        }

        let config = Config::load(Case::Snake)?;

        assert_eq!(config.contact.first_name.as_str(), "Jane-env");
        assert_eq!(config.contact.last_name.as_str(), "Doe-dev");
        assert!(config.contact.favorite);
        assert_eq!(config.address.street_name.as_str(), "Main Street feat");
        assert_eq!(config.address.street_number, 42);
        assert_eq!(config.address.zip_code.as_str(), "12345");

        Ok(())
    }

    #[test]
    fn test_load_kebab() -> Result<(), Error> {
        let _guard = ENV_LOCK.lock().unwrap();

        unsafe {
            env::set_var("CONFIG_DIR", "test-config-kebab");
            env::remove_var("CONFIG_OVERLAYS");
            env::set_var("CFG__CONTACT__FIRST_NAME", "Jane-env");
        }

        let config = KebabConfig::load(Case::Kebab)?;

        assert_eq!(config.contact.first_name.as_str(), "Jane-env");
        assert_eq!(config.contact.last_name.as_str(), "Doe");
        assert!(config.contact.favorite);
        assert_eq!(config.address.street_name.as_str(), "Main Street");
        assert_eq!(config.address.street_number, 42);
        assert_eq!(config.address.zip_code.as_str(), "12345");

        Ok(())
    }

    #[test]
    fn test_load_env_non_string() -> Result<(), Error> {
        let _guard = ENV_LOCK.lock().unwrap();

        unsafe {
            env::set_var("CONFIG_DIR", "test-config");
            env::remove_var("CONFIG_OVERLAYS");
            env::set_var("CFG__CONTACT__FAVORITE", "false");
            env::set_var("CFG__ADDRESS__STREET_NUMBER", "7");
        }

        let config = Config::load(Case::Snake);

        unsafe {
            env::remove_var("CFG__CONTACT__FAVORITE");
            env::remove_var("CFG__ADDRESS__STREET_NUMBER");
        }

        let config = config?;

        assert!(!config.contact.favorite);
        assert_eq!(config.address.street_number, 7);

        Ok(())
    }
}
