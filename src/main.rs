extern crate chrono;
extern crate serde;
extern crate serde_json;
extern crate regex;

#[macro_use]
extern crate serde_derive;

use std::{thread, time};

mod clock;
mod disk_usage;
mod i3;
mod render;
mod battery;
//mod load;
//mod memory;
//mod network;
mod volume;

fn render_left() -> String {
    return format!(
        "{} {}",
        i3::get(),
        volume::get()
    );
}

fn render_center() -> String {
    return format!(
        "{}",
        clock::get()
    );
}

fn render_right() -> String {
    return format!(
        "{} {}",
        battery::get(),
        disk_usage::get(),
    );
}

fn render_bar() -> String {
    let left = render_left();
    let center = render_center();
    let right = render_right();

    return format!( "%{{l}}{}%{{c}}{}%{{r}}{}", left, center, right );
}

fn main() {
    let ten_millis = time::Duration::from_millis(1000);

    loop {
        println!("{}", render_bar());
        thread::sleep(ten_millis);
    }
}
