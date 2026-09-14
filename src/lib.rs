#![cfg_attr(docsrs, feature(doc_cfg))]

//! Opinionated utility to load a configuration from well defined layers into any type which can be
//! deserialized by [Serde](https://serde.rs/), using the [LoadOptions] given by the caller.
//!
//! First, values from the mandatory default configuration file at `<config_dir>/default.yaml/toml`
//! are loaded.
//!
//! Then the overlays (e.g. "prod" or "feat" and "dev") at `<config_dir>/<overlay>.yaml/toml` are
//! loaded from left to right as overlays, i.e. adding or overwriting already existing values.
//!
//! Finally environment variables prefixed with `<config_env_prefix>__` and segments separated by
//! `__` (double underscores are used as segment separators to allow for single underscores in
//! segment names) are used as final overlay. Each segment is converted to the [Case] of the
//! [LoadOptions], so it matches the `#[serde(rename_all = ...)]` of the target type.
//!
//! The configuration directory, the overlays and the environment variable prefix can each be set
//! on the [LoadOptions]. A value which is not set there is taken from the respective environment
//! variable ([CONFIG_DIR], [CONFIG_OVERLAYS] or [CONFIG_ENV_PREFIX]) and, if that one is not
//! defined either, from the fallback: `"config"` for the directory, no overlays and `"CFG"` for
//! the prefix. Setting an empty list of overlays means no overlays, even if [CONFIG_OVERLAYS] is
//! defined.
//!
//! ## Example
//!
//! ```rust
//! use configured::{ Configured, Error, LoadOptions };
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
//!         env::set_var("CFG__CONTACT__FIRST_NAME", "Jane-env");
//!     }
//!
//!     let options = LoadOptions::default()
//!         .config_dir("test-config")
//!         .config_overlays(["feat", "dev"]);
//!     let config = Config::load(options)?;
//!
//!     assert_eq!(config.contact.first_name.as_str(), "Jane-env");
//!     assert_eq!(config.contact.last_name.as_str(), "Doe-dev");
//!     assert_eq!(config.address.street_name.as_str(), "Main Street feat");
//!     assert_eq!(config.address.zip_code.as_str(), "12345");
//!
//!     Ok(())
//! }
//! # load_config().unwrap();
//! ```

#![warn(missing_docs)]

use config::{Config, ConfigError, Environment, File};
use serde::Deserialize;
use std::{
    env,
    path::{Path, PathBuf},
};
use thiserror::Error;

pub use config::Case;

/// Environment variable for the configuration directory, used as fallback if
/// [LoadOptions::config_dir] was not called; if it is not defined either, `"config"` is used.
pub const CONFIG_DIR: &str = "CONFIG_DIR";

/// Environment variable for the optional comma separated overlays (e.g. "prod" or "feat, dev"),
/// used as fallback if [LoadOptions::config_overlays] was not called; if it is not defined either,
/// no overlays are used.
pub const CONFIG_OVERLAYS: &str = "CONFIG_OVERLAYS";

/// Environment variable for the prefix of environment variable overrides, used as fallback if
/// [LoadOptions::config_env_prefix] was not called; if it is not defined either, `"CFG"` is used.
pub const CONFIG_ENV_PREFIX: &str = "CONFIG_ENV_PREFIX";

/// Use (import) this trait and all types that implement `Deserialize` are extended with the `load`
/// associated function.
pub trait Configured: Sized {
    /// Load this configuration as described by the given [LoadOptions].
    fn load(options: LoadOptions) -> Result<Self, Error>;
}

impl<'de, T> Configured for T
where
    T: Deserialize<'de>,
{
    fn load(options: LoadOptions) -> Result<Self, Error> {
        let LoadOptions {
            case,
            config_dir,
            config_overlays,
            config_env_prefix,
        } = options;

        let config_dir = config_dir
            .or_else(|| env::var_os(CONFIG_DIR).map(PathBuf::from))
            .unwrap_or_else(|| PathBuf::from("config"));

        let config_overlays = config_overlays.unwrap_or_else(|| {
            env::var(CONFIG_OVERLAYS)
                .into_iter()
                .flat_map(|overlays| {
                    overlays
                        .split(',')
                        .map(|overlay| overlay.trim().to_owned())
                        .collect::<Vec<_>>()
                })
                .collect()
        });

        let config_env_prefix = config_env_prefix
            .unwrap_or_else(|| env::var(CONFIG_ENV_PREFIX).unwrap_or_else(|_| "CFG".to_string()));

        config_overlays
            .into_iter()
            .fold(
                Config::builder().add_source(File::from(config_dir.join("default"))),
                |config, overlay| config.add_source(File::from(config_dir.join(overlay))),
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

/// Options for [Configured::load].
///
/// By default the [Case] is [Case::Snake] and none of the other values is set, i.e. each of them is
/// taken from the respective environment variable or, if that one is not defined, from the
/// fallback.
#[derive(Debug, Clone)]
pub struct LoadOptions {
    case: Case,
    config_dir: Option<PathBuf>,
    config_overlays: Option<Vec<String>>,
    config_env_prefix: Option<String>,
}

impl Default for LoadOptions {
    fn default() -> Self {
        Self {
            case: Case::Snake,
            config_dir: None,
            config_overlays: None,
            config_env_prefix: None,
        }
    }
}

impl LoadOptions {
    /// Set the [Case] the segments of the environment variable overrides are converted to.
    pub fn case(self, case: Case) -> Self {
        Self { case, ..self }
    }

    /// Set the configuration directory, taking precedence over the [CONFIG_DIR] environment
    /// variable.
    pub fn config_dir<P>(self, config_dir: P) -> Self
    where
        P: AsRef<Path>,
    {
        Self {
            config_dir: Some(config_dir.as_ref().to_path_buf()),
            ..self
        }
    }

    /// Set the overlays, taking precedence over the [CONFIG_OVERLAYS] environment variable; an
    /// empty list means no overlays.
    pub fn config_overlays<I, S>(self, config_overlays: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        Self {
            config_overlays: Some(config_overlays.into_iter().map(Into::into).collect()),
            ..self
        }
    }

    /// Set the prefix of the environment variable overrides, taking precedence over the
    /// [CONFIG_ENV_PREFIX] environment variable.
    pub fn config_env_prefix<S>(self, config_env_prefix: S) -> Self
    where
        S: Into<String>,
    {
        Self {
            config_env_prefix: Some(config_env_prefix.into()),
            ..self
        }
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

        let config = Config::load(LoadOptions::default())?;

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

        let config = KebabConfig::load(LoadOptions::default().case(Case::Kebab))?;

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

        let config = Config::load(LoadOptions::default());

        unsafe {
            env::remove_var("CFG__CONTACT__FAVORITE");
            env::remove_var("CFG__ADDRESS__STREET_NUMBER");
        }

        let config = config?;

        assert!(!config.contact.favorite);
        assert_eq!(config.address.street_number, 7);

        Ok(())
    }

    #[test]
    fn test_load_explicit_config_dir() -> Result<(), Error> {
        let _guard = ENV_LOCK.lock().unwrap();

        unsafe {
            env::remove_var("CONFIG_DIR");
            env::remove_var("CONFIG_OVERLAYS");
            env::set_var("CFG__CONTACT__FIRST_NAME", "Jane-env");
        }

        let options = LoadOptions::default()
            .config_dir("test-config")
            .config_overlays(["feat", "dev"]);
        let config = Config::load(options)?;

        assert_eq!(config.contact.first_name.as_str(), "Jane-env");
        assert_eq!(config.contact.last_name.as_str(), "Doe-dev");
        assert!(config.contact.favorite);
        assert_eq!(config.address.street_name.as_str(), "Main Street feat");
        assert_eq!(config.address.street_number, 42);
        assert_eq!(config.address.zip_code.as_str(), "12345");

        Ok(())
    }

    #[test]
    fn test_load_options_beat_env() -> Result<(), Error> {
        let _guard = ENV_LOCK.lock().unwrap();

        unsafe {
            env::set_var("CONFIG_DIR", "does-not-exist");
            env::set_var("CONFIG_OVERLAYS", "feat");
            env::set_var("CONFIG_ENV_PREFIX", "OTHER");
            env::set_var("OTHER__CONTACT__FIRST_NAME", "Jane-other");
            env::set_var("MINE__CONTACT__FIRST_NAME", "Jane-mine");
        }

        let options = LoadOptions::default()
            .config_dir("test-config")
            .config_overlays(["dev"])
            .config_env_prefix("MINE");
        let config = Config::load(options);

        unsafe {
            env::remove_var("CONFIG_ENV_PREFIX");
            env::remove_var("OTHER__CONTACT__FIRST_NAME");
            env::remove_var("MINE__CONTACT__FIRST_NAME");
        }

        let config = config?;

        assert_eq!(config.contact.first_name.as_str(), "Jane-mine");
        assert_eq!(config.contact.last_name.as_str(), "Doe-dev");
        assert_eq!(config.address.street_name.as_str(), "Main Street");

        Ok(())
    }

    #[test]
    fn test_load_empty_overlays_ignores_env() -> Result<(), Error> {
        let _guard = ENV_LOCK.lock().unwrap();

        unsafe {
            env::set_var("CONFIG_DIR", "test-config");
            env::set_var("CONFIG_OVERLAYS", "feat, dev");
            env::set_var("CFG__CONTACT__FIRST_NAME", "Jane-env");
        }

        let config = Config::load(LoadOptions::default().config_overlays(Vec::<String>::new()))?;

        assert_eq!(config.contact.last_name.as_str(), "Doe");
        assert_eq!(config.address.street_name.as_str(), "Main Street");

        Ok(())
    }
}
