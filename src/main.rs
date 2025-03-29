mod metadata;

use metadata::Config;
use std::fs;
use toml;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let toml_content = fs::read_to_string("rules_v2.toml")?;
    let decoded: Config = toml::from_str(&toml_content).unwrap();
    println!("{:#?}", decoded);

    Ok(())
}