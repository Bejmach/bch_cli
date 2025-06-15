# bch_cli

Tool for controlling linux system written in rust, with config in json

## Current support

### Wallpapers
#### Needed packages
- waypaper( + backend for it)
### Audio
- pipewire
### Waybar
- well... waybar

## Current commands
### Wallpaper
- enable/disable
- status
- path {path}
- theme {path}
- set/preview/revert {name}     set wallpaper on path "{path}/{theme}/{name}"
### Waybar
- enable/disable
- status
### Volume
- info
- set {absolute volume}
- change -- {relative volume}
### Load
- config {name}                 Load config on path "~/.config/bch_cli/configs/{name.json}"
