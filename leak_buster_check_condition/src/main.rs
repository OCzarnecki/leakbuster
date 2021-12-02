use leak_buster_lib::usage_monitor;

fn main() {
    usage_monitor::launch_with_monitoring(
        "/usr/bin/sleep",
        &vec!["3".to_string()]
    );
}
