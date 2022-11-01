use log::{trace,debug,warn,info,error};
use env_logger::Env;
mod settings;
use settings::Settings;
use serde::{Deserialize, Serialize};


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

#[derive(Debug, Deserialize, Serialize)]
struct Cat {
    name: String,
    rate: i32,
}

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
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
    info!("{:?}", settings);

    info!("Start your app.");

    let new_post = Cat {
        name: "ssfdsfdsfds".into(),
        rate: 45,
    };

    let new_post: Cat = reqwest::Client::new()
        //.post("https://jsonplaceholder.typicode.com/posts")
        .post("http://127.0.0.1:8080/submit")
        .json(&new_post)
        .send()
        .await?
        .json()
        .await?;

    info!("{:#?}", new_post);
    // Post {
    //     id: Some(
    //         101
    //     ),
    //     title: "Reqwest.rs",
    //     body: "https://docs.rs/reqwest",
    //     user_id: 1
    // }
    Ok(())
}
