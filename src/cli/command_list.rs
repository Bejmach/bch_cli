use clap::{Parser, Subcommand};

#[derive(Subcommand)]
pub enum Commands{
    Wallpaper(WallpaperCommand),
    Waybar(WaybarCommand),
    Volume(VolumeCommand),
    Load(LoadCommand),
    Set(SetCommand),
}


#[derive(Parser)]
pub struct WallpaperCommand{
    #[command(subcommand)]
    pub action: WallpaperAction,
}
#[derive(Subcommand)]
pub enum WallpaperAction{
    Status,
    Enable,
    Disable,
    Set(WallpaperSetCommand),

    Preview {
        name: String,
    },
    Revert
}

#[derive(Parser)]
pub struct WallpaperSetCommand{
    #[command(subcommand)]
    pub action: WallpaperSetAction,
}
#[derive(Subcommand)]
pub enum WallpaperSetAction{
    Name {
        name: String,
    },
    Theme {
        name: String,
    },
    Path {
        path: String,
    }
}


#[derive(Parser)]
pub struct WaybarCommand{
    #[command(subcommand)]
    pub action: WaybarAction,
}
#[derive(Subcommand)]
pub enum WaybarAction{
    Status,
    Enable,
    Disable,
    Set(WaybarSetCommand),
}

#[derive(Parser)]
pub struct WaybarSetCommand{
    #[command(subcommand)]
    pub action: WaybarSetAction,
}
#[derive(Subcommand)]
pub enum WaybarSetAction{
    Theme {
        name: String,
    },
    Path {
        path: String,
    }
}


#[derive(Parser)]
pub struct VolumeCommand{
    #[command(subcommand)]
    pub action: VolumeAction,
}
#[derive(Subcommand)]
pub enum VolumeAction{
    Info,
    Set {
        vol: u32,
    },
    Change{
        vol: i32,
    }
}


#[derive(Parser)]
pub struct LoadCommand{
    #[command(subcommand)]
    pub action: LoadAction,
}
#[derive(Subcommand)]
pub enum LoadAction{
    Config {
        name: String,
    }
}


#[derive(Parser)]
pub struct SetCommand{
    #[command(subcommand)]
    pub action: SetAction,
}
#[derive(Subcommand)]
pub enum  SetAction {
    GlobalTheme {
        name: String,
    }
}
