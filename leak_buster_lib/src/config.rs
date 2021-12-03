use serde::Deserialize;
use std::fs;
use std::path::Path;
use std::result;

#[derive(Deserialize, PartialEq, Debug)]
pub struct Config {
    pub apps: Vec<App>
}

impl Config {
    pub fn load<P: AsRef<Path>>(config_path: P) -> Result<Config> {
        let config_str = fs::read_to_string(config_path)?;
        let config = serde_yaml::from_str(&config_str)?;
        Ok(config)
    }
}

#[derive(Deserialize, PartialEq, Debug)]
pub struct App {
    pub id: String,
    pub cmd: String,
    pub args: Vec<String>,
    pub startup_hooks: Vec<StartupHook>,
    pub time_hooks: Vec<TimeHook>,
}

#[derive(Deserialize, PartialEq, Debug)]
pub struct StartupHook {
    pub cmd: String,
    pub args: Vec<String>
}

#[derive(Deserialize, PartialEq, Debug)]
pub struct TimeHook {
    pub cmd: String,
    pub args: Vec<String>,
    pub condition_cmd: String,
    pub condition_args: Vec<String>
}

#[derive(Debug)]
pub enum Error {
    IoError(std::io::Error),
    DeserializationError(serde_yaml::Error)
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Error {
        Error::IoError(err)
    }
}

impl From<serde_yaml::Error> for Error {
    fn from(err: serde_yaml::Error) -> Error {
        Error::DeserializationError(err)
    }
}

pub type Result<T, E = Error> = result::Result<T, E>;

mod test {
    use tempfile::NamedTempFile;
    use std::fs;
    use crate::config::{App, Config, Result, StartupHook, TimeHook};

    #[test]
    fn load_minimal_config() {
        let cfg = config_from("apps: []")
            .expect("Error while loading config");
        assert_eq!(Config { apps: vec![] }, cfg);
    }

    #[test]
    fn load_fully_featured_config() {
        let cfg = config_from(r#"
apps:
    - id: app1
      cmd: echo
      args: ["-n", "Hello world"]
      startup_hooks:
        - cmd: touch
          args: ["startup_marker"]
      time_hooks:
        - cmd: killall
          args: ["firefox"]
          condition_cmd: sh
          condition_args: ["-c", "exit 1"]
"#).expect("Error while loading config");
        let expected = Config {
            apps: vec![
                App {
                    id: "app1".to_string(),
                    cmd: "echo".to_string(),
                    args: vec!["-n".to_string(), "Hello world".to_string()],
                    startup_hooks: vec![
                        StartupHook {
                            cmd: "touch".to_string(),
                            args: vec!["startup_marker".to_string()],
                        }
                    ],
                    time_hooks: vec![
                        TimeHook {
                            cmd: "killall".to_string(),
                            args: vec!["firefox".to_string()],
                            condition_cmd: "sh".to_string(),
                            condition_args: vec![
                                "-c".to_string(),
                                "exit 1".to_string()
                            ]
                        }
                    ]
                }
            ]
        };
        assert_eq!(expected, cfg);
    }

    fn config_from(config_str: &str) -> Result<Config> {
        let f = NamedTempFile::new().unwrap();
        fs::write(f.path(), config_str).unwrap();
        Config::load(f.path())
    }

}
