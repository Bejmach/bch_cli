use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Config{
    pub global_theme: String,

    pub wallpaper: Wallpaper,
    pub waybar: Waybar,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Wallpaper{
    pub enabled: bool,
    pub path: String,
    pub theme: String,
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Waybar{
    pub enabled: bool,
    pub path: String,
    pub theme: String,
}
