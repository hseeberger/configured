use anyhow::{Context, Result};
use config::{Config, Environment, File};
use serde::Deserialize;
use std::env;

/// Environment variable for the config directory; default is `"config"`.
pub const CONFIG_DIR: &str = "CONFIG_DIR";

/// Environment variable for the optional environment (e.g. "prod" or "dev") configuration file.
pub const CONFIG_ENVIRONMENT: &str = "CONFIG_ENVIRONMENT";

/// Environment variable for the prefix of environment variable overrides
pub const CONFIG_ENV_PREFIX: &str = "CONIFG_ENV_PREFIX";

pub trait Configured: Sized {
    /// Utility to load configuration from three well defined layers into any type which can be deserialized by [Serde](https://serde.rs/).
    ///
    /// First, values from the mandatory default configuration file at `<CONFIG_DIR>/default.toml`
    /// are loaded.
    ///
    /// Then, if if the environment variable `CONFIG_ENVIRONMENT` is defined, values from the
    /// environment (e.g. "prod") specific configuration file at
    /// `<CONFIG_DIR>/<CONFIG_ENVIRONMENT>.toml` are loaded as an overlay, i.e. adding or
    /// overwriting already existing values.
    ///
    /// Finally environment variables prefixed with `<CONFIG_ENV_PREFIX>_` and separated by `_` are
    /// used as additional overlay.
    fn load() -> Result<Self>;
}

impl<'de, T> Configured for T
where
    T: Deserialize<'de>,
{
    fn load() -> Result<Self> {
        let config_dir = env::var(CONFIG_DIR).unwrap_or_else(|_| "config".to_string());
        let config_env_prefix = env::var(CONFIG_ENV_PREFIX).unwrap_or_else(|_| "app".to_string());
        env::var(CONFIG_ENVIRONMENT)
            .iter()
            .fold(
                Config::builder().add_source(File::with_name(&format!("{config_dir}/default"))),
                |config, env| config.add_source(File::with_name(&format!("{config_dir}/{env}"))),
            )
            .add_source(Environment::with_prefix(&config_env_prefix).separator("_"))
            .build()?
            .try_deserialize()
            .context("Cannot load configuration")
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
        quux_quux: String,
    }

    #[test]
    fn test_load() {
        env::set_var(CONFIG_DIR, "test-config");
        env::set_var(CONFIG_ENVIRONMENT, "dev");
        env::set_var("APP_QUX_QUUX-QUUX", "Quux2");

        let config = Config::load();
        assert!(config.is_ok());
        let config = config.unwrap();

        assert_eq!(config.foo.bar.as_str(), "Bar");
        assert_eq!(config.foo.baz.as_str(), "Baz2");
        assert_eq!(config.qux.quux_quux.as_str(), "Quux2");
    }
}
