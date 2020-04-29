extern crate dirs;
extern crate clap;
use std::env;
use std::path::Path;
use std::path::PathBuf;
use std::fs::File;
use std::io::Read;
use std::fs;
use std::process::Command;
use clap::{App, Arg};




fn main() -> std::io::Result<()> {
    
    let matches = App::new("Themefox-manager")
        .version("1.0")
        .author("MY NAME <legendofmiracles@protonmail.com>")
        .about("Does awesome things with your firefox")
        .arg(
            Arg::with_name("reset")
                .long("reset")
                .help("Resets firefox theme by deleting all chrome files")
                )
        .get_matches();

    if matches.is_present("reset") {
        let message = r#"
        ______  __  __ __    __   __  ___   ___    __   ___  __       __  _    __    __  _    __     __  ____  ___
        |_   _| | || | | __| |  V  | | __| | __|  /__\  \ \_/ /  __  |  V  |  /  \  |  \| |  /  \   / _] | __| | _ \ 
          | |   | >< | | _|  | \_/ | | _|  | _|  | \/ |  > , <  |__| | \_/ | | /\ | | | ' | | /\ | | [/\ | _|  | v / 
          |_|   |_||_| |___| |_| |_| |___| |_|    \__/  /_/ \_\      |_| |_| |_||_| |_|\__| |_||_|  \__/ |___| |_|_\ 
         "#;
        // prints it
        print!("{}", message);
        
        // Prints the starting message
        println!("Starting the program. \n The application will print data to the screen, if you notice that the data is incorrect, please stop the application by htting control+c.");
        // fetches what operating system you use
        let os = std::env::consts::OS;
        if os == "linux" {
            // If the operating system is linux then it does everything that is in those brackets
            // It prints "you are on linux"
            println!("You are on linux.");
            // It gets your home directory
            let home_dir: PathBuf = dirs::home_dir().unwrap();
            // It changes the directory in which it is being executed to the previously set variable (in this case it is the homedir)
            env::set_current_dir(home_dir);
            // checks if the config directory exists
            if Path::new(".config/firefox-theme-manager").exists() == false {
             // creates the config directory if the statement above is false
             fs::create_dir_all(".config/firefox-theme-manager");
            }
 
            
            // The next part is that the program tries to understand with which package manager you have firefox installed
            // The native package manager installs the config files of firefox to /home/$USER/.mozilla/firefox
            let native = Path::new(".mozilla/firefox").exists();
            // The snap one to /home/USER/snap.firefox/common/,mozilla/firefox
            let snap = Path::new("snap/firefox/common/.mozilla/firefox").exists();
            // Makes a new variable
            let mut complete_path = PathBuf::new();
            
            // checks If native is true, which is being set to true/false further up
            if native == true {
                // Prints the message
                println!("You have firefox installed via the native package manager");
                // We already had a very simillar piece of code. Try to understand it yourself :)
                complete_path.push(".mozilla/firefox");
                env::set_current_dir(complete_path);
            // Checks if the variable that determines if firefox was installed via snap is true
            } else if  snap == true {
                println!("You have firefox installed via the snap package manager");
                complete_path.push("snap/firefox/common/.mozilla/firefox");
                env::set_current_dir(complete_path);
            } else {
                // If non of the above is true then it prints an error and asks the user to help the program (not yet fully implemented)
                eprintln!("Error: We can not seem to find your firefox folder, Would you like to specify where it is? Y/n");
                // Todo
            }
        
            let default_profile;
            if Path::new("installs.ini").is_file() == true {
                let mut file = File::open("installs.ini")?;
            } else {
                let mut file = File::open("profiles.ini")?;
            }
            let mut contents = String::new();
            file.read_to_string(&mut contents)?;
            //println!("{}", contents);
            let v: Vec<&str> = contents.split(|c| c == '=' || c == ']' || c == '\n').collect();

            default_profile = v[3];
            let mut new_path = PathBuf::new();
            new_path.push(default_profile);
            env::set_current_dir(new_path);

            println!("This application will now attempt to delete all files in the chrome dir.");

            for entry in fs::read_dir("chrome")? {
                let entry = entry?;
                let path = entry.path();
                fs::remove_file(path);
            }
    
    } else {
        // The ascii art message
    let message = r#"
    ______  __  __ __    __   __  ___   ___    __   ___  __       __  _    __    __  _    __     __  ____  ___
    |_   _| | || | | __| |  V  | | __| | __|  /__\  \ \_/ /  __  |  V  |  /  \  |  \| |  /  \   / _] | __| | _ \ 
      | |   | >< | | _|  | \_/ | | _|  | _|  | \/ |  > , <  |__| | \_/ | | /\ | | | ' | | /\ | | [/\ | _|  | v / 
      |_|   |_||_| |___| |_| |_| |___| |_|    \__/  /_/ \_\      |_| |_| |_||_| |_|\__| |_||_|  \__/ |___| |_|_\ 
     "#;
     // prints it
     print!("{}", message);
     
     // Prints the starting message
     println!("Starting the program. \n The application will print data to the screen, if you notice that the data is incorrect, please stop the application by htting control+c.");
     // fetches what operating system you use
     let os = std::env::consts::OS;
     // The files that the program will download
     let files = ["https://pastebin.com/raw/1LV99cKd"];//, "https://raw.githubusercontent.com/AnubisZ9/Prismatic-Night/master/firefox/chrome/userChrome.js", "https://raw.githubusercontent.com/AnubisZ9/Prismatic-Night/master/firefox/chrome/userChrome.xml", "https://raw.githubusercontent.com/AnubisZ9/Prismatic-Night/master/firefox/chrome/userContent.css"];
     // The names of the files
     let names = ["userChrome.css"];//, "userChrome.js", "userChrome.xml", "userContent.css"];
     // If the operating system is linux then it does everything that is in those brackets
     if os == "linux" {
         // It prints "you are on linux"
         println!("You are on linux.");
         // It gets your home directory
         let home_dir: PathBuf = dirs::home_dir().unwrap();
         // It changes the directory in which it is being executed to the previously set variable (in this case it is linux)
         env::set_current_dir(home_dir);
         // checks if the config directory exists
         if Path::new(".config/firefox-theme-manager").exists() == false {
             // creates the config directory if the statement above is false
             fs::create_dir_all(".config/firefox-theme-manager");
         }
 
 
         // The next part is that the program tries to understand with which package manager you have firefox installed
         // The native package manager installs the config files of firefox to /home/USER/.mozilla/firefox
         let native = Path::new(".mozilla/firefox").exists();
         // The snap one to /home/USER/snap.firefox/common/,mozilla/firefox
         let snap = Path::new("snap/firefox/common/.mozilla/firefox").exists();
         // Makes a new variable
         let mut complete_path = PathBuf::new();
         // checks If native is true, which is being set to true/false further up
         if native == true {
             // Prints the message
             println!("You have firefox installed via the native package manager");
             // We already had a very simillar piece of code. Try to understand it yourself :)
             complete_path.push(".mozilla/firefox");
             env::set_current_dir(complete_path);
         // Checks if the variable that determines if firefox was installed via snap is true
         } else if  snap == true {
             println!("You have firefox installed via the snap package manager");
             complete_path.push("snap/firefox/common/.mozilla/firefox");
             env::set_current_dir(complete_path);
         } else {
             // If non of the above is true then it prints an error and asks the user to help the program (not yet fully implemented)
             eprintln!("Error: We can not seem to find your firefox folder, Would you like to specify where it is? Y/n");
         }
 
         //Checks that the installs.ini file exists (some versions come shipped with that and some do not its really weird) 
         if Path::new("installs.ini").is_file() == true {
             let default_profile;
             let mut file = File::open("installs.ini")?;
             let mut contents = String::new();
             file.read_to_string(&mut contents)?;
             //println!("{}", contents);
             let v: Vec<&str> = contents.split(|c| c == '=' || c == ']' || c == '\n').collect();
             default_profile = v[3];
             let mut new_path = PathBuf::new();
             new_path.push(default_profile);
             env::set_current_dir(new_path);
             
             if Path::new("chrome").exists() == false {
                 fs::create_dir("chrome");
                 println!("Created the chrome directory, because it didn't exist before");
             } else {
                 println!("This application will now attempt to write the files for the firefox customization. \n This will overwrite all files that are now in the chrome directory.");
             }
             
             let mut chrome_path = PathBuf::new();
             chrome_path.push("chrome");
             env::set_current_dir(chrome_path);
 
         } else if Path::new("profiles.ini").is_file() == true{
             let default_profile;
             let mut file = File::open("profiles.ini")?;
             let mut contents = String::new();
             file.read_to_string(&mut contents)?;
             //println!("{}", contents);
             let v: Vec<&str> = contents.split(|c| c == '=' || c == ']' || c == '\n').collect();
             println!("Warning, because for whatever reason firefox didn't generate a installs.ini file, so we will just install the theme to the last used profile.");
             default_profile = v[3];
             let mut new_path = PathBuf::new();
             //new_path.push(env::current_dir()?);
             new_path.push(default_profile);
             env::set_current_dir(new_path);
             
             if Path::new("chrome").exists() == false {
                 fs::create_dir("chrome");
                 println!("Created the chrome directory, because it didn't exist before");
             } else {
                 println!("This application will now attempt to write the files for the firefox customization. \n This will overwrite all files that are now in the chrome directory.");
             }
             
             let mut chrome_path = PathBuf::new();
             chrome_path.push("chrome");
             env::set_current_dir(chrome_path);
 
         }
         
         for file in 0..files.len(){
             
             
             let curl = Command::new("curl")    
             .arg(files[file])
             .arg("-o")
             .arg(names[file])
             .status()
             .expect("curl command failed to start");
              
         }
         Ok(())
         
     } //else if os == "macos"{
         // not yet fully implemented, i am concentrating on linux first and then i am updating it to macos and windows respectavely
        
     //} 
     else {
         eprintln!("Error: You seem to use a Operating System that is not supported. Please report this issue on github (https://github.com/alx365/Themefox-Manager)");
         panic!("Quitting...");
     }
    }
    
    
}
