extern crate clap;
extern crate dirs;
//extern crate zip;
use clap::{App, Arg};
use std::env;
use std::fs;
use std::fs::File;
use std::io;
use std::io::Read;
use std::path::Path;
use std::path::PathBuf;
use std::process::Command;
use zip;
//use zipper::Archive;
//use std::ops::Index<usize>;

fn main() /*-> std::io::Result<()>*/
{
    let matches = App::new("themefox-manager")
        .version("1.0")
        //.set_term_width(if let Some((Width(w), _)) = terminal_size() { w as usize } else { 120 })
        .author("MY NAME <legendofmiracles@protonmail.com>")
        .about("Does awesome things with your firefox. \n If no valid argument supplied it will try to install the theme from that url \n DON'T RUN THIS WITH ELEVATED PERMISSIONS")
        .arg(
            Arg::with_name("reset")
                .long("reset")
                .help("Resets firefox theme by deleting all chrome files")
                )
        .get_matches();

    if matches.is_present("reset") {
        let os = std::env::consts::OS;
        println!("Deleting all chrome files so that your firefox looks normal again");
        if os == "linux" {
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
            } else if snap == true {
                println!("You have firefox installed via the snap package manager");
                complete_path.push("snap/firefox/common/.mozilla/firefox");
                env::set_current_dir(complete_path);
            } else {
                // If non of the above is true then it prints an error and asks the user to help the program (not yet fully implemented)
                eprintln!("Error: We can not seem to find your firefox folder. \n If you ran this application with elevated permissions, please try again without. \n Would you like to specify where it is? Y/n");
            }

            find_profile(false);
            fs::remove_dir_all("chrome");
        } else if os == "macos" {
            /////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
            /////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
            /////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
            // It prints "you are on macos"
            println!("You are on macos.");
            // It gets your home directory
            let home_dir: PathBuf = dirs::home_dir().unwrap();
            // It changes the directory in which it is being executed to the previously set variable (in this case it is the homedir)
            env::set_current_dir(home_dir);
            // checks if the config directory exists
            // I know this isn't a common config directory on macos. But i'm lazy
            if Path::new(".config/firefox-theme-manager").exists() == false {
                // creates the config directory if the statement above is false
                fs::create_dir_all(".config/firefox-theme-manager");
            }

            // The next part is that the program tries to understand with which package manager you have firefox installed
            // The native package manager installs the config files of firefox to /home/USER/.mozilla/firefox
            let native = Path::new("Library/Application Support/Firefox/Profiles").exists();
            // Makes a new variable
            let mut complete_path = PathBuf::new();
            // checks If native is true, which is being set to true/false further up
            if native == true {
                // We already had a very simillar piece of code. Try to understand it yourself :)
                complete_path.push("Library/Application Support/Firefox");
                env::set_current_dir(complete_path);
            } else {
                // If non of the above is true then it prints an error and asks the user to help the program (not yet fully implemented)
                eprintln!("Error: We can not seem to find your firefox folder. \n If you ran this application with elevated permissions, please try again without. \n You can find it by typing about:profiles in the adress bar and then select the button open in finder on the first one. \n  Would you like to specify where it is? Y/n" );
            }

            find_profile(false);
            fs::remove_dir_all("chrome");
        } else if os == "windows" {
            /////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
            /////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
            /////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
            // It prints "you are on macos"
            println!("You are on windows.");
            // It gets your home directory
            let home_dir: PathBuf = dirs::home_dir().unwrap();
            // It changes the directory in which it is being executed to the previously set variable (in this case it is the homedir)
            env::set_current_dir(home_dir);
            // checks if the config directory exists
            // I know this isn't a common config directory on macos. But i'm lazy
            /*
            if Path::new(".config/firefox-theme-manager").exists() == false {
                // creates the config directory if the statement above is false
                fs::create_dir_all(".config/firefox-theme-manager");
            }
            */

            // The next part is that the program tries to understand with which package manager you have firefox installed
            // The native package manager installs the config files of firefox to /home/USER/.mozilla/firefox
            let native = Path::new("AppData\\Roaming\\Mozilla\\Firefox\\Profiles").exists();
            // Makes a new variable
            let mut complete_path = PathBuf::new();
            // checks If native is true, which is being set to true/false further up
            if native == true {
                // We already had a very simillar piece of code. Try to understand it yourself :)
                complete_path.push("AppData\\Roaming\\Mozilla\\Firefox");
                env::set_current_dir(complete_path);
            } else {
                // If non of the above is true then it prints an error and asks the user to help the program (not yet fully implemented)
                eprintln!("Error: We can not seem to find your firefox folder. \n If you ran this application with elevated permissions, please try again without. \n You can find it by typing about:profiles in the adress bar and then select the button open in finder on the first one. \n  Would you like to specify where it is? Y/n" );
            }

            find_profile(false);
            fs::remove_dir_all("chrome");
        }
    } else {
        // The ascii art message
        let message = r#"
    ______  __  __ __    __   __  ___   ___    __   ___  __       __  _    __    __  _    __     __  ____  ___
    |_   _| | || | | __| |  V  | | __| | __|  /__\  \ \_/ /  __  |  V  |  /  \  |  \| |  /  \   / _| | __| | _ \ 
      | |   | >< | | _|  | \_/ | | _|  | _|  | \/ |  > , <  |__| | \_/ | | /\ | | | ' | | /\ | | |/\ | _|  | v / 
      |_|   |_||_| |___| |_| |_| |___| |_|    \__/  /_/ \_\      |_| |_| |_||_| |_|\__| |_||_|  \__/ |___| |_|_\ 
     "#;
        // prints it
        print!("{}", message);

        // Prints the starting message
        println!("Starting the program. \n The application will print data to the screen, if you notice that the data is incorrect, please stop the application by htting control+c.");
        // fetches what operating system you use
        let os = std::env::consts::OS;
        // The files that the program will download
        let mut files = Vec::new();
        files.push("https://pastebin.com/raw/1LV99cKd"); //, "https://raw.githubusercontent.com/AnubisZ9/Prismatic-Night/master/firefox/chrome/userChrome.js", "https://raw.githubusercontent.com/AnubisZ9/Prismatic-Night/master/firefox/chrome/userChrome.xml", "https://raw.githubusercontent.com/AnubisZ9/Prismatic-Night/master/firefox/chrome/userContent.css"];
        let mut names = Vec::new(); // The names of the files
        names.push("userChrome.css"); //, "userChrome.js", "userChrome.xml", "userContent.css"];
                                      // If the operating system is linux then it does everything that is in those brackets
        if os == "linux" {
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
            } else if snap == true {
                println!("You have firefox installed via the snap package manager");
                complete_path.push("snap/firefox/common/.mozilla/firefox");
                env::set_current_dir(complete_path);
            } else {
                // If non of the above is true then it prints an error and asks the user to help the program (not yet fully implemented)
                eprintln!("Error: We can not seem to find your firefox folder. \n If you ran this application with elevated permissions, please try again without. \n Would you like to specify where it is? Y/n");
            }

            find_profile(true);
            /*
            for file in 0..files.len() {
                let curl = Command::new("curl")
                    .arg(files[file])
                    .arg("-o")
                    .arg(names[file])
                    .status()
                    .expect("curl command failed to start");
            */
            download("http://alx365.github.io/minimal-functional-fox.zip");
        //}
        } else if os == "macos" {
            /////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
            /////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
            /////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
            // It prints "you are on macos"
            println!("You are on macos.");
            // It gets your home directory
            let home_dir: PathBuf = dirs::home_dir().unwrap();
            // It changes the directory in which it is being executed to the previously set variable (in this case it is the homedir)
            env::set_current_dir(home_dir);
            // checks if the config directory exists
            // I know this isn't a common config directory on macos. But i'm lazy
            if Path::new(".config/firefox-theme-manager").exists() == false {
                // creates the config directory if the statement above is false
                fs::create_dir_all(".config/firefox-theme-manager");
            }

            // The next part is that the program tries to understand with which package manager you have firefox installed
            // The native package manager installs the config files of firefox to /home/USER/.mozilla/firefox
            let native = Path::new("Library/Application Support/Firefox/Profiles").exists();
            // Makes a new variable
            let mut complete_path = PathBuf::new();
            // checks If native is true, which is being set to true/false further up
            if native == true {
                // We already had a very simillar piece of code. Try to understand it yourself :)
                complete_path.push("Library/Application Support/Firefox");
                env::set_current_dir(complete_path);
            } else {
                // If non of the above is true then it prints an error and asks the user to help the program (not yet fully implemented)
                eprintln!("Error: We can not seem to find your firefox folder. \n If you ran this application with elevated permissions, please try again without. \n You can find it by typing about:profiles in the adress bar and then select the button open in finder on the first one. \n  Would you like to specify where it is? Y/n" );
            }

            find_profile(true);

            for file in 0..files.len() {
                Command::new("curl")
                    .arg(files[file])
                    .arg("-o")
                    .arg(names[file])
                    .status()
                    .expect("curl command failed to start");
            }
        } else if os == "windows" {
            /////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
            /////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
            /////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
            // It prints "you are on macos"
            println!("You are on windows.");
            // It gets your home directory
            let home_dir: PathBuf = dirs::home_dir().unwrap();
            // It changes the directory in which it is being executed to the previously set variable (in this case it is the homedir)
            env::set_current_dir(home_dir);
            // checks if the config directory exists
            // I know this isn't a common config directory on macos. But i'm lazy
            /*
            if Path::new(".config/firefox-theme-manager").exists() == false {
                // creates the config directory if the statement above is false
                fs::create_dir_all(".config/firefox-theme-manager");
            }
            */

            // The next part is that the program tries to understand with which package manager you have firefox installed
            // The native package manager installs the config files of firefox to /home/USER/.mozilla/firefox
            let native = Path::new("AppData\\Roaming\\Mozilla\\Firefox\\Profiles").exists();
            // Makes a new variable
            let mut complete_path = PathBuf::new();
            // checks If native is true, which is being set to true/false further up
            if native == true {
                // We already had a very simillar piece of code. Try to understand it yourself :)
                complete_path.push("AppData\\Roaming\\Mozilla\\Firefox");
                env::set_current_dir(complete_path);
            } else {
                // If non of the above is true then it prints an error and asks the user to help the program (not yet fully implemented)
                eprintln!("Error: We can not seem to find your firefox folder. \n If you ran this application with elevated permissions, please try again without. \n You can find it by typing about:profiles in the adress bar and then select the button open in finder on the first one. \n  Would you like to specify where it is? Y/n" );
            }

            find_profile(true);
            for file in 0..files.len() {
                Command::new("curl")
                    .arg(files[file])
                    .arg("-o")
                    .arg(names[file])
                    .status()
                    .expect("curl command failed to start");
            }
        } else {
            eprintln!("Error: You seem to use a Operating System that is not supported. Please report this issue on github (https://github.com/alx365/Themefox-Manager)");
            panic!("Quitting...");
        }
    }
}

fn find_profile(goChrome: bool) {
    let default_profile;
    let mut contents = String::new();
    if Path::new("installs.ini").is_file() == true {
        let mut file = File::open("installs.ini").expect("Unable to open");
        file.read_to_string(&mut contents);
    } else if Path::new("profiles.ini").is_file() == true {
        let mut file = File::open("profiles.ini").expect("Unable to open");
        file.read_to_string(&mut contents);
    } else {
        println!("Error: We cannot find your last used or your default profile. \n Please report this issue on github (https://github.com/alx365/Themefox-Manager)");
    }
    //println!("{}", contents);
    let v: Vec<&str> = contents
        .split(|c| c == '=' || c == ']' || c == '\n')
        .collect();
    default_profile = v[3];
    let mut new_path = PathBuf::new();
    new_path.push(default_profile);
    //println!("{:?}", new_path);
    env::set_current_dir(new_path).expect("failed to cd. \n Please report this issue on GitHub");
    if Path::new("chrome").exists() == false {
        if goChrome == true {
            fs::create_dir("chrome");
            println!("Created the chrome directory, because it didn't exist before");
        } else {
            println!("You chrome directory doesn't exist, so we can't remove it -.-")
        }
    } else {
        if goChrome == true {
            println!("This application will now attempt to write the files for the firefox customization. \n This will overwrite all files that are now in the chrome directory.");
        } else {
            print!("The application will now delete all files in the chrome directory");
        }
    }
    let mut chrome_path = PathBuf::new();
    if goChrome == true {
        chrome_path.push("chrome");
    }
    env::set_current_dir(chrome_path);
}
/*
fn download(files: Vec<>, names: Vec<>) {

}
*/

fn download(file: &str) {
    Command::new("curl")
        .arg("-L")
        .arg(file)
        .arg("-o")
        .arg("ChromeFiles.zip")
        .status()
        .expect("curl command failed to start");

    let file = fs::File::open("ChromeFiles.zip").unwrap();
    let mut archive = zip::ZipArchive::new(file).unwrap();

    for i in 0..archive.len() {
        let mut file = archive.by_index(i).unwrap();
        println!("{}", file.name());
        let toutpath: Vec<&str> = file.name().split("/").collect();
        let outpath = PathBuf::from(toutpath[1]);
        {
            let comment = file.comment();
            if !comment.is_empty() {
                println!("File {} comment: {}", i, comment);
            }
        }

        if (&*file.name()).ends_with('/') {
            println!(
                "File {} extracted to \"{}\"",
                i,
                outpath.as_path().display()
            );
            fs::create_dir_all(&outpath).unwrap();
        } else {
            println!(
                "File {} extracted to \"{}\" ({} bytes)",
                i,
                outpath.as_path().display(),
                file.size()
            );
            if let Some(p) = outpath.parent() {
                if !p.exists() {
                    fs::create_dir_all(&p).unwrap();
                }
            }
            let mut outfile = fs::File::create(&outpath).unwrap();
            io::copy(&mut file, &mut outfile).unwrap();
        }
    }
}
