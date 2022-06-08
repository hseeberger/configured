# Configured

Load configuration from three layers into any struct which can be deserialized by Serde.

First values from the mandatory default configuration file at `<CONFIG_DIR>/default.toml` are loaded.

Then, if if the environment variable `CONFIG_ENVIRONMENT` is defined, values from the environment (e.g. "prod") specific configuration file at `<CONFIG_DIR>/<CONFIG_ENVIRONMENT>.toml` are loaded as an overlay.

Finally environment variables prefixed with `<CONFIG_ENV_PREFIX>__` and separated by `__` (double underscores are used as separators because of snake_cased keys) are used as additional overlay.
