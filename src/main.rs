//extern crate chrono;

use std::{thread, time};

//mod clock;
mod disk_usage;
mod render;

fn render_left() -> String {
    let acc = String::from("RIGHT");
    return acc;
}

fn render_center() -> String {
    let acc = render::with_fg((render::RED), String::from("Center"));
    return acc;
}

fn render_right() -> String {
    let acc = disk_usage::get();
    //let acc = clock::get();
    return acc;
}

fn render_bar() -> String {
    let left = render_left();
    let center = render_center();
    let right = render_right();

    return render::with_bg(
        render::BACKGROUND,
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
