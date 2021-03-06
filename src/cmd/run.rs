use std::cmp::Reverse;
use std::process::{Command, self};
use std::sync::Arc;
use std::time::{SystemTime, Instant};
use std::{thread, time};

use by_address::ByAddress;
use ctrlc;
use priority_queue::PriorityQueue;

use crate::config::{App, StartupHook, TimeHook, ShutdownHook};
use crate::cmd;


use std::path::PathBuf;

pub fn run(
    config_path: Option<PathBuf>,
    db_path: Option<PathBuf>,
    app_id: &str,
    args: &[String]
) {
    let config = Arc::new(cmd::get_config(config_path));
    let db = cmd::get_db(db_path);
    let app = config.get_app(app_id)
        .unwrap_or_else(|| panic!("Unknown app: {:}", app_id));
    check_startup_hooks(&app).expect("Startup hook prevented run");

    // Create shutdown handler for SIGINT
    let local_config = config.clone();
    let local_app_id = app_id.to_string();
    ctrlc::set_handler(move || {
        start_shutdown_hooks(&local_config
            .get_app(&local_app_id)
            .unwrap()
            .shutdown_hooks
        );
        process::exit(1);
    }).expect("Could not set shutdown hook!");

    // Calculate for each startup hook when it needs to be run
    let mut time_hook_schedule = schedule_time_hooks(&app.time_hooks);

    // Start the app
    let mut app_cmd = Command::new(&app.cmd)
        .args(app.args.iter().chain(args.iter()))
        .spawn()
        .expect("Failed to execute command");

    // Loop every second while the app is running:
    let delay = time::Duration::from_secs(1);
    while app_cmd.try_wait().unwrap().is_none() {
        thread::sleep(delay);

        // Log usage in db
        let timestamp = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .map(|d| d.as_secs())
            .expect("Could not obtain timestamp");
        db.record_usage(&app.id, timestamp, delay.as_secs())
            .unwrap_or_else(
                |e| println!("Failed to record usage: {:?}", e)
            );

        // Run time hooks that are due
        let now = Instant::now();
        while let Some(hook) = time_hook_schedule.peek()
            .and_then(|(hook_by_addr, prio)| {
                if prio.0 > now {
                    None
                } else {
                    Some(hook_by_addr.0)
                }
        }) {
            start_time_hook(hook);
            if hook.interval.seconds > 0 {
                time_hook_schedule.push_decrease(
                    ByAddress(hook),
                    Reverse(now + hook.interval.into())
                );
            } else {
                time_hook_schedule.pop();
            }
        }
    }

    // Run shutdown hook once app terminates correctly
    start_shutdown_hooks(&app.shutdown_hooks);
}

fn start_time_hook(hook: &TimeHook) {
    Command::new(&hook.cmd)
        .args(&hook.args)
        .spawn()
        .expect("Couldn't start time hook!");
}

fn start_shutdown_hooks(hooks: &[ShutdownHook]) {
    for hook in hooks {
        Command::new(&hook.cmd)
            .args(&hook.args)
            .spawn()
            .expect("Failed to start shutdown hook");
    }
}

fn schedule_time_hooks<'a>(time_hooks: &'a [TimeHook]) -> PriorityQueue<ByAddress<&'a TimeHook>, Reverse<Instant>> {
    let now = Instant::now();
    let mut q: PriorityQueue<ByAddress<&'a TimeHook>, Reverse<Instant>> = PriorityQueue::new();
    for time_hook in time_hooks {
        q.push(ByAddress(time_hook), Reverse(now + time_hook.initial_delay.into()));
    }
    q
}

fn check_startup_hooks(app: &App) -> Result<(), &StartupHook> {
    for hook in &app.startup_hooks {
        let status = Command::new(&hook.cmd)
            .args(&hook.args)
            .status()
            .unwrap_or_else(|_| panic!("Failed to run startup hook: {:?}", hook));
        if !status.success() {
            return Err(&hook)
        }
    }
    Ok(())
}

