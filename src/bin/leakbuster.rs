use leakbuster::cmd::run;
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
    }
}

fn main() {
    let leakbuster = Leakbuster::from_args();
    match leakbuster {
        Leakbuster::Run{config, db, app_id} => run::run(config, db, &app_id)
    }
}
