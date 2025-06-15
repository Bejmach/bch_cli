use anyhow::anyhow;

use crate::{cli::command_runner::run_command, configs::json_handler::*};

pub fn enable_wallpaper(status: bool, config_name: String) -> anyhow::Result<()>{
    let mut config = parse_config_json(&config_name);
    config.wallpaper.enabled = status;
    return save_config_json(&config_name, config);
}
pub fn wallpaper_status(config_name: String) -> anyhow::Result<()>{
    let config = parse_config_json(&config_name);
    println!("Enabled: {}\nPath: {}\nTheme: {}\nName: {}",
        &config.wallpaper.enabled,
        &config.wallpaper.path,
        &config.wallpaper.theme,
        &config.wallpaper.name);
    Ok(())
}

pub fn change_wallpaper(name: String, config_name: String) -> anyhow::Result<()>{
    let mut config = parse_config_json(&config_name);

    if !&config.wallpaper.enabled {
        return Err(anyhow!("Wallpaper is disabled. Enable it before changing"));
    }

    let wallpaper_path = &config.wallpaper.path;
    let theme;

    if &config.wallpaper.theme != ""{
        theme = &config.wallpaper.theme;
    }
    else if &config.global_theme != ""{
        theme = &config.global_theme;
    }
    else{
        return Err(anyhow!("No theme for wallpaper found"));
    }
    

    println!("setting wallpaper to \"{}\"", name);
    let status = run_command(&format!("waypaper --wallpaper {}/{}/{}", wallpaper_path, theme, name));
    if status.is_err(){
        println!("failed setting wallpaper to \"{}\"\nfile with path {}/{}/{} does not exist", name, wallpaper_path, theme, name);
    }
    else{
        config.wallpaper.name = name;

        return save_config_json(&config_name, config);
    }

    status
}

pub fn preview_wallpaper (name: String, config_name: String) -> anyhow::Result<()> {
    let config = parse_config_json(&config_name);

    if !&config.wallpaper.enabled {
        return Err(anyhow!("Wallpaper is disabled. Enable it before changing"));
    }

    let wallpaper_path = &config.wallpaper.path;
    let theme;

    if &config.wallpaper.theme != ""{
        theme = &config.wallpaper.theme;
    }
    else if &config.global_theme != ""{
        theme = &config.global_theme;
    }
    else{
        return Err(anyhow!("No theme for wallpaper found"));
    }

    let status = run_command(&format!("waypaper --wallpaper {}/{}/{}", wallpaper_path, theme, name));
    if status.is_err(){
        println!("failed setting wallpaper to \"{}\"\nfile with path {}/{}/{} does not exist", name, wallpaper_path, theme, name);
    }

    status
}

pub fn revert_wallpaper (config_name: String) -> anyhow::Result<()> {
    let config = parse_config_json(&config_name);

    if !&config.wallpaper.enabled {
        return Err(anyhow!("Wallpaper is disabled. Enable it before changing"));
    }

    let wallpaper_path = &config.wallpaper.path;
    let wallpaper = &config.wallpaper.name;
    let theme;

    if &config.wallpaper.theme != ""{
        theme = &config.wallpaper.theme;
    }
    else if &config.global_theme != ""{
        theme = &config.global_theme;
    }
    else{
        return Err(anyhow!("No theme for wallpaper found"));
    }

    let status = run_command(&format!("waypaper --wallpaper {}/{}/{}", wallpaper_path, theme, wallpaper));
    if status.is_err(){
        println!("failed setting wallpaper to \"{}\"\nfile with path {}/{}/{} does not exist", wallpaper, wallpaper_path, theme, wallpaper);
    }

    status 
}

pub fn change_wallpapers_path (path: String, config_name: String) -> anyhow::Result<()> {
    let mut config = parse_config_json(&format!("~/.config/bch_cli/configs/{}.json", config_name));
    config.wallpaper.path = path;
    save_config_json(&config_name, config)
}

pub fn change_wallpapers_theme (theme: String, config_name: String) -> anyhow::Result<()>{
    let mut config = parse_config_json(&format!("~/.config/bch_cli/configs/{}.json", config_name));
    config.wallpaper.theme = theme;
    save_config_json(&config_name, config)
}
