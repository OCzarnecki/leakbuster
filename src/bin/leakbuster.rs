use leakbuster::cmd::{delay, eval, run};
use structopt::StructOpt;
use std::path::PathBuf;

#[derive(StructOpt)]
/// Leakbuster is a tool for monitoring the time for which applications are
/// running, and for triggering configurable events once the usage time
/// meets some condition.
enum Leakbuster {
    /// Run an app, defined in config, and trigger startup and time hooks
    Run {
        #[structopt(parse(from_os_str))]
        /// Path to the configuration file
        config: PathBuf,

        #[structopt(parse(from_os_str))]
        /// Path to the usage db
        db: PathBuf,

        /// Id of the app to start, as defined in config
        app_id: String
    },
    /// Evaluate a condition on the usage of a given app.
    /// Exit 0: if the condition is true.
    /// Exit 1: if the condition is false.
    /// All other exit codes mean that there was an error.
    Eval {
        #[structopt(parse(from_os_str))]
        /// Path to the usage db
        db: PathBuf,

        /// Id of the app that the condition is about.
        app_id: String,

        /// Condition to be evaluated.
        condition: String
    },
    Delay {
        duration: u64,
        message: Option<String>
    }
}

fn main() {
    let leakbuster = Leakbuster::from_args();
    match leakbuster {
        Leakbuster::Run{ config, db, app_id } =>
            run::run(&config, &db, &app_id),
        Leakbuster::Eval{ db, app_id, condition } =>
            eval::eval(&db, &app_id, &condition),
        Leakbuster::Delay{ duration, message } =>
            delay::delay(duration, message)
    }
}
