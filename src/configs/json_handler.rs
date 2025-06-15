use std::{env::home_dir, fs::{self, File}, io::Write};

use crate::configs::config::*;

pub fn get_config_path(config_name: &String) -> String {
    let name;

    if config_name == "" {
        name = "def";
    }
    else {
        name = &config_name;
    }

    let mut path = home_dir().expect("Failed to get home directory");
    path.push(format!(".config/bch_cli/configs/{}.json", name));
    path.to_str().unwrap().to_string()
}

pub fn parse_config_json(name: &String) -> Config{
    println!("{}", get_config_path(name));
    let json_file = fs::read_to_string(get_config_path(name)).unwrap();

    serde_json::from_str(&json_file).unwrap()
}

pub fn save_config_json(name: &String, config: Config) -> anyhow::Result<()>{
    let json_string = serde_json::to_string(&config).unwrap();

    let mut file = File::create(get_config_path(name))?;
    file.write_all(json_string.as_bytes())?;

    Ok(())
}
