//use std::process::Command;

use renderers;

    //fn get_disk_usage () -> String {
        //let output = Command::new("df")
            //.output().unwrap_or_else(|e| {
                //panic!("failed to execute process: {}", e)
            //});

        //if !output.status.success() {
            //return String::from("failure");
        //}

        //let disk_info_string= String::from_utf8_lossy(&output.stdout);
        ////Filesystem              1K-blocks      Used   Available   Use%   Mounted on
        //let lines = disk_info_string.split("\n");

        //let mut partitions_data = Vec::new();
        //let mut first = true;
        //for line in lines {
            //if first {
                //first = false;
                //continue;
            //}
            //let mut fields_itterator = line.split_whitespace();

            ////fieldsItterator.next();
            ////fieldsItterator.next();
            //let used = match fields_itterator.nth(2) {
                //Some(x) => x.parse::<i64>().unwrap(),
                //None => 0,
            //};
            //let size = match fields_itterator.nth(0) {
                //Some(x) => x.parse::<i64>().unwrap(),
                //None => 1,
            //};
            //let mounted_on = match fields_itterator.nth(1) {
                //Some(x) => x,
                //None => "___",
            //};
            //partitions_data.push( ( used, size, mounted_on ) );
        //}

        //println!("{:?}", partitions_data);

        ////let vec = split.collect::<Vec<&str>>();
        //// OR
        ////let vec: Vec<&str> = split.collect();

        //return String::from("return");
    //}

    pub fn get () -> String {
        return String::from("hello modules");
    }

}
