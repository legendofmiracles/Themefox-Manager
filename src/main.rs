extern crate dirs;
use std::env;
use std::path::Path;
//use std::ffi::OsString;
use std::path::PathBuf;
use std::fs::File;
use std::io::Read;
//use std::io::{stdout, Write};
//use curl::easy::Easy;
//use std::fs;
//use std::io::prelude::*;
use std::process::Command;

fn main() -> std::io::Result<()> {
    let os = std::env::consts::OS;
    let files = ["https://raw.githubusercontent.com/AnubisZ9/Prismatic-Night/master/firefox/chrome/userChrome.css", "https://raw.githubusercontent.com/AnubisZ9/Prismatic-Night/master/firefox/chrome/userChrome.js", "https://raw.githubusercontent.com/AnubisZ9/Prismatic-Night/master/firefox/chrome/userChrome.xml", "https://raw.githubusercontent.com/AnubisZ9/Prismatic-Night/master/firefox/chrome/userContent.css"];
    let names = ["userChrome.css", "userChrome.js", "userChrome.xml", "userContent.css"];
    if os == "linux" {
        
        
        
        
        let home_dir: PathBuf = dirs::home_dir().unwrap();
        
        env::set_current_dir(home_dir);
        
        let native = Path::new(".mozilla/firefox").exists();
        let snap = Path::new("snap/firefox/common/.mozilla/firefox").exists();
        let flatpack = Path::new("TEST").exists();
        let appimage = Path::new("TEST").exists();
        let mut complete_path = PathBuf::new();
        
        if native == true {
            println!("You have firefox installed via the native package manager");
            complete_path.push(".mozilla/firefox");
            env::set_current_dir(complete_path);
        
        } else if  snap == true {
            println!("You have firefox installed via the snap package manager");
            complete_path.push("snap/firefox/common/.mozilla/firefox");
            env::set_current_dir(complete_path);
        } else {
            eprintln!("Error: We can not seem to find your firefox folder, Would you like to specify where it is? Y/n");

        }
        
        
        
        
        if Path::new("installs.ini").is_file() == true {
            let default_profile;
            let mut file = File::open("installs.ini")?;
            let mut contents = String::new();
            file.read_to_string(&mut contents)?;
            //println!("{}", contents);
            let v: Vec<&str> = contents.split(|c| c == '=' || c == ']' || c == '\n').collect();
            default_profile = v[3];
            let mut new_path = PathBuf::new();
            //new_path.push(env::current_dir()?);
            new_path.push(default_profile);
            new_path.push("chrome");
            env::set_current_dir(new_path);

        } else if Path::new("profiles.ini").is_file() == true{
            let default_profile;
            let mut file = File::open("profiles.ini")?;
            let mut contents = String::new();
            file.read_to_string(&mut contents)?;
            //println!("{}", contents);
            let v: Vec<&str> = contents.split(|c| c == '=' || c == ']' || c == '\n').collect();
            default_profile = v[3];
            let mut new_path = PathBuf::new();
            //new_path.push(env::current_dir()?);
            new_path.push(default_profile);
            new_path.push("chrome");
            env::set_current_dir(new_path);

        }
        
        
        
       
        
        for file in 0..files.len(){
            
            let clear = Command::new("echo")
            .arg("' '")    
            .arg(">")
            .arg(names[file])
            .status()
            .expect("echo command failed to start");
             
        
        
        
            let curl = Command::new("curl")    
            .arg(files[file])
            .arg("-o")
            .arg(names[file])
            .status()
            .expect("curl command failed to start");
             
        }
        Ok(())
        
    } else if os == "macos"{
        
        let mut complete_path = PathBuf::new();
        
        let temp: PathBuf = dirs::home_dir().unwrap();
        
        complete_path.push(temp);
        
        complete_path.push("Library/firefox");
        
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
            
            let clear = Command::new("echo")
            .arg("' '")    
            .arg(">")
            .arg(names[file])
            .status()
            .expect("echo command failed to start");
             
        
        
        
            let curl = Command::new("curl")    
            .arg(files[file])
            .arg("-o")
            .arg(names[file])
            .status()
            .expect("curl command failed to start");
             
            println!("{}", curl)
        }
        Ok(())
        
    } else {
        unimplemented!();
        eprintln!("Error: You seem to use a OS that is not supported. Please report this issue on github (https://www.github.com/alx365/firefox-manager");
        panic!("Quitting...");
    }
}
