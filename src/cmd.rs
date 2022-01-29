pub mod delay;
pub mod eval;
pub mod run;

use crate::config;
use crate::config::Config;
use crate::db;
use crate::db::Db;

use home;
use std::io::ErrorKind;
use std::path::PathBuf;

/// Attempts to load configuration either from a default location, or from a
/// manual parameter. Will panic on failure and display a (hopefully) helpful
/// error message to the user.
fn get_config(manual: Option<PathBuf>) -> Config {
    let path = manual.unwrap_or_else(default_config_path);
    let config = Config::load(&path);
    match config {
        Err(cause) => {
            match cause {
                config::Error::IoError(io_error_cause) => match io_error_cause.kind() {
                    ErrorKind::NotFound => {
                        eprintln!("Config file not found. You need to provide a config file in order for `leakbuster run` to know what to run.");
                    },
                    other => {
                        eprintln!("Could not load config file due to Os error: {:#?}", other);
                    }
                },
                config::Error::DeserializationError(cause) => {
                    panic!("Config file is malformed: {:#?}", cause);
                }
            }
            eprintln!(
                "Could not load config file at location: {}",
                path.to_str().expect("The path you have specified contains invalid UTF-8 *and* points to a file that doesn't exist. I'm afraid you're on your own.")
            );
            panic!("Could not load config file!");
        },
        Ok(config) => config
    }
}

fn get_db(manual: Option<PathBuf>) -> Db {
    let path = manual.unwrap_or_else(default_db_path);
    db::connect_sqlite(path).expect("Could not load db!")
}

fn default_config_path() -> PathBuf {
    // ~/.config/leakbuster
    let mut path = home::home_dir().expect("Could not get home dir. Please specify config location manually");
    path.push(".config");
    path.push("leakbuster.config");
    path
}

fn default_db_path() -> PathBuf {
    // ~/.leakbuster.db
    let mut path = home::home_dir().expect("Could not get home dir. Please specify db location manually");
    path.push(".leakbuster.db");
    path
}
