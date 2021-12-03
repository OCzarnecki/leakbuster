use std::process::Command;
use std::time::SystemTime;
use std::{thread, time};

use crate::config::{App, Config, StartupHook, TimeHook};
use crate::db::Db;

use std::path::Path;

pub fn run<P: AsRef<Path>>(config_path: P, db_path: P, app_id: &str) {
    let config = Config::load(config_path).expect("Couldn't load config");
    let db = Db::connect(db_path).expect("Couldn't connect to db");
    let app = &config.get_app(app_id)
        .expect(&format!("Unknown app: {:}", app_id));
    check_startup_hooks(&app).expect("Startup hook prevented run");
    run_app(&app, &db);
}

fn check_startup_hooks<'a>(app: &'a App) -> Result<(), &'a StartupHook> {
    for hook in &app.startup_hooks {
        let status = Command::new(&hook.cmd)
            .args(&hook.args)
            .status()
            .expect(&format!("Failed to run startup hook: {:?}", hook));
        if !status.success() {
            return Err(&hook)
        }
    }
    Ok(())
}

fn run_app(app: &App, db: &Db) {
    let mut command = Command::new(&app.cmd)
        .args(&app.args)
        .spawn()
        .expect("Failed to execute command");

    let delay = time::Duration::from_secs(1);
    while command.try_wait().unwrap().is_none() {
        thread::sleep(delay);
        let timestamp = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .map(|d| d.as_secs());
        match timestamp {
            Ok(ts) => db.record_usage(&app.id, ts, delay.as_secs())
                .unwrap_or_else(
                    |e| println!("Failed to record usage: {:?}", e)
                ),
            Err(e) => println!("Could not obtain timestamp: {:?}", e)
        };
    }
}

