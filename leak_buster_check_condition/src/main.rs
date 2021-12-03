use leak_buster_lib::cmd::run;

fn main() {
    run::run("../test-config.yaml", "../usage.sqlite", "testapp");
}
