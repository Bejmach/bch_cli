use anyhow::Result;

use crate::cli::command_runner::{output_command, run_command};

/// Get first sink and volume of that string
fn get_volume_info() -> (u32, u32){
    let output_data = output_command("pactl list sinks | grep -E 'Sink #|Volume: ' -m 2");
    let eol = output_data.find("\n").unwrap();
    let sink_id = &output_data[6..eol];

    let volume_start = output_data.find("/").unwrap()+3;
    let volume_end = output_data[volume_start..].find("/").unwrap()-2;
    let volume_percentage = &output_data[volume_start..volume_start + volume_end];

    (sink_id.parse().unwrap(), volume_percentage.parse().unwrap())
}

pub fn print_volume_info() -> Result<()>{
    let (sink, volume) = get_volume_info();
    println!("Sink: {}\nVolume: {}%", sink, volume);

    Ok(())
}

pub fn set_volume(volume: &u32) -> Result<()>{
    let (sink, _volume) = get_volume_info();

    run_command(&format!("pactl set-sink-volume {} {}%", sink, volume))
}

pub fn change_volume(volume_change: &i32) -> Result<()>{
    let (sink, volume) = get_volume_info();

    run_command(&format!("pactl set-sink-volume {} {}%", sink, volume as i32 + volume_change))
}
