use leakbuster::cmd::run;

fn main() {
    run::run("test-config.yaml", "usage.sqlite", "testapp");
}
