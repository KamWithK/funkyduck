// Prevent console window in addition to Slint window in Windows release builds when, e.g., starting the app via file manager. Ignored on other platforms.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{error::Error, fs, io};

use directories::ProjectDirs;
use serde::Deserialize;

slint::include_modules!();

#[derive(Deserialize)]
struct Credentials {
    ip: String,
    port: Option<u16>,
    username: String,
    password: String,
}

fn get_credentials() -> Result<Credentials, Box<dyn Error>> {
    let project_dirs = ProjectDirs::from("com", "kamwithk", "funkyduck")
        .ok_or(io::Error::other("Couldn't get project directories"))?;
    let config_dir = ProjectDirs::config_dir(&project_dirs);
    let credentials_file = config_dir.join("credentials.toml");
    let parsed_credentials = toml::from_str(&fs::read_to_string(credentials_file)?);
    match parsed_credentials {
        Ok(credentials) => Ok(credentials),
        Err(e) => Err(Box::new(e)),
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let credentials = get_credentials()?;

    let ui = AppWindow::new()?;

    ui.run()?;

    Ok(())
}
