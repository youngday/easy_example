#[macro_use]
extern crate log;

mod settings;

use settings::Settings;


use env_logger::Env;

use std::fs::File;
use std::io::Read;

use serde::{Deserialize, Serialize};

//#![warn(rust_2018_idioms)]
use tokio::io::AsyncWriteExt;
use tokio::net::TcpStream;

use std::error::Error;




#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct Application {
    application: Data,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct Data {
    build: String,
    container_name: String,
    environment2: Data2,
    #[serde(skip_serializing_if = "Option::is_none")]
    environment: Option<Vec<String>>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct Data2 {
    one_env2: String,
    sec_env2: String,
}

#[tokio::main]
 pub async fn main()-> Result<(), Box<dyn Error>> {
    let env = Env::default()
        .filter_or("MY_LOG_LEVEL", "debug")
        .write_style_or("MY_LOG_STYLE", "always");

    env_logger::init_from_env(env);

    trace!("some trace log");
    debug!("some debug log");
    info!("some information log");
    warn!("some warning log");
    error!("some error log");










    let settings = Settings::new();

    // Print out our settings
    println!("{:?}", settings);



    let filename = "config.yaml";
    match File::open(filename) {
        Ok(mut file) => {
            let mut content = String::new();
            file.read_to_string(&mut content).unwrap();

            let application_data: Application = serde_yaml::from_str(&content).unwrap();
            info!("{:?}", application_data.application.build);
            info!("{:?}", application_data.application.environment);
            //info!("{:?}", application_data.application.environment2);
            //info!("{:?}", application_data.application.environment2.one_env2);
        }
        Err(error) => {
            info!("There is an error {}: {}", filename, error);
        }
    }

    info!("Start your app.");

     // Open a TCP stream to the socket address.
    //
    // Note that this is the Tokio TcpStream, which is fully async.
    let mut stream = TcpStream::connect("127.0.0.1:6142").await?;
    println!("created stream");

    let result = stream.write(b"hello world\n").await;
    println!("wrote to stream; success={:?}", result.is_ok());


    Ok(())
 
}

