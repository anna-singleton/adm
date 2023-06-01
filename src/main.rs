mod deployment;
mod module;

use deployment::Deployment;

fn main() {
    let s = include_str!("test.toml");
    let d = Deployment::new(s, "cynthia");
}
