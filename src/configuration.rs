use config::{Config, Environment, File};
use serde_derive::{Deserialize, Serialize};
use std::env;
use tracing_subscriber::fmt::format;

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct ApplicationSettings {
    pub application_port: u16,
    pub application_host: String,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct Settings {
    pub database: DbSettings,
    pub application: ApplicationSettings,
}

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct DbSettings {
    pub username: String,
    pub password: String,
    pub port: u16,
    pub host: String,
    pub database_name: String,
}

pub fn get_config() -> Result<Settings, config::ConfigError> {
    Settings::new()
}

impl Settings {
    pub fn new() -> Result<Self, config::ConfigError> {
        let run_mode = env::var("RUN_MODE").unwrap_or_else(|_| "dev".into());
        print!("RUNMODE {}", run_mode);

        let s = Config::builder()
            // Start off by merging in the "default" configuration file
            .add_source(File::with_name("config/settings.toml"))
            // Add in the current environment file
            // Default to 'development' env
            // Note that this file is _optional_
            .add_source(
                File::with_name(&format!("config/settings-{}.toml", run_mode)).required(false),
            )
            // Add in a local configuration file
            // This file shouldn't be checked in to git
            .add_source(File::with_name("config/settings-local.toml").required(false))
            // Add in settings from the environment (with a prefix of APP)
            // Eg.. `APP_DEBUG=1 ./target/app` would set the `debug` key
            .add_source(Environment::with_prefix("app"))
            // You may also programmatically change settings
            //.set_override("database.url", "postgres://")?
            .build()?;

        // Now that we're done, let's access our configuration
        //println!("debug: {:?}", s.get_bool("debug"));
        //println!("application_port: {:?}", s.get::<String>("application_port"));
        // println!("all: {:?}", s);

        // You can deserialize (and thus freeze) the entire configuration as
        s.try_deserialize()
    }
}

impl DbSettings {
    pub fn connection_string(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}/{}",
            self.username, self.password, self.host, self.port, self.database_name
        )
    }
    pub fn connection_string_without_db(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}",
            self.username, self.password, self.host, self.port
        )
    }
}
