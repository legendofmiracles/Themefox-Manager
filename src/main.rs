extern crate dirs;
use std::env;
//use std::path::Path;
//use std::ffi::OsString;
use std::path::PathBuf;
use std::fs::File;
use std::io::Read;
use std::io::{stdout, Write};
use curl::easy::Easy;
//use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    let os = std::env::consts::OS;
    let mut files = ["https://raw.githubusercontent.com/AnubisZ9/Prismatic-Night/master/firefox/chrome/userChrome.css", "https://raw.githubusercontent.com/AnubisZ9/Prismatic-Night/master/firefox/chrome/userChrome.js", "https://raw.githubusercontent.com/AnubisZ9/Prismatic-Night/master/firefox/chrome/userChrome.xml", "https://raw.githubusercontent.com/AnubisZ9/Prismatic-Night/master/firefox/chrome/userContent.css"];
        
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
        
        let mut new_path = PathBuf::new();
        //new_path.push(env::current_dir()?);
        new_path.push(default_profile);
        new_path.push("chrome");

        env::set_current_dir(new_path);
        
        for file in 0..files.len(){
            let mut easy = Easy::new();
            easy.url("https://www.rust-lang.org/").unwrap();
            easy.write_function(|data| {
                stdout().write_all(data).unwrap();
                Ok(data.len())
            }).unwrap();
            easy.perform().unwrap();
        
            let result = easy.response_code().unwrap();
        }
        Ok(())
        
    } else {
        unimplemented!();
    }
}
