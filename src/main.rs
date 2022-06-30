#[macro_use]
extern crate ini;
extern crate glob;

use chrono::{Utc};
use glob::glob;
use std::fs;
use std::collections::HashMap;

fn main() {
    // Get some program variables from the config file
    let map = ini!("/etc/locutus/settings.ini");
    let token = map["default"]["token"].clone().unwrap();
    let logdir = map["default"]["logdirectory"].clone().unwrap();
    // debugging: print the variables
    println!("Token: {}", token);
    println!("Log Directory: {}", logdir);

    // get today's log files so we can send them off to the API
    let date = Utc::now().format("%Y%m%d").to_string();
    let source_files = format!("{}/{}-*", logdir, date);
    // debugging: print the variables
    println!("Log Files: {}", source_files);

    // grab the correct files from the log dir and iterate through them
    for entry in glob(&source_files).expect("Failed to read glob pattern") {
        let file_name = format!("{}", entry.unwrap().display());
        let file_content = fs::read_to_string(&file_name).expect("Something went wrong reading the file");
        let json: serde_json::Value = serde_json::from_str(&file_content)
        .expect("JSON does not have correct format.");
        
        let token = &token.to_string();
        let duration = &json["archive"]["duration"].to_string();
        let files = &json["archive"]["stats"]["nfiles"].to_string();
        let compressed = &json["archive"]["stats"]["compressed_size"].to_string();
        let original = &json["archive"]["stats"]["original_size"].to_string();
        let repository = &json["repository"]["id"].to_string();
        let date = &date.to_string();

        // add the variables to a hasmap which we can send to the API
        let mut map = HashMap::new();
        map.insert("token", token);
        map.insert("duration", duration);
        map.insert("files", files);
        map.insert("compressed_size", compressed);
        map.insert("original_size", original);
        map.insert("repository_id", repository);
        map.insert("date", &date);
        // debugging: print the variables
        println!("{:?}", map)
    }
}
// SEND TO:
// https://borgreporting/<TOKEN>/post