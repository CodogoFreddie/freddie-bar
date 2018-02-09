pub const BACKGROUND : &str = "#272822";
pub const RED : &'static str = "#f92672";
//const WHITE : &'static str = "#eeeeee";
//const ORANGE : &'static str = "#fd971f";
//const YELLOW : &'static str = "#e6db74";
//const GREEN : &'static str = "#a6e22e";
//const BLUE : &'static str = "#66d9ef";
//const PURPLE : &str = "#ae81ff";

pub fn with_cmd(command: String, message: String) -> String {
    format!("%{{A:{}:}}{}%{{A}}", command, message)
}

pub fn with_bg(color: String, message: String) -> String {
    format!("%{{B{}}}{}%{{B-}}", color, message)
}

pub fn with_fg(color: String, message: String) -> String {
    format!("%{{F{}}}{}%{{F-}}", color, message)
}
