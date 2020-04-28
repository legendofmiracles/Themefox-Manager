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
        let file = openFile();
        println!("{:?}", file);
        
    }
}


fn openFile() -> std::io::Result<()> {
    let mut f = File::open("installs.ini")?;
    Ok(())
}