extern crate dirs;
use std::env;
//use std::path::Path;
//use std::ffi::OsString;
use std::path::PathBuf;
use std::fs::File;
use std::io::Read;
//use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    let os = std::env::consts::OS;

    if os == "linux" {
        let mut complete_path = PathBuf::new();
        
        let temp: PathBuf = dirs::home_dir().unwrap();
        
        complete_path.push(temp);
        
        complete_path.push(".mozilla/firefox");
        
        env::set_current_dir(complete_path);
        
        let mut file = File::open("installs.ini")?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        //println!("{}", contents);
        let v: Vec<&str> = contents.split(|c| c == '=' || c == ']' || c == '\n').collect();
        let default_profile = v[3];
        println!("{:?}", default_profile);
        
        
        
        Ok(())
        
    } else {
        unimplemented!();
    }
}
