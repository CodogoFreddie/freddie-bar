use std::{thread, time, fmt};
use std::process::Command;

const BACKGROUND : &str = "#272822";
const RED : &'static str = "#f92672";
//const WHITE : &'static str = "#eeeeee";
//const ORANGE : &'static str = "#fd971f";
//const YELLOW : &'static str = "#e6db74";
//const GREEN : &'static str = "#a6e22e";
//const BLUE : &'static str = "#66d9ef";
//const PURPLE : &str = "#ae81ff";

fn get_disk_usage () -> String {
    let output = Command::new("df")
        .output().unwrap_or_else(|e| {
            panic!("failed to execute process: {}", e)
    });

    if !output.status.success() {
        return String::from("failure");
    }

    let disk_info_string= String::from_utf8_lossy(&output.stdout);
    //Filesystem              1K-blocks      Used   Available   Use%   Mounted on
    let lines = disk_info_string.split("\n");

    let mut partitions_data = Vec::new();
    let mut first = true;
    for line in lines {
        if first {
            first = false;
            continue;
        }
        let mut fields_itterator = line.split_whitespace();

        //fieldsItterator.next();
        //fieldsItterator.next();
        let used = match fields_itterator.nth(2) {
            Some(x) => x.parse::<i64>().unwrap(),
            None => 0,
        };
        let size = match fields_itterator.nth(0) {
            Some(x) => x.parse::<i64>().unwrap(),
            None => 1,
        };
        let mounted_on = match fields_itterator.nth(1) {
            Some(x) => x,
            None => "___",
        };
        partitions_data.push( ( used, size, mounted_on ) );
    }

    println!("{:?}", partitions_data);

    //let vec = split.collect::<Vec<&str>>();
    // OR
    //let vec: Vec<&str> = split.collect();

    return String::from("return");
}

fn with_cmd(command: String, message: String) -> String {
    format!("%{{A:{}:}}{}%{{A}}", command, message)
}

fn with_bg(color: String, message: String) -> String {
    format!("%{{B{}}}{}%{{B-}}", color, message)
}

fn with_fg(color: String, message: String) -> String {
    format!("%{{F{}}}{}%{{F-}}", color, message)
}

fn render_left() -> String {
    let acc = get_disk_usage();
    return acc;
}

fn render_center() -> String {
    let acc = with_fg(String::from(RED), String::from("Center"));
    return acc;
}

fn render_right() -> &'static str {
    let acc = "Right";
    return acc;
}

fn render_bar() -> String {
    let left = render_left();
    let center = render_center();
    let right = render_right();

    return with_bg(
        String::from(BACKGROUND),
        format!(
            "%{{l}}{}%{{c}}{}%{{r}}{}", left, center, right
            )
        );
}

fn main() {
    let ten_millis = time::Duration::from_millis(1000);

    loop {
        println!("{}", render_bar());
        thread::sleep(ten_millis);
    }
}
