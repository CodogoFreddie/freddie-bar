use chrono::prelude::*;

pub fn get(){
    let current_time = Local::now(); // e.g. `2014-11-28T21:45:59.324310806+09:00`

    return current_time.format("%Y-%m-%d %H:%M:%S").to_string();
}
