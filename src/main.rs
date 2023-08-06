use challenge_checker::analytics::folders::run;
use challenge_checker::config::Config;

fn main() {
    let config = Config::from_file("tests/challenges/c1/basic.yaml").unwrap();
    run(&config);
}
