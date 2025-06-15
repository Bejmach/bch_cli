pub mod command_list;
pub mod command_runner;
pub mod commands;

use anyhow::Ok;
use clap::{Parser};

use command_list::{Commands, LoadAction, SetAction, VolumeAction, WallpaperAction, WallpaperSetAction, WaybarAction, WaybarSetAction};
use command_runner::*;
use commands::{wallpaper::{change_wallpaper, change_wallpapers_path, change_wallpapers_theme, enable_wallpaper, preview_wallpaper, revert_wallpaper, wallpaper_status}, waybar::{enable_waybar, waybar_status}};

use crate::pw_sound_controll::volume::{change_volume, print_volume_info, set_volume};

#[derive(Parser)]
#[command(name = "bch_scriprts")]
#[command(about = "theme controll with Cli", long_about = None)]
pub struct Cli{
    #[command(subcommand)]
    command: Commands,
}

impl Cli {
    pub fn match_commands(&self) -> anyhow::Result<()>{
        let config_name: String = "def".to_string();

        match &self.command{
            Commands::Wallpaper(wall_cmd) => match &wall_cmd.action {
                WallpaperAction::Set(wall_set_cmd) => match &wall_set_cmd.action {
                    WallpaperSetAction::Name { name } => {
                        change_wallpaper(name.to_string(), config_name)
                    }
                    WallpaperSetAction::Path { path } => {
                        change_wallpapers_path(path.to_string(), config_name)
                    }
                    WallpaperSetAction::Theme { name } => {
                        change_wallpapers_theme(name.to_string(), config_name)
                    }
                }

                WallpaperAction::Enable => {
                    enable_wallpaper(true, config_name)
                }
                WallpaperAction::Disable => {
                    enable_wallpaper(false, config_name)
                }
                WallpaperAction::Status => {
                    wallpaper_status(config_name)
                }

                WallpaperAction::Preview { name } => {
                    preview_wallpaper(name.to_string(), "def".to_string())
                }
                WallpaperAction::Revert => {
                    revert_wallpaper("def".to_string())
                }
            }
            Commands::Waybar(waybar_cmd) => match &waybar_cmd.action {
                WaybarAction::Set(waybar_set_cmd) => match &waybar_set_cmd.action {
                    WaybarSetAction::Theme { name } => {
                        println!("{}", name);
                        Ok(())
                    }
                    WaybarSetAction::Path { path } => {
                        println!("{}", path);
                        Ok(())
                    }
                }
                WaybarAction::Enable => {
                    enable_waybar(true, config_name)
                }
                WaybarAction::Disable => {
                    enable_waybar(false, config_name)
                }
                WaybarAction::Status => {
                    waybar_status(config_name)
                }  
            }
            Commands::Volume(volume_cmd) => match &volume_cmd.action {
                VolumeAction::Info {} => {
                    print_volume_info()
                }
                VolumeAction::Set { vol } => {
                    set_volume(vol)
                }
                VolumeAction::Change { vol } => {
                    change_volume(vol)
                }

            }
            Commands::Set(set_cmd) => match &set_cmd.action {
                SetAction::GlobalTheme { name } => {
                    change_global_theme(name.to_string(), config_name)
                }
            }
            Commands::Load(load_cmd) => match &load_cmd.action {
                LoadAction::Config { name } => {
                    load_config(name.to_string())
                }
            }
        }
    }
}
