use std::process::Command;

use render;

//TODO
//change partitions to be a struct;

fn is_interesting_partition ( partition : (i64, i64, f64, &str) ) -> bool {
    let ( used, _size, percentage, mounted_on ) = partition;

    return 
        used != 0
        &&
        percentage > 5.0
        &&
        mounted_on.split("/").count() == 2;
}

fn render_partition ( partition : (i64, i64, f64, &str) ) -> String {
    let ( _used, _size, percentage, mounted_on ) = partition;

    let msg = String::from(format!(" {}: {}% ", mounted_on, percentage as i64));

    if percentage > 80.0 {
        return render::with_bg(render::RED, msg);
    }
    if percentage > 40.0 {
        return render::with_bg(render::ORANGE, msg);
    }

    return render::with_fg(
        render::BACKGROUND,
        render::with_bg(render::GREEN, msg)
        );
}

pub fn get () -> String {
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
            None => "/___",
        };

        let percentage = 100.0 * used as f64 / size as f64;

        let partition = ( used, size, percentage, mounted_on );

        if !is_interesting_partition(partition ){
            continue;
        }

        partitions_data.push(render_partition(partition));
    }

    return partitions_data.join("");
}
