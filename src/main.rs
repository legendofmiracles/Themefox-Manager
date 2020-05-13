extern crate clap;
extern crate dirs;
//extern crate zip;
use clap::{App, Arg};
use colored::*;
use dialoguer::{theme::ColorfulTheme, Confirm, Input, Select};
use serde_json::Value;
use std::env;
use std::fs;
use std::fs::File;
use std::io;
use std::io::Read;
use std::path::Path;
use std::path::PathBuf;
use std::process::Command;
use std::str;
use zip::ZipArchive;
//use std::io::Write;

fn main() {
    let matches = App::new("themefox-manager")
        .version("v0.2")
        //.set_term_width(if let Some((Width(w), _)) = terminal_size() { w as usize } else { 120 })
        .author("MY NAME <legendofmiracles@protonmail.com>")
        .about("Does awesome things with your firefox. \n If no valid argument supplied it will try to install the theme from that url \n DON'T RUN THIS WITH ELEVATED PERMISSIONS")
        .arg(
            Arg::with_name("URL")
            .help("Sets the URL to install from")
            .required(false)
            .index(1))
        .arg(
            Arg::with_name("reset")
                .long("reset")
                .help("Resets firefox theme by deleting all chrome files")
                )
        .get_matches();

    if matches.is_present("reset") {
        if Confirm::new()
            .with_prompt("Do you want to continue, and delete all chrome files?")
            .interact()
            .unwrap()
        {
            println!("ok, your chrome files will be deleted");
        } else {
            println!("Ok, looks like you changed your mind");
            panic!("{}", "Quitting...".red());
        }
        let os = std::env::consts::OS;
        println!("Deleting all chrome files so that your firefox looks normal again");
        if os == "linux" {
            // It gets your home directory
            let home_dir: PathBuf = dirs::home_dir().unwrap();
            // It changes the directory in which it is being executed to the previously set variable (in this case it is the homedir)
            env::set_current_dir(home_dir).expect("Error: failed to cd");
            // checks if the config directory exists
            if Path::new(".config/firefox-theme-manager").exists() == false {
                // creates the config directory if the statement above is false
                fs::create_dir_all(".config/firefox-theme-manager")
                    .expect(&format!("{}", "Error: failed to mkdir".red()));
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
                env::set_current_dir(complete_path)
                    .expect(&format!("{}", "Error: failed to cd".red()));
            // Checks if the variable that determines if firefox was installed via snap is true
            } else if snap == true {
                println!("You have firefox installed via the snap package manager");
                complete_path.push("snap/firefox/common/.mozilla/firefox");
                env::set_current_dir(complete_path)
                    .expect(&format!("{}", "Error: failed to cd".red()));
            } else {
                // If non of the above is true then it prints an error and asks the user to help the program (not yet fully implemented)
                eprintln!("Error: We can not seem to find your firefox folder. \n If you ran this application with elevated permissions, please try again without. \n Would you like to specify where it is? Y/n");
            }

            find_profile(false);
            fs::remove_dir_all("chrome").expect(&format!("{}", "Error: failed to rmdir".red()));
        } else if os == "macos" {
            // It gets your home directory
            let home_dir: PathBuf = dirs::home_dir().unwrap();
            // It changes the directory in which it is being executed to the previously set variable (in this case it is the homedir)
            env::set_current_dir(home_dir).expect(&format!("{}", "Error: failed to cd".red()));

            // The next part is that the program tries to understand with which package manager you have firefox installed
            // The native package manager installs the config files of firefox to /home/USER/.mozilla/firefox
            let native = Path::new("Library/Application Support/Firefox/Profiles").exists();
            // Makes a new variable
            let mut complete_path = PathBuf::new();
            // checks If native is true, which is being set to true/false further up
            if native == true {
                // We already had a very simillar piece of code. Try to understand it yourself :)
                complete_path.push("Library/Application Support/Firefox");
                env::set_current_dir(complete_path).expect("Error: failed to cd");
            } else {
                // If non of the above is true then it prints an error and asks the user to help the program (not yet fully implemented)
                eprintln!("Error: We can not seem to find your firefox folder. \n If you ran this application with elevated permissions, please try again without. \n You can find it by typing about:profiles in the adress bar and then select the button open in finder on the first one. \n  Would you like to specify where it is? Y/n" );
            }

            find_profile(false);
            fs::remove_dir_all("chrome").expect(&format!("{}", "Error: failed to rmdir".red()));
        } else if os == "windows" {
            // It gets your home directory
            let home_dir: PathBuf = dirs::home_dir().unwrap();
            // It changes the directory in which it is being executed to the previously set variable (in this case it is the homedir)
            env::set_current_dir(home_dir).expect(&format!("{}", "Error: failed to cd".red()));

            // The next part is that the program tries to understand with which package manager you have firefox installed
            // The native package manager installs the config files of firefox to /home/USER/.mozilla/firefox
            let native = Path::new("AppData\\Roaming\\Mozilla\\Firefox\\Profiles").exists();
            // Makes a new variable
            let mut complete_path = PathBuf::new();
            // checks If native is true, which is being set to true/false further up
            if native == true {
                // We already had a very simillar piece of code. Try to understand it yourself :)
                complete_path.push("AppData\\Roaming\\Mozilla\\Firefox");
                env::set_current_dir(complete_path)
                    .expect(&format!("{}", "Error: failed to cd".red()));
            } else {
                // If non of the above is true then it prints an error and asks the user to help the program (not yet fully implemented)
                eprintln!("Error: We can not seem to find your firefox folder. \n If you ran this application with elevated permissions, please try again without. \n You can find it by typing about:profiles in the adress bar and then select the button open in finder on the first one. \n  Would you like to specify where it is? Y/n" );
            }

            find_profile(false);
            fs::remove_dir_all("chrome").expect(&format!("{}", "Error: failed to rmdir".red()));
        }
    } else if matches.is_present("URL") {
        // The ascii art message
        let message = r#"
    ______  __  __  ___  __   __  ___   ___    __   ___  __       __  _    __    __  _    __     __   ___   ___
    |_   _| | || | | __| |  V  | | __| | __|  /__\  \ \_/ /  __  |  V  |  /  \  |  \| |  /  \   / _| | __| | _ \ 
      | |   | >< | | _|  | \_/ | | _|  | _|  | \/ |  > , <  |__| | \_/ | | /\ | | | ' | | /\ | | |/\ | _|  | v / 
      |_|   |_||_| |___| |_| |_| |___| |_|    \__/  /_/ \_\      |_| |_| |_||_| |_|\__| |_||_|  \__/ |___| |_|_\ 
     "#;
        // prints it
        print!("{}\n", message);
        let arguments: Vec<String> = env::args().collect();
        //let mut output = "";
        let the_argument: Vec<&str>;
        the_argument = arguments[arguments.len() - 1].split(' ').collect();

        println!("{}", the_argument[1]);
        let mut download_url = String::new();
        if the_argument[1].starts_with("http")
            && the_argument[1].contains("://")
            && the_argument[1].contains("themefox.net")
            && the_argument[1].contains("/")
        {
            let id: Vec<&str> = the_argument[1].split('/').collect();
            //println!("{:?}", id[id.len() - 2]);

            let output_exit = Command::new("curl")
                .arg(format!("127.0.0.1:1234/get/{}", id[id.len() - 2]))
                .output()
                .expect(&format!("{}", "Error: cURL failed to spawn".red()));

            let output = output_exit.stdout;
            let output = str::from_utf8(&output).unwrap();
            let downloads;
            let output_json: Value = serde_json::from_str(output)
                .expect("the json seems to be corrupt. Please report this issue on github.");
            if let Some(output_json) = output_json.as_array() {
                downloads = output_json.len();
            } else {
                panic!(
                    "{}",
                    "json again seemed to be wrong formatted... Please report this issue.".red()
                );
            }
            //println!("{}", downloads);
            if downloads - 2 == 1 {
                download_url = format!(
                    "http://beta.themefox.net/themes/{}/{}-{}.{}",
                    output_json[3]["theme_id"],
                    output_json[3]["id"],
                    output_json[3]["filename"],
                    output_json[3]["filetype"]
                );
            } else if downloads - 2 > 1 {
                let mut selections = Vec::new();
                for i in 0..downloads - 2 {
                    selections.push(&output_json[i + 2]["title"]);
                }
                let selection = Select::with_theme(&ColorfulTheme::default())
                    .with_prompt(format!(
                        "{}",
                        "Pick your flavor of the theme (navigate with arrow keys)".yellow()
                    ))
                    .default(0)
                    .items(&selections[..])
                    .interact()
                    .unwrap();
                download_url = format!(
                    "http://beta.themefox.net/themes/{}/{}-{}.{}",
                    output_json[selection + 2]["theme_id"],
                    output_json[selection + 2]["id"],
                    output_json[selection + 2]["filename"],
                    output_json[selection + 2]["filetype"]
                );
            }
        } else {
            println!("The argument you supplied didn't seem to be a correct url, or you didn't supply any url. \n Run with -h in order to see the usage");
            panic!("{}", "\n There is nothing to do. \n Quitting...".red());
        }

        // fetches what operating system you use
        let os = std::env::consts::OS;
        // If the operating system is linux then it does everything that is in those brackets
        if os == "linux" {
            // It gets your home directory
            let home_dir: PathBuf = dirs::home_dir().unwrap();
            // It changes the directory in which it is being executed to the previously set variable (in this case it is the homedir)
            env::set_current_dir(home_dir).expect("Error: failed to cd");
            // checks if the config directory exists
            if Path::new(".config/firefox-theme-manager").exists() == false {
                // creates the config directory if the statement above is false
                fs::create_dir_all(".config/firefox-theme-manager")
                    .expect(&format!("{}", "Error: unable to cmkdird".red()));
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
                //println!("You have firefox installed via the native package manager");
                // We already had a very simillar piece of code. Try to understand it yourself :)
                complete_path.push(".mozilla/firefox");

            // Checks if the variable that determines if firefox was installed via snap is true
            } else if snap == true {
                //println!("You have firefox installed via the snap package manager");
                complete_path.push("snap/firefox/common/.mozilla/firefox");
            } else {
                complete_path.push(manual_profile_path());
            }
            env::set_current_dir(complete_path).expect(&format!("{}", "Error: unable to cd".red()));

            find_profile(true);
            download(&download_url);
        } else if os == "macos" {
            // It gets your home directory
            let home_dir: PathBuf = dirs::home_dir().unwrap();
            // It changes the directory in which it is being executed to the previously set variable (in this case it is the homedir)
            env::set_current_dir(home_dir).expect(&format!("{}", "Error: unable to cd".red()));
            // checks if the config directory exists
            // I know this isn't a common config directory on macos. But i'm lazy
            if Path::new(".config/firefox-theme-manager").exists() == false {
                // creates the config directory if the statement above is false
                fs::create_dir_all(".config/firefox-theme-manager")
                    .expect("Error: failed to mkdir");
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
            } else {
                complete_path.push(manual_profile_path());
            }
            env::set_current_dir(complete_path).expect(&format!("{}", "Error: unable to cd".red()));

            find_profile(true);

            download(&download_url);
        } else if os == "windows" {
            // It prints "you are on macos"
            //println!("You are on windows.");
            // It gets your home directory
            let home_dir: PathBuf = dirs::home_dir().unwrap();
            // It changes the directory in which it is being executed to the previously set variable (in this case it is the homedir)
            env::set_current_dir(home_dir).expect(&format!("{}", "Error: unable to cd".red()));

            // The next part is that the program tries to understand with which package manager you have firefox installed
            // The native package manager installs the config files of firefox to /home/USER/.mozilla/firefox
            let native = Path::new("AppData\\Roaming\\Mozilla\\Firefox\\Profiles").exists();
            // Makes a new variable
            let mut complete_path = PathBuf::new();
            // checks If native is true, which is being set to true/false further up
            if native == true {
                // We already had a very simillar piece of code. Try to understand it yourself :)
                complete_path.push("AppData\\Roaming\\Mozilla\\Firefox");
            } else {
                complete_path.push(manual_profile_path());
            }
            env::set_current_dir(complete_path).expect(&format!("{}", "Error: unable to cd".red()));

            find_profile(true);
            download(&download_url);
        } else {
            eprintln!("Error: You seem to use a Operating System that is not supported. Please report this issue on github (https://github.com/alx365/Themefox-Manager)");
            panic!("{}", "Quitting...".red());
        }
    } else {
        let os = std::env::consts::OS;
        let mut path = dirs::config_dir().unwrap();
        path.push("/themefox-manager.txt");

        if !path.exists() {
            print!("Performing first time setup and installing, configuring stuff, so that this application will work.");
            let _file =
                File::create(path).expect(&format!("{}", "Failed to make config directory".red()));
            //file.write_all(b"DO NOT DELETE THIS FILE, IF YOU SHOULD DELETE IS, IT WILL ON THE NEXT STARTUP, WITHOUT ANY ARGUMENTS, TRY TO INSTALL THE CUSTOM PROTOCOL HANDLERS").expect(&format!("{}", "Error: Failed to write to config file".red()))
            if os == "linux" {
                File::create("/usr/bin/themefox-manager").expect(&format!(
                    "{}",
                    "Error: failed to create file in /usr/bin. Got r00t?".red()
                ));
                fs::copy(std::env::current_exe().unwrap(), "/usr/bin/themefox-manager").expect(&format!("{}", "Failed to copy executable content to the executable in the /usr/bin directory.\nDo i have the permissions for this executable?".red()));
                
                /*fs::remove_file(std::env::current_exe().unwrap()).expect(&format!(
                    "{}",
                    "Error: An error occured when deleteing this executable.".red()
                ));*/
                let output_exit = Command::new("curl")
                .arg()
                .output()
                .expect(&format!("{}", "Error: cURL failed to spawn".red()));
                let output = output_exit.stdout;
            } else if os == "windows" {
                File::create("C:\\Program Files\\themefox\\themefox-manager.exe").expect(&format!(
                    "{}",
                    "Error: failed to create file in C:\\Program Files\\themefox\\themefox-manager.exe. Did you run this with elevate permissions?".red()
                ));
                fs::copy(std::env::current_exe().unwrap(), "C:\\Program Files\\themefox\\themefox-manager.exe").expect(&format!("{}", "Failed to copy executable content to the executable in the C:\\Program Files\\themefox\\themefox-manager.exe directory.\nDo i have the permissions for this executable?".red()));
                /*
                fs::remove_file(std::env::current_exe().unwrap()).expect(&format!(
                    "{}",
                    "Error: An error occured when deleteing this executable.".red()
                ));
                */
            }
        } else {
            print!("Bad usage. \n Have a look at the usage with the `-h` flag");
        }
    }
    if Confirm::new()
        .with_prompt(format!("{}", "Choose any, to exit.".yellow()))
        .interact()
        .unwrap()
    {
        panic!("{}", "Quitting...".red());
    } else {
        panic!("{}", "Quitting...".red());
    }
}

fn find_profile(go_chrome: bool) {
    let default_profile;
    let mut contents = String::new();
    if Path::new("installs.ini").is_file() == true {
        let mut file = File::open("installs.ini")
            .expect(&format!("{}", "Error: unable to open installs.ini".red()));
        file.read_to_string(&mut contents)
            .expect("Error: Unable to read file");
    } else if Path::new("profiles.ini").is_file() == true {
        let mut file = File::open("profiles.ini")
            .expect(&format!("{}", "Error: unable to open profiles.ini".red()));
        file.read_to_string(&mut contents)
            .expect("Error: Unable to read file");
    } else {
        println!("Error: We cannot find your last used or your default profile. because the file is missing, with which we can find out.\n Please report this issue on github (https://github.com/alx365/Themefox-Manager)");
        panic!("{}", "Quitting...".red());
    }
    //println!("{}", contents);
    let v: Vec<&str> = contents
        .split(|c| c == '=' || c == ']' || c == '\n')
        .collect();
    default_profile = v[3];
    if !default_profile.contains(".") {
        println!("{}", "You seem to be using a very old firefox version. Consider updating. \n We do not support such old versions".red());
        panic!("");
    }
    let mut new_path = PathBuf::new();
    new_path.push(default_profile);
    //println!("{:?}", new_path);
    env::set_current_dir(new_path).expect("failed to cd. \n Please report this issue on GitHub");
    if Path::new("chrome").exists() == false {
        if go_chrome == true {
            fs::create_dir("chrome").expect("Error: failed to mkdir");
            println!("Created the chrome directory, because it didn't exist before");
        } else {
            println!("You chrome directory doesn't exist, so we can't remove it -.-")
        }
    } else {
        if go_chrome == true {
            println!("This application will now attempt to write the files for the firefox customization. \n This will overwrite all files that are now in the chrome directory.");
        } else {
            print!("The application will now delete all files in the chrome directory");
        }
    }
    if go_chrome == true {
        let mut chrome_path = PathBuf::new();
        chrome_path.push("chrome");
        env::set_current_dir(chrome_path).expect(&format!(
            "{}",
            "Error: failed to cd into the Chrome dir".red()
        ));
    }
}

fn download(file: &str) {
    Command::new("curl")
        .arg("-L")
        .arg(file)
        .arg("-o")
        .arg("ChromeFiles.zip")
        .status()
        .expect(&format!("{}", "Error: cURL failed to start".red()));

    let file = fs::File::open("ChromeFiles.zip").unwrap();
    let mut archive = ZipArchive::new(file).unwrap();

    for i in 0..archive.len() {
        let mut file = archive.by_index(i).unwrap();
        let outpath = file.sanitized_name();

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
    fs::remove_file("ChromeFiles.zip").expect(&format!(
        "{}",
        "Error: failed to rm the Chrome zip file".red()
    ));
}

fn manual_profile_path() -> String {
    eprintln!("Error: We can not seem to find your firefox folder. \n If you ran this application with elevated permissions, please try again without. \n You can find your profile folder by typing about:profiles in the adress bar and then select the button open directory on the first one. Then navigate back one directory and thats the path you should enter\n" );
    if Confirm::new()
        .with_prompt(format!(
            "{}",
            "Would you now like to manually specify the chrome directory?".yellow()
        ))
        .interact()
        .unwrap()
    {
        let path: String = Input::with_theme(&ColorfulTheme::default())
            .with_prompt(format!("{}", "What is the path?".yellow()))
            .interact()
            .unwrap();
        return path;
    } else {
        println!("Ok, Bye.");
        panic!("{}", "Quitting...".red());
    }
}
