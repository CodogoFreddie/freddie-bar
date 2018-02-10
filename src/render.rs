pub const BACKGROUND : &str = "#272822";
pub const RED : &'static str = "#f92672";
pub const GREEN : &'static str = "#a6e22e";
pub const ORANGE : &'static str = "#fd971f";
pub const BLUE : &'static str = "#66d9ef";
pub const PURPLE : &str = "#ae81ff";
//pub const YELLOW : &'static str = "#e6db74";
//const WHITE : &'static str = "#eeeeee";

pub fn with_cmd(command: &'static str, message: String) -> String {
    format!("%{{A:{}:}}{}%{{A}}", command, message)
}

pub fn with_bg(color: &'static str, message: String) -> String {
    format!("%{{B{}}}{}%{{B-}}", color, message)
}

pub fn with_fg(color: &'static str, message: String) -> String {
    format!("%{{F{}}}{}%{{F-}}", color, message)
}
