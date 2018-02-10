use std::process::Command;

use render;

fn render_battery ( state: &str, percentage: i64, time_to: String) -> String {
    let text = String::from(format!(" {} {}% {} ", state, percentage, time_to));

    let length = text.len() as i64;
    let split_point = ( ( percentage * length ) / 100 ) as usize;

    let first_half = text.chars().take(split_point).collect();
    let second_half = text.chars().skip(split_point).collect();

    let colored = format!(
        " {}{} ",
        render::with_fg(
            render::BACKGROUND,
            render::with_bg(render::GREEN, first_half)
        ),
        render::with_bg(render::RED, second_half)
        );

    return String::from(colored);
}

pub fn get() -> String {
    let output = Command::new("upower")
        .args(&["-d"])
        .output().unwrap_or_else(|e| {
            panic!("failed to execute process: {}", e)
        });

    if !output.status.success() {
        return String::from("failure");
    }

    let battery_info_string= String::from_utf8_lossy(&output.stdout);
    let lines = battery_info_string.split("\n");

    let mut state = "";
    let mut percentage = 0;
    let mut time_to = String::from("");
    for line in lines {
        if line.starts_with("    state:") {
            state = match line.split_whitespace().nth(1) {
                Some(x) => x,
                None => ""
            };
        }

        if line.starts_with("    percentage") {
            let mut percentage_sting = String::from(
                match line.split_whitespace().nth(1) {
                    Some(x) => x,
                    None => ""
                }
                );

            percentage_sting.pop();

            percentage = percentage_sting.parse::<i64>().unwrap();
        }

        if line.starts_with("    time to") {
            time_to = line.split_whitespace().skip(3).collect::<Vec<&str>>().join(" ");
        }
    }

    return render_battery(state, percentage, time_to);
}
