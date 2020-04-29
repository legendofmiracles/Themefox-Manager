extern crate dirs;
use std::env;
use std::path::Path;
use std::path::PathBuf;
use std::fs::File;
use std::io::Read;
use std::fs;
use std::process::Command;

fn main() -> std::io::Result<()> {
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

        //Checks that the installs.ini file exists (some versions come shipped with that and some do not its realy weird) 
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
        // not yet fully implemented, i am concentrating on linux first and then i am updating it to macos and windows respectavely
        let mut complete_path = PathBuf::new();
        
        let home_dir: PathBuf = dirs::home_dir().unwrap();
        
        complete_path.push(home_dir);
        
        complete_path.push("Library/Application Support/firefox");
        
        env::set_current_dir(complete_path);
        
        
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
        eprintln!("Error: You seem to use a Operating System that is not supported. Please report this issue on github (https://github.com/alx365/Themefox-Manager)");
        panic!("Quitting...");
    }
}
