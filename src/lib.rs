

use config::{ConfigError, Config};
use actix_web::{error, Error};

pub fn get_config(service: &str) -> Result<Config, ConfigError> {
    let mut config = config::Config::new();

    config.merge(config::File::with_name(
        &*("./config/".to_owned() + &*service + &*".toml".to_owned())))?;
    Ok(config)
}