use std::env;

fn main() {}

pub fn get_clargs() -> Vec<String> {
    let args: Vec<String> = env::args().collect();
    args
}
