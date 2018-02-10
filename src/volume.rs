use std::process::Command;
use regex::Regex;

use render;

fn render_volume ( highest_volume: i64, is_muted: bool ) -> String {
    if is_muted {
        return render::with_bg(render::RED, String::from("   MUTED!   ") );
    } else {
        let text = String::from(format!(" volume {}% ", highest_volume));

        let length = text.len() as i64;
        let split_point = ( ( highest_volume * length ) / 100 ) as usize;

        let first_half = text.chars().take(split_point).collect();
        let second_half = text.chars().skip(split_point).collect();

        let colored = format!(
            "{}{}",
            render::with_bg(render::PURPLE, first_half),
            render::with_bg(render::BLUE, second_half)
            );

        return String::from(colored);
    }
}

pub fn get() -> String {
    let output = Command::new("amixer")
        .args(&["sget", "Master"])
        .output().unwrap_or_else(|e| {
            panic!("failed to execute process: {}", e)
        });

    if !output.status.success() {
        return String::from("failure");
    }

    let battery_info_string= String::from_utf8_lossy(&output.stdout);
    let lines = battery_info_string.split("\n");
    let re = Regex::new(r"Playback \d+ \[(\d+)%\] \[(\w+)\]").unwrap();
    let mut highest_volume = 0;
    let mut is_muted = false;

    for line in lines {
        for capture in re.captures_iter(line) {

            let vol = String::from(&capture[1]).parse::<i64>().unwrap();
            let muted = capture[2] == *"off";

            if vol > highest_volume {
                highest_volume = vol
            }

            if muted {
                is_muted = true ;
            }
        }
    }

    return render::with_cmd(
        "gnome-terminal -e \"alsamixer\"",
        render_volume(highest_volume, is_muted)
    );
}

