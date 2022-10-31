use config::{Config, ConfigError, Environment, File};
use log::info;
use serde_derive::Deserialize;
use std::env;

use std::collections::HashMap;
use std::path::Path;

#[derive(Debug, Deserialize)]
#[allow(unused)]
struct Database {
    url: String,
}

#[derive(Debug, Deserialize)]
#[allow(unused)]
struct Sparkpost {
    key: String,
    token: String,
    url: String,
    version: u8,
}

#[derive(Debug, Deserialize)]
#[allow(unused)]
struct Twitter {
    consumer_token: String,
    consumer_secret: String,
}

#[derive(Debug, Deserialize)]
#[allow(unused)]
struct Braintree {
    merchant_id: String,
    public_key: String,
    private_key: String,
}

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct Settings {
    debug: bool,
    database: Database,
    sparkpost: Sparkpost,
    twitter: Twitter,
    braintree: Braintree,
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let run_mode = env::var("RUN_MODE").unwrap_or_else(|_| "development".into());

        let s = Config::builder()
            // Start off by merging in the "default" configuration file
            .add_source(File::with_name("examples/config/default"))
            // Add in the current environment file
            // Default to 'development' env
            // Note that this file is _optional_
            .add_source(File::with_name(&format!("examples/config/{}", run_mode)).required(false))
            // Add in a local configuration file
            // This file shouldn't be checked in to git
            .add_source(File::with_name("examples/config/local").required(false))
            // Add in settings from the environment (with a prefix of APP)
            // Eg.. `APP_DEBUG=1 ./target/app` would set the `debug` key
            .add_source(Environment::with_prefix("app"))
            // You may also programmatically change settings
            .set_override("database.url", "postgres://")?
            .build()?;

        // Option 1
        // --------
        // Gather all conf files from conf/ manually
        let settings2 = Config::builder()
            // File::with_name(..) is shorthand for File::from(Path::new(..))
            .add_source(File::with_name("examples/config/00-default.toml"))
            .add_source(File::from(Path::new("examples/config/05-some.yml")))
            .add_source(File::from(Path::new("examples/config/99-extra.json")))
            .build()
            .unwrap();

        // Print out our settings (as a HashMap)
        info!(
            "\n{:?} \n\n-----------",
            settings2
                .try_deserialize::<HashMap<String, String>>()
                .unwrap()
        );

        // Now that we're done, let's access our configuration
        info!("debug: {:?}", s.get_bool("debug"));
        info!("database: {:?}", s.get::<String>("database.url"));

        // You can deserialize (and thus freeze) the entire configuration as
        s.try_deserialize()
    }
}
