// Prevent console window in addition to Slint window in Windows release builds when, e.g., starting the app via file manager. Ignored on other platforms.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{error, fs, io, sync::LazyLock};

use async_compat::Compat;
use directories::ProjectDirs;
use md5::{Digest, Md5};
use progenitor::generate_api;
use rand::{distributions::Alphanumeric, Rng};
use serde::Deserialize;

slint::include_modules!();
generate_api!(spec = "openapi.json", pre_hook_async = crate::add_auth);

#[derive(Deserialize, Debug)]
struct Credentials {
    ip: String,
    port: Option<u16>,
    username: String,
    password: String,
}

impl Credentials {
    fn baseurl(&self) -> String {
        self.ip.to_owned()
            + &self
                .port
                .map_or_else(|| "".to_string(), |v| format!(":{v}"))
            + "/rest"
    }
}

fn get_credentials() -> Result<Credentials, Box<dyn error::Error>> {
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

static OPENSUBSONIC_VERSION: &str = "1.13.0";
static OPENSUBSONIC_CLIENT_NAME: &str = "funkyduck";
static OPENSUBSONIC_FORMAT: &str = "json";
static SALT_LENGTH: usize = 20;
static CREDENTIALS: LazyLock<Credentials> = LazyLock::new(|| get_credentials().unwrap());
static OPENSUBSONIC_AUTH_ARGS: LazyLock<Vec<(&str, String)>> = LazyLock::new(|| {
    let salt = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(SALT_LENGTH)
        .map(char::from)
        .collect();

    let hash = Md5::new()
        .chain_update(&CREDENTIALS.password)
        .chain_update(&salt)
        .finalize();

    Vec::from([
        ("u", CREDENTIALS.username.to_owned()),
        ("s", salt),
        ("t", format!("{:x}", hash)),
        ("v", OPENSUBSONIC_VERSION.to_owned()),
        ("c", OPENSUBSONIC_CLIENT_NAME.to_owned()),
        ("f", OPENSUBSONIC_FORMAT.to_owned()),
    ])
});

async fn add_auth(req: &mut reqwest::Request) -> Result<(), reqwest::header::InvalidHeaderValue> {
    req.url_mut()
        .query_pairs_mut()
        .extend_pairs(OPENSUBSONIC_AUTH_ARGS.iter());

    Ok(())
}

fn main() -> Result<(), Box<dyn error::Error>> {
    let client = Client::new(&CREDENTIALS.baseurl());

    let slint_future = async move {
        client.get_artists(Option::None).await.unwrap();
    };
    slint::spawn_local(Compat::new(slint_future)).unwrap();

    let ui = AppWindow::new()?;

    ui.run()?;

    Ok(())
}
