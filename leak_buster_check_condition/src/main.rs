use leak_buster_lib;

fn main() {
    println!("{:?}", leak_buster_lib::parse_condition("weekday or (not weekday)"));
}
