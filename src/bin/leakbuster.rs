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
        #[structopt(long, parse(from_os_str))]
        /// Path to the configuration file
        config: Option<PathBuf>,

        #[structopt(long, parse(from_os_str))]
        /// Path to the usage db
        db: Option<PathBuf>,

        /// Id of the app to start, as defined in config
        app_id: String
    },
    /// Evaluate a condition on the usage of a given app.
    /// Exit 0: if the condition is true.
    /// Exit 1: if the condition is false.
    /// Other exit codes indicate that an error occured.
    Eval {
        #[structopt(long, parse(from_os_str))]
        /// Path to the usage db
        db: Option<PathBuf>,

        /// Id of the app that the condition is about.
        app_id: String,

        /// Condition to be evaluated.
        condition: String
    },
    /// Show a window with a countdown to delay program start. Use as a
    /// startup hook, in combination with `leakbuster run`.
    /// Exit 0: If the user lets the countdown elapse.
    /// Exit 1: If the user aborts by pressing ESC or closing the window.
    /// Other exit codes indicate that an error occured.
    Delay {
        /// Duration of the countdown, in seconds
        duration: u64,
        /// Message to display to the user
        message: Option<String>
    }
}

fn main() {
    let leakbuster = Leakbuster::from_args();
    match leakbuster {
        Leakbuster::Run{ config, db, app_id } =>
            run::run(config, db, &app_id),
        Leakbuster::Eval{ db, app_id, condition } =>
            eval::eval(db, &app_id, &condition),
        Leakbuster::Delay{ duration, message } =>
            delay::delay(duration, message)
    }
}
