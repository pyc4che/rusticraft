mod app;
mod task;
mod database;

use std::fs::File;
use std::io::Read;

use rusqlite::Result;

use serde::{
    Deserialize, 
    Serialize
};

use app::Todo;


#[derive(Deserialize, Serialize, Debug)]
pub struct AppConfig
{
    database_path: String
}


fn main() -> Result<()>{
    let mut config_file = File::open(
        "path/to/config.json"
    ).expect("Failed to open config file.");
    let mut config_contents = String::new();

    config_file
        .read_to_string(&mut config_contents)
        .expect("Failed to read config file.");

    let config: AppConfig = serde_json::from_str(
        &config_contents
    ).expect("Failed to deserialize config JSON");

    let mut app = Todo::new(
        config.database_path
    )?;

    app.run()?;

    Ok(())
}
