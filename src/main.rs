extern crate dirs;
use std::env;
use std::path::Path;
use std::ffi::OsString;
use std::path::PathBuf;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let os = std::env::consts::OS;

    if os == "linux" {
        let mut completePath = PathBuf::new();
        let temp: PathBuf = dirs::home_dir().unwrap();
        completePath.push(temp);
        completePath.push(".mozilla/firefox");
        env::set_current_dir(completePath);
        let mut file = File::open("installs.ini");
        let mut contents = String::new();
        file.read_to_string(&mut contents);
        println!("{:?}", file);
        
    }
}

/*
fn openFile() -> String {
    let f = File::open("installs.ini");
    //Ok(())
}
*/