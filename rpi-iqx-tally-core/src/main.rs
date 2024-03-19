mod json_handler;

//use std::thread;
use crate::json_handler::*;

fn main() {
    let tally_cfg = init_tally_config().expect("Error accessing tally config file");
    println!("{:?}", tally_cfg);
}
