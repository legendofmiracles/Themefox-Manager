extern crate dirs;
use std::env;
use std::path::Path;
use std::ffi::OsString;
use std::path::PathBuf;

fn main() {
    let os = std::env::consts::OS;

    if os == "linux" {
        let mut completePath = PathBuf::new();
        let temp: PathBuf = dirs::home_dir().unwrap();
        completePath.push(temp);
        completePath.push(".mozilla/firefox");
        println!("{:?}", completePath.to_str());
        //env::set_current_dir(path);
        //env::set_current_dir("/.mozilla/firefox");
    }
}