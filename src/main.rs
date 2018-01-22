use std::{thread, time, fmt};

const BACKGROUND : &'static str = "#272822";
const WHITE : &'static str = "#eeeeee";
const RED : &'static str = "#f92672";
const ORANGE : &'static str = "#fd971f";
const YELLOW : &'static str = "#e6db74";
const GREEN : &'static str = "#a6e22e";
const BLUE : &'static str = "#66d9ef";
const PURPLE : &'static str = "#ae81ff";

fn with_cmd(command: String, message: String) -> String {
    format!("%{{A:{}:}}{}%{{A}}", command, message)
}

fn with_bg(color: &'static str, message: String) -> String {
    format!("%{{B{}}}{}%{{B-}}", color, message)
}

fn with_fg(color: &'static str, message: String) -> String {
    format!("%{{F{}}}{}%{{F-}}", color, message)
}

fn render_left() -> String {
    let acc = String::from("left");
    return acc;
}

fn render_center() -> String {
    let acc = with_fg(RED, String::from("Center"));
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
        BACKGROUND, 
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
