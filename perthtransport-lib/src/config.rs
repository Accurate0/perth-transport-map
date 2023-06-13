use crate::{constants::CONFIG_PREFIX, types::config::ApplicationConfig};
use config::{Config, Environment};

pub fn get_application_config() -> Result<ApplicationConfig, anyhow::Error> {
    Ok(Config::builder()
        .add_source(Environment::with_prefix(CONFIG_PREFIX))
        .build()?
        .try_deserialize::<ApplicationConfig>()?)
}
