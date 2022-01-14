
use config::{ConfigError, Config};
use actix_web::{error, Error};
use std::collections::HashMap;

pub fn get_config(service: &str) -> Result<Config, ConfigError> {
    let mut config = config::Config::new();

    config.merge(config::File::with_name(
        &*("./config/".to_owned() + &*service + &*".toml".to_owned())))?;
    Ok(config)
}

pub fn build_form_params(config: Config) -> [(&'static str, &'static str); 9] {
    [("redirectionUrl", config.get("redirect_url").unwrap()),
        ("credentialStrength", config.get("credential_strength").unwrap()),
        ("confidenceLevel", config.get("affinity_group").unwrap()),
        ("affinityGroup", config.get("affinity_group").unwrap()),
        ("enrolment[0].name", config.get("enrolement.key").unwrap()),
        ("enrolment[0].taxIdentifier[0].name", config.get("enrolement.identifier0.name").unwrap()),
        ("enrolment[0].taxIdentifier[0].value", config.get("enrolement.identifier0.value").unwrap()),
        ("enrolment[0].taxIdentifier[1].name", config.get("enrolement.identifier1.name").unwrap()),
        ("enrolment[0].taxIdentifier[1].value", config.get("enrolement.identifier1.value").unwrap()),
    ]
}
