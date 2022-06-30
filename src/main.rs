#[macro_use]
extern crate ini;

use chrono::Utc;
use std::fs::File;
use std::path::Path;

fn main() -> std::io::Result<()> {
    let map = ini!("/etc/locutus/settings.ini");
    let token = map["default"]["token"].clone().unwrap();
    let backupdir = map["borg"]["backupdir"].clone().unwrap();
    let backuprepo = map["borg"]["backuprepo"].clone().unwrap();
    let starttime = map["borg"]["starttime"].clone().unwrap();
    println!("Token: {}", token);
    println!("Backup Directory: {}", backupdir);
    println!("Backup Repository: {}", backuprepo);
    println!("Backup Start time: {}", starttime);
    let pattern = format!("{}-", Utc::now().format("%Y%m%d"));
    for entry in std::fs::read_dir("/var/log/borg")? {
        let entry = entry?;
        let file_name = entry.file_name();
        let file_name: &Path = file_name.as_ref();
        if !file_name.starts_with(&pattern) {
        let json: serde_json::Value = serde_json::from_reader(File::open(entry.path())?)?;
        println!("Duration: {}", json["archive"]["duration"]);
        println!("Files: {}", json["archive"]["stats"]["nfiles"]);
        println!("Compressed Size: {}",json["archive"]["stats"]["compressed_size"]);
        println!("Original Size: {}",json["archive"]["stats"]["original_size"]);
        }
    }
    Ok(())
}