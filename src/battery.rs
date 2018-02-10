use std::process::Command;

use render;

pub fn get() -> String {
    let output = Command::new("upower")
        .args(&["-d"])
        .output().unwrap_or_else(|e| {
            panic!("failed to execute process: {}", e)
        });

    if !output.status.success() {
        return String::from("failure");
    }

    let disk_info_string= String::from_utf8_lossy(&output.stdout);
    //Filesystem              1K-blocks      Used   Available   Use%   Mounted on
    let lines = disk_info_string.split("\n");

    let mut starts_with = "";
    let mut percentage = "";
    let mut time_to = "";
    for line in lines {
        if line.starts_with("    state:") {
            starts_with = match line.split_whitespace().nth(1) {
                Some(x) => x,
                None => ""
            };
        }

        if line.starts_with("    percentage") {
            percentage = match line.split_whitespace().nth(1) {
                Some(x) => x,
                None => ""
            };
        }

        if line.starts_with("    time to") {
            time_to = line;
        }
    }

    println!( "sw {}", starts_with );
    println!( "per {}", percentage );
    println!( "tt {}", time_to);

    return String::from("bat");
}
