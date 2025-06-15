use std::{process::Command};
use anyhow::{Ok};

use crate::configs::json_handler::{parse_config_json, save_config_json};

use super::commands::{wallpaper::revert_wallpaper, waybar::run_waybar};

pub fn run_command(command: &str) -> anyhow::Result<()>{
    let status = Command::new("sh")
        .arg("-c")
        .arg(command)
        .status()?;

    if !status.success(){
        anyhow::bail!("command failed with status: {}", status);
    }

    Ok(())
} 
pub fn output_command(command: &str) -> String{
    let output = Command::new("sh")
        .arg("-c")
        .arg(command)
        .output()
        .expect("Command could not be finished");

    String::from_utf8(output.stdout).unwrap()
}

pub fn change_global_theme (theme: String, config_name: String) -> anyhow::Result<()> {
    let mut config = parse_config_json(&config_name);
    config.global_theme = theme;
    save_config_json(&config_name, config)
}

pub fn load_config (config_name: String) -> anyhow::Result<()> {
    let config = parse_config_json(&config_name);
    if config.waybar.enabled{
        run_waybar()?;
    }
    if config.wallpaper.enabled{
        revert_wallpaper(config_name)?;
    }

    Ok(())
}

