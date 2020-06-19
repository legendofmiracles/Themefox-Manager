use clap::{App, Arg};
use colored::*;
use dialoguer::{theme::ColorfulTheme, Confirm, Input, Select};
use dirs;
use serde_json::Value;
use std::{
    env, fs, fs::File, fs::OpenOptions, io, io::Read, io::Write, path::Path, path::PathBuf,
    process::Command, str,
};
use zip::ZipArchive;
fn main() {
    // The ascii art message
    let message = r#"
    ______  __  __  ___  __   __  ___   ___    __   ___  __       __  _    __    __  _    __     __   ___   ___
    |_   _| | || | | __| |  V  | | __| | __|  /__\  \ \_/ /  __  |  V  |  /  \  |  \| |  /  \   / _| | __| | _ \ 
      | |   | >< | | _|  | \_/ | | _|  | _|  | \/ |  > , <  |__| | \_/ | | /\ | | | ' | | /\ | | |/\ | _|  | v / 
      |_|   |_||_| |___| |_| |_| |___| |_|    \__/  /_/ \_\      |_| |_| |_||_| |_|\__| |_||_|  \__/ |___| |_|_\ 
     "#;
    // prints it
    print!("{}\n", message);
    let app = App::new("themefox-manager")
        .name("themefox-manager")
        .version("v0.9.9.9")
        //.set_term_width(if let Some((Width(w), _)) = terminal_size() { w as usize } else { 120 })
        .author("The authors name <lolsu@hash.fyi>")
        .about("Installs themes to your firefox, from a valid themefox url, or git url")
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
        .arg(
            Arg::with_name("path")
            .long("dir")
            .short("d")
            .help("Sets the path to install to, will automaticly trigger if no path is being found")
            .long_help("Sets the path to install to, will automaticly trigger if no path is being found. Can very well be used to install themes for waterfox, firefox developer edition and other browsers that are a fork from firefox")
        )
        .arg(
            Arg::with_name("git")
            .long("git")
            .short("g")
            .help("Installs from git repo, must be specified in a full URL. For example: https://githost.domain/foo/bar.git. Will remove all other files in the dir")
            //.long("Installs from git repo, must be specified in a full URL. . Will remove all other files in the dir")
        )
        .arg(Arg::with_name("profile")
        .long("profile")
        .short("p")
        .help("This argument lets you chose which profile you want to install from"));
    let matches = app.get_matches();
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
        succes("Fetched your operating system");
        if os == "linux" {
            get_firefox_linux(false, matches, "null".to_string())
        } else if os == "macos" {
            // It gets yourd home directory
            let home_dir: PathBuf = dirs::home_dir().unwrap();
            // It changes the directory in which it is being executed to the previously set variable (in this case it is the homedir)
            env::set_current_dir(home_dir).expect(&format!("{}", "Error: failed to cd".red()));

            // The next part is that the program tries to understand with which package manager you have firefox installed
            // The native package manager installs the config files of firefox to /home/USER/.mozilla/firefox
            let native = Path::new("Library/Application Support/Firefox/Profiles").exists();
            // Makes a new variable
            let mut complete_path = PathBuf::new();
            // checks If native is true, which is being set to true/false further up
            if native == true && !matches.is_present("path") {
                // We already had a very simillar piece of code. Try to understand it yourself :)
                complete_path.push("Library/Application Support/Firefox");
                env::set_current_dir(complete_path).expect("Error: failed to cd");
            } else {
                complete_path.push(manual_profile_path());
                env::set_current_dir(complete_path)
                    .expect(&format!("{}", "Error: failed to cd".red()));
            }
            succes("Got your firefox directory");

            find_profile(false, matches.is_present("profile"));
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
            if native == true && !matches.is_present("path") {
                // We already had a very simillar piece of code. Try to understand it yourself :)
                complete_path.push("AppData\\Roaming\\Mozilla\\Firefox");
                env::set_current_dir(complete_path)
                    .expect(&format!("{}", "Error: failed to cd".red()));
            } else {
                complete_path.push(manual_profile_path());
                env::set_current_dir(complete_path)
                    .expect(&format!("{}", "Error: failed to cd".red()));
            }
            succes("Got your firefox directory");

            find_profile(false, matches.is_present("profile"));
            fs::remove_dir_all("chrome").expect(&format!("{}", "Error: failed to rmdir".red()));
        }
    } else if matches.is_present("URL") {
        //println!("{:?}", matches);
        let mut download_url = String::new();

        if !matches.is_present("git") {
            //let mut output = "";
            let arguments: Vec<String> = env::args().collect();
            let mut the_argument: Vec<&str> = Vec::new();
            println!("{}", arguments[arguments.len() - 1]);
            if arguments[arguments.len() - 1].starts_with("themefox-manager:// ") {
                the_argument = arguments[arguments.len() - 1].split(' ').collect();
            } else {
                the_argument.push("nothing");
                the_argument.push(&arguments[arguments.len() - 1]);
            }
            if the_argument[1].contains("\"") {
                panic!("{}", "The program quitted itself, because the url contained a \". Now i know this might seem stupid, but believe me, it's for the better");
            }

            if the_argument[1].starts_with("http")
                && the_argument[1].contains("://")
                && the_argument[1].contains("themefox.net")
                && the_argument[1].contains("/")
            //&& the_argument[1].ends_with(".git") || the_argument[1].ends_with(".zip")
            {
                succes("Good url");
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
                        "json again seemed to be wrong formatted... Please report this issue."
                            .red()
                    );
                }
                succes("Good response from the server");
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
                    //}
                }
            } else {
                println!("The argument you supplied didn't seem to be a correct url, or you didn't supply any url. \n Run with -h in order to see the usage");
                panic!("{}", "\n There is nothing to do. \n Quitting...".red());
            }
        } else if matches.is_present("git") {
            let arguments: Vec<String> = env::args().collect();
            println!("{}", arguments[0]);
            let mut _the_argument: Vec<&str> = Vec::new();
            _the_argument = arguments[arguments.len() - 1].split(' ').collect();
            download_url = _the_argument[0].to_string();
        }

        // fetches what operating system you use
        let os = std::env::consts::OS;
        succes("Fetched your operating system");
        // If the operating system is linux then it does everything that is in those brackets
        if os == "linux" {
            get_firefox_linux(true, matches, download_url);
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
            if native == true && !matches.is_present("path") {
                // We already had a very simillar piece of code. Try to understand it yourself :)
                complete_path.push("Library/Application Support/Firefox");
            } else {
                complete_path.push(manual_profile_path());
            }
            env::set_current_dir(complete_path).expect(&format!("{}", "Error: unable to cd".red()));
            succes("Got your firefox directory");
            find_profile(true, matches.is_present("profile"));
            download(&download_url, matches.is_present("git"));
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
            if native == true && !matches.is_present("path") {
                // We already had a very simillar piece of code. Try to understand it yourself :)
                complete_path.push("AppData\\Roaming\\Mozilla\\Firefox");
            } else {
                complete_path.push(manual_profile_path());
            }
            env::set_current_dir(complete_path).expect(&format!("{}", "Error: unable to cd".red()));
            succes("Got your firefox directory");
            find_profile(true, matches.is_present("profile"));
            download(&download_url, matches.is_present("git"));
        } else {
            eprintln!("Error: You seem to use a Operating System that is not supported. Please report this issue on github (https://github.com/alx365/Themefox-Manager)");
            panic!("{}", "Quitting...".red());
        }
    } else {
        let os = std::env::consts::OS;
        let mut path: PathBuf = PathBuf::new();
        //path.push("/usr/share/themefox");
        path.push(dirs::config_dir().expect(&format!("{}", "Failed to get your config dir".red())));
        path.push("themefox-manager");
        fs::create_dir_all(&path).expect(&format!(
            "{}",
            "Failed to make config file in the config dir".red()
        ));
        path.push("themefox-manager.txt");

        if !path.exists() {
            install(path, os);
        } else {
            print!("Bad usage.\n Have a look at the usage with the `--help` flag. ");
        }
    }
}

fn find_profile(go_chrome: bool, find_profile: bool) {
    if !find_profile {
        find_default_profile();
    } else {
        ask_for_profile();
    }

    // Now we are in the default profile, the programm now enables stylesheets, so that the theme will also be shown.
    enable_css();
    if Path::new("chrome").exists() == false {
        if go_chrome == true {
            fs::create_dir("chrome").expect("Error: failed to mkdir");
            println!("Created the chrome directory, because it didn't exist before");
        } else {
            println!("You chrome directory doesn't exist, so we can't remove it -.-");
            panic!("{}", "Quitting...".red())
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
    succes("Almost everything is finished. Now installing/uninstalling the theme");
}

fn download(file: &str, git: bool) {
    if !git {
        Command::new("curl")
            .arg("-L")
            .arg(file)
            .arg("-o")
            .arg("ChromeFiles.zip")
            .status()
            .expect(&format!(
                "{}",
                "Error: cURL failed to start. Do you have it installed?".red()
            ));
        succes("Downloaded the theme now unzipping");
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
        succes("Finished installing the theme: enjoy!");
    } else {
        let paths = fs::read_dir(".").unwrap();

        for path in paths {
            let foobar = path.unwrap().path();
            if !foobar.is_dir() {
                fs::remove_file(format!("{}", foobar.display())).expect(&format!(
                    "{}",
                    "Failed to remove all files in the chrome dir".red()
                ));
            } else {
                fs::remove_dir_all(format!("{}", foobar.display())).expect(&format!(
                    "{}",
                    "Failed to remove all files in the chrome dir".red()
                ));
            }
            //println!("Name: {:?}", path.unwrap());
        }
        Command::new("git")
            .arg("clone")
            .arg(file)
            .arg(".")
            .status()
            .expect(&format!(
                "{}",
                "Error: git failed to start. Do you have it installed?".red()
            ));
        let exceptions = ["userContent.css", "userChrome.css", "userContent.js", "userChrome.js"];
        let tabu = [".git"];
        if !Path::new("userChrome.css").exists() || !Path::new("userContent.css").exists() || !Path::new("userContent.js").exists(){
            let mut options: Vec<String> = Vec::new();
            let paths = fs::read_dir(".").unwrap();

            // zero loop
            for dir in paths {
                //println!("Zero loop");
                let name = &dir.unwrap().path();
                //println!("Found a dir: {:?}", &name);
                // First loop
                if name.is_dir() && !tabu.contains(&name.file_name().unwrap().to_str().unwrap()) {
                    //println!("{:?}", name);
                    // !after this point the recurive loops are running
                    for path in fs::read_dir(&name).unwrap() {
                        // !
                        //println!("First loop");
                        //println!("Found a dir: {:?}", &path);
                        let tmp = path.unwrap();
                        if !tmp.path().is_dir()
                            && exceptions.contains(&tmp.file_name().to_str().unwrap())
                        {
                            if !options.contains(&name.to_str().unwrap().to_string()) {
                                options.push(tmp.path().to_str().unwrap().to_string());
                            }
                        // println!("HEY");
                        } else {
                            // !
                            let name = tmp.path();
                            if tmp.path().is_dir() {
                                for path2 in fs::read_dir(&name).unwrap() {
                                    //println!("Third loop");
                                    let tmp2 = path2.unwrap();

                                    if !tmp2.path().is_dir()
                                        && exceptions.contains(&tmp2.file_name().to_str().unwrap())
                                    {
                                        if !options.contains(&name.to_str().unwrap().to_string()) {
                                            options.push(name.to_str().unwrap().to_string());
                                        }
                                    //println!("HEY");
                                    } else {
                                        // !
                                        let name = tmp2.path();
                                        if !tmp.path().is_dir() {
                                            for path3 in fs::read_dir(&name).unwrap() {
                                                let tmp3 = path3.unwrap();
                                                if !tmp3.path().is_dir()
                                                    && exceptions
                                                        .contains(&tmp3.path().to_str().unwrap())
                                                {
                                                    if !options.contains(
                                                        &name.to_str().unwrap().to_string(),
                                                    ) {
                                                        options.push(
                                                            name.to_str().unwrap().to_string(),
                                                        );
                                                    }
                                                //println!("HEY");
                                                } else {
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                } else {
                    //println!("Its a file, so it isn't important.");
                }
            }
            
            if options.len() > 0 {
            //println!("{:?}", &options);
            options.sort();

            let selection = Select::with_theme(&ColorfulTheme::default())
                .with_prompt(format!(
                    "{}",
                    "Couldn't find any files, that change the way firefox behaves, we searched 4 directories deep, to find something, here is what we found.\nPick your profile, to install into (navigate with arrow keys)".yellow()
                ))
                .default(0)
                .items(&options[..])
                .interact()
                .unwrap();
            for file in fs::read_dir(Path::new(&options[selection])).unwrap() {
                let tmp = &file.unwrap().path();
                //println!("{:?}", tmp);

                syslinks(&tmp);

                //fs::link(tmp, tmp.file_name().unwrap()).expect("Failed to create systemlink");
            }
        } else {
            println!("{}", "Warning: The file doesn't have any files, that change the way firefox looks/behave. Unfortunately we couldn't find anything in the subdirectories".yellow())
        }
        }
    }
}

#[cfg(target_os = "linux")]
fn syslinks(tmp: &std::path::PathBuf) {
    std::os::unix::fs::symlink(tmp, tmp.file_name().unwrap()).expect("Failed to create systemlink");
}
#[cfg(target_os = "windows")]
fn syslinks() {
    if tmp.is_dir(tmp: &std::path::PathBuf) {
        std::os::windows::fs::symlink_dir(tmp, tmp.file_name().unwrap());

        std::os::windows::fs::symlink_file(tmp, tmp.file_name().unwrap());
    }
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

fn install(path: PathBuf, os: &str) {
    println!("Performing first time setup and installing, configuring stuff, so that this application will work.");
    File::create(path).expect(&format!("{}", "Failed to make config directory".red()));
    //file.write_all(b"DO NOT DELETE THIS FILE, IF YOU SHOULD DELETE IS, IT WILL ON THE NEXT STARTUP, WITHOUT ANY ARGUMENTS, TRY TO INSTALL THE CUSTOM PROTOCOL HANDLERS").expect(&format!("{}", "Error: Failed to write to config file".red()))
    if os == "linux" {
        File::create("/home/legendofmiracles/.local/bin/themefox-manager").expect(&format!(
            "{}",
            "Error: failed to create file in /.local/bin. Got the right perms?".red()
        ));
        fs::copy(std::env::current_exe().unwrap(), "/home/legendofmiracles/.local/bin/themefox-manager").expect(&format!("{}", "Failed to copy executable content to the executable in the /usr/bin directory.\n Do i have the permissions for this executable?".red()));

        /*fs::remove_file(std::env::current_exe().unwrap()).expect(&format!(
            "{}",
            "Error: An error occured when deleteing this executable.".red()
        ));*/
        Command::new("curl")
                .arg("-L")
                .arg("https://github.com/alx365/Themefox-Manager/releases/download/v0.9.9.9/stdin-themefox-manager")
                .arg("-o")
                .arg("/home/legendofmiracles/.local/bin/stdin-themefox-manager")
                .status()
                .expect(&format!("{}", "Error: sudo and/or curl failed to spawn".red()));
        Command::new("curl")
                .arg("https://raw.githubusercontent.com/alx365/Themefox-Manager/master/files/themefox-manager.json")
                .arg("-o")
                .arg("/home/legendofmiracles/.mozilla/native-messaging-hosts/themefox_manager.json")
                .status()
                .expect(&format!("{}", "Error: curl failed to complete".red()));

        Command::new("chmod")
            .arg("+x")
            .arg("/home/legendofmiracles/.local/bin/stdin-themefox-manager")
            .arg("/home/legendofmiracles/.local/bin/themefox-manager")
            .status()
            .expect(&format!(
                "{}",
                "Error: sudo and/or chmod failed to complete"
            ));

        if Confirm::with_theme(&ColorfulTheme::default())
            .with_prompt("Do you want to install the browser addon now?")
            .interact()
            .unwrap()
        {
            println!("You will have to press the add to firefox button");
            Command::new("firefox")
                .arg(" --new-tab ")
                .arg("https://addons.mozilla.org/en-US/firefox/addon/themefox-manager/")
                .status()
                .expect(&format!("{}", "firefox failed to spawn".red()));
        }

        /*
        Command::new("xdg-mime")
            .arg("default")
            .arg("/usr/share/applications/themefox-manager.desktop")
            .arg("x-scheme-handler/themefox-manager")
            .status()
            .expect(&format!(
                "{}",
                "Error: sudo and/or xdg-mime failed to spawn".red()
            ));

        Command::new("update-desktop-database")
            .status()
            .expect(&format!(
                "{}",
                "Error: update-desktop-database failed to spawn".red()
            ));
            */
        succes("Finished installing Enjoy!");
    } else if os == "windows" {
        fs::create_dir_all ("C:\\Program Files\\themefox\\").expect(&format!(
                    "{}",
                    "Error: failed to create folder in C:\\Program Files\\themefox. Did you run this with elevate permissions?".red()
                ));

        File::create("C:\\Program Files\\themefox\\themefox-manager.exe").expect(&format!(
                    "{}",
                    "Error: failed to create file in C:\\Program Files\\themefox\\themefox-manager.exe. Did you run this with elevate permissions?".red()
                ));
        fs::copy(std::env::current_exe().unwrap(), "C:\\Program Files\\themefox\\themefox-manager.exe").expect(&format!("{}", "Failed to copy executable content to the executable in the C:\\Program Files\\themefox\\themefox-manager.exe directory.\nDo i have the permissions for this executable?".red()));
        Command::new("curl")
        .arg("https://raw.githubusercontent.com/alx365/Themefox-Manager/master/files/protocol-handler.reg")
        .arg("-o")
        .arg("C:\\Program Files\\themefox\\themefox-manager.reg")
        .status()
        .expect(&format!("{}", "Error: Failed to run the curl command, do you have it installed?".red()));
        //Command::new("reg").arg("import").arg("C:\\Program Files\\themefox\\themefox-manager.reg").status().expect(&format!("{}", "failed to run the red command. Or there was an error, because this shell wasn't launched with elevated permissions"));
    }
}

fn succes(msg: &str) {
    println!("{}", format!("✔ {}", &msg).green());
}

fn enable_css() {
    // This asssumes that you already are in the profile directory
    let file = PathBuf::from("user.js");
    //let option = ""
    if !file.is_file() {
        let mut file =
            File::create(file).expect(&format!("{}", "failed to make user.js file".red()));
        file.write_all(
            b"user_pref(\"toolkit.legacyUserProfileCustomizations.stylesheets\", true); ",
        )
        .expect(&format!("{}", "Failed to write to user.js file".red()));
        succes("Enabled stylesheets in your browsers settings");
    } else {
        //println!("Beep");
        let mut file = File::open(file).expect(&"Failed to open user.js".red());
        let mut contents = String::new();
        file.read_to_string(&mut contents)
            .expect(&"Failed to read the contents of the user.js file".red());
        if contents.contains("\"toolkit.legacyUserProfileCustomizations.stylesheets\", true") {
            succes("You already had the stylesheet option enabled")
        } else {
            let mut file = OpenOptions::new()
                .append(true)
                .write(true)
                .open("user.js")
                .unwrap();
            if let Err(e) = writeln!(
                file,
                "\nuser_pref(\"toolkit.legacyUserProfileCustomizations.stylesheets\", true); "
            ) {
                eprintln!("Couldn't write to file: {}", e);
            }
            succes("Enabled stylesheets in your browsers settings");
        }
    }
}

fn get_firefox_linux(reset: bool, matches: clap::ArgMatches, download_url: String) {
    // It gets your home directory
    let home_dir: PathBuf = dirs::home_dir().unwrap();
    // It changes the directory in which it is being executed to the previously set variable (in this case it is the homedir)
    env::set_current_dir(home_dir).expect("Error: failed to cd");
    // The next part is that the program tries to understand with which package manager you have firefox installed
    // The native package manager installs the config files of firefox to /home/USER/.mozilla/firefox
    let native = Path::new(".mozilla/firefox").exists();
    // The snap one to /home/USER/snap.firefox/common/,mozilla/firefox
    let snap = Path::new("snap/firefox/common/.mozilla/firefox").exists();
    // Makes a new variable
    let mut complete_path = PathBuf::new();
    // checks If native is true, which is being set to true/false further up
    if native == true && !matches.is_present("path") {
        // Prints the message
        //println!("You have firefox installed via the native package manager");
        // We already had a very simillar piece of code. Try to understand it yourself :)
        complete_path.push(".mozilla/firefox");

    // Checks if the variable that determines if firefox was installed via snap is true
    } else if snap == true && !matches.is_present("path") {
        //println!("You have firefox installed via the snap package manager");
        complete_path.push("snap/firefox/common/.mozilla/firefox");
    } else {
        complete_path.push(manual_profile_path());
    }
    succes("Got your firefox directory");
    env::set_current_dir(complete_path).expect(&format!("{}", "Error: unable to cd".red()));
    find_profile(reset, matches.is_present("profile"));

    if reset {
        download(&download_url, matches.is_present("git"));
    } else {
        fs::remove_dir_all("chrome").expect(&format!("{}", "Error: failed to rmdir".red()));
    }
}

fn find_default_profile() {
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
    succes("Found your default profile");
    //println!("{}", contents);
    let v: Vec<&str> = contents
        .split(|c| c == '=' || c == ']' || c == '\n')
        .collect();

    default_profile = v[3].to_string();

    if !default_profile.contains(".") {
        println!("{}", "You seem to be using a very old firefox version. Consider updating. \n We do not support such old versions. \nIf you want, you can try again with the --profile flag".red());
        panic!("Quitting...".red());
    }
    let default_profile_path: Vec<&str> = default_profile.split('/').collect();
    let mut new_path = PathBuf::new();
    for el in &default_profile_path {
        new_path.push(el.trim_end());
    }
    //println!("{:?}", new_path);
    env::set_current_dir(new_path).expect(&format!(
        "{}",
        "failed to cd. \n Please report this issue on GitHub".red()
    ));
}

fn ask_for_profile() {
    let mut options: Vec<String> = Vec::new();
    let paths = fs::read_dir(".").unwrap();
    let exceptions = ["Pending Pings", "Crash Reports", "Caches", ".mozilla"];

    for path in paths {
        let tmp = path.unwrap();
        if tmp.path().is_dir() && !exceptions.contains(&tmp.file_name().to_str().unwrap()) {
            options.push(tmp.file_name().to_str().unwrap().to_string());
        }
    }
    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt(format!(
            "{}",
            "Pick your profile, to install into (navigate with arrow keys)".yellow()
        ))
        .default(0)
        .items(&options[..])
        .interact()
        .unwrap();
    env::set_current_dir(PathBuf::from(&options[selection])).unwrap();
}
