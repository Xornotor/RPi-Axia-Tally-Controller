mod json_handler;

use serde_json;
use std::{fs, io::Write};
use crate::json_handler::*;

fn main() {
    let tally_cfg = TallyConfig::standard();

    let serialized = tally_cfg.to_json().unwrap();

    println!("serialized: \n{}", serialized);

    let deserialized: TallyConfig = serde_json::from_str(&serialized).unwrap();

    println!("deserialized: \n{:?}", deserialized);

    let mut f = fs::File::create("tally_config.json").expect("Failed to create");

    f.write_all(&serialized.as_bytes()).expect("Failed to write");

    drop(f);
}
