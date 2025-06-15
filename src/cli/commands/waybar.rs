use crate::{cli::command_runner::run_command, configs::json_handler::*};

pub fn run_waybar () -> anyhow::Result<()>{
    run_command("waybar &")
}

pub fn enable_waybar(status: bool, config_name: String) -> anyhow::Result<()>{
    let mut config = parse_config_json(&config_name);
    config.waybar.enabled = status;
    return save_config_json(&config_name, config);
}
pub fn waybar_status(config_name: String) -> anyhow::Result<()>{
    let config = parse_config_json(&config_name);
    println!("Enabled: {}\nPath: {}\nTheme: {}",
        &config.waybar.enabled,
        &config.waybar.path,
        &config.waybar.theme);
    Ok(())
}
