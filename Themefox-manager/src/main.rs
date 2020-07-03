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
    let os = std::env::consts::OS;
    // The ascii art message
    let message = r#"
    ______  __  __  ___  __   __  ___   ___    __   ___  __       __  _    __    __  _    __     __   ___   ___
    |_   _| | || | | __| |  V  | | __| | __|  /__\  \ \_/ /  __  |  V  |  /  \  |  \| |  /  \   / _| | __| | _ \ 
      | |   | >< | | _|  | \_/ | | _|  | _|  | \/ |  > , <  |__| | \_/ | | /\ | | | ' | | /\ | | |/\ | _|  | v / 
      |_|   |_||_| |___| |_| |_| |___| |_|    \__/  /_/ \_\      |_| |_| |_||_| |_|\__| |_||_|  \__/ |___| |_|_\ 
     "#;
    // prints it
    print!("{}\n", message.bright_red());
    let mut app = App::new("themefox-manager")
        .name("themefox-manager")
        .version("v0.9.11")
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
        .arg(
            Arg::with_name("profile")
            .long("profile")
            .short("p")
            .help("This argument lets you chose which profile you want to install to")
        )
        .arg(
            Arg::with_name("addon")
            .long("addon")
            .short("a")
            .help("This argument installs the files on client side, for the browser addon to work.")
        );
    if os == "linux" {
        let mut shell_path = PathBuf::new();
        let extension = ".fish";
        shell_path.push(dirs::config_dir().unwrap());
        shell_path.push("fish/completions");
        fs::create_dir_all(&shell_path).expect(&format!(
            "{}",
            "Failed to create the fish autocomplete dir".red()
        ));
        shell_path.push(format!("themefox-manager{}", extension));

        let mut completion = File::create(shell_path)
            .expect(&format!("{}", "Failed to make shell completion".red()));

        app.gen_completions_to("themefox-manager", clap::Shell::Fish, &mut completion);
    } else if os == "windows" {
        app.gen_completions_to(
            "themefox-manager",
            clap::Shell::PowerShell,
            &mut io::stdout(),
        );
    }

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
        // i know we already fetched it way earlier, but the user doesn't know that :)
        succes("Fetched your operating system");
        if os == "linux" {
            get_firefox_linux(true, matches, "null".to_string())
        } else if os == "macos" {
            firefox_dir(&matches);
            find_profile(false, matches.is_present("profile"));
            fs::remove_dir_all("chrome").expect(&format!("{}", "Error: failed to rmdir".red()));
        } else if os == "windows" {
            firefox_dir(&matches);
            env::set_current_dir("firefox").expect(&format!(
                "{}",
                "failed to cd into the firefox dir in the firefox dir".red()
            ));
            find_profile(false, matches.is_present("profile"));
            fs::remove_dir_all("chrome").expect(&format!("{}", "Error: failed to rmdir".red()));
        }
    } else if matches.is_present("URL") {
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

            if the_argument[1].starts_with("http")
                && the_argument[1].contains("://")
                && the_argument[1].contains("themefox.net")
                && the_argument[1].contains("/")
            //&& the_argument[1].ends_with(".git") || the_argument[1].ends_with(".zip")
            {
                succes("Good url");
                let id: Vec<&str> = the_argument[1].split('/').collect();

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
                println!("The argument you supplied didn't seem to be a correct url, or you didn't supply any url. \nRun with -h in order to see the usage");
                panic!("{}", "\nThere is nothing to do. \nQuitting...".red());
            }
        } else if matches.is_present("git") {
            let arguments: Vec<String> = env::args().collect();
            let mut _the_argument: Vec<&str> = Vec::new();
            _the_argument = arguments[arguments.len() - 1].split(' ').collect();
            download_url = _the_argument[0].to_string();
        }

        // fetches what operating system you use
        //let os = std::env::consts::OS;
        succes("Fetched your operating system");
        // If the operating system is linux then it does everything that is in those brackets
        if os == "linux" {
            get_firefox_linux(true, matches, download_url);
        } else if os == "macos" {
            firefox_dir(&matches);
            find_profile(true, matches.is_present("profile"));
            download(&download_url, matches.is_present("git"));
        } else if os == "windows" {
            firefox_dir(&matches);
            env::set_current_dir("firefox").expect(&format!(
                "{}",
                "failed to cd into the firefox dir in the firefox dir".red()
            ));
            find_profile(true, matches.is_present("profile"));
            download(&download_url, matches.is_present("git"));
        } else {
            eprintln!("Error: You seem to use a Operating System that is not supported. Please report this issue on github (https://github.com/alx365/Themefox-Manager)");
            panic!("{}", "Quitting...".red());
        }
    } else if matches.is_present("addon") {
        let os = std::env::consts::OS;
        install(os, matches);
    } else {
        print!(
            "Bad usage.\nHave a look at the usage with the '{}' flag. ",
            "--help".green()
        );
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
            println!("Your chrome directory doesn't exist, so we can't remove it -.-");
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
        // ptr = path to repo
        // download git only suceeds if there are no errors, so we can be positive that the ptr has the files we need.
        let ptr = download_git(file);
        // Then we remove everything in the current dir
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
        }
        recursive(&PathBuf::from(ptr)).expect("TEST");

        // The program looks if two key files exist, in the download, if not it proceeds
        if !Path::new("userChrome.css").exists() || !Path::new("userContent.css").exists() {
            let exceptions = [
                "userContent.css",
                "userChrome.css",
                "userContent.js",
                "userChrome.js",
            ];
            let tabu = [".git"];
            let mut options: Vec<String> = Vec::new();
            let paths = fs::read_dir(".").unwrap();
            // for thing in this directory
            // zero loop
            for dir in paths {
                // sets name to the thing in the directry
                let name = &dir.unwrap().path();
                // First loop
                // Checks if name is a directory and if its not an exception like .git
                if name.is_dir() && !tabu.contains(&name.file_name().unwrap().to_str().unwrap()) {
                    // !after this point the recurive loops are running
                    // Checks the content of the dir in the current dir
                    for path in fs::read_dir(&name).unwrap() {
                        // !
                        let tmp = path.unwrap();
                        // Checks that its not a dir and if it already exists in the exceptions variable, so that only the right files can pass
                        if !tmp.path().is_dir()
                            && exceptions.contains(&tmp.file_name().to_str().unwrap())
                        {
                            // Then it checks if it already exists in the vec
                            if !options.contains(&name.to_str().unwrap().to_string()) {
                                // Adds the dir to the vec
                                options.push(tmp.path().to_str().unwrap().to_string());
                            }
                        // println!("HEY");
                        } else {
                            // !
                            // sets name to the last dir we were in
                            let name = tmp.path();
                            // Checks if it is a dir, because we don't want to visit files
                            if tmp.path().is_dir() {
                                //reads every file in the dir
                                println!("{:?}", name);
                                for path2 in fs::read_dir(&name).unwrap() {
                                    let tmp2 = path2.unwrap();

                                    if !tmp2.path().is_dir()
                                        && exceptions.contains(&tmp2.file_name().to_str().unwrap())
                                    {
                                        if !options.contains(&name.to_str().unwrap().to_string()) {
                                            options.push(name.to_str().unwrap().to_string());
                                        }
                                    } else {
                                        // !
                                        let name = tmp2.path();
                                        println!("{:?}", name);
                                        if tmp2.path().is_dir() {
                                            for path3 in fs::read_dir(&name).unwrap() {
                                                let tmp3 = path3.unwrap();
                                                if !tmp3.path().is_dir()
                                                    && exceptions.contains(
                                                        &tmp3.file_name().to_str().unwrap(),
                                                    )
                                                {
                                                    if !options.contains(
                                                        &name.to_str().unwrap().to_string(),
                                                    ) {
                                                        options.push(
                                                            name.to_str().unwrap().to_string(),
                                                        );
                                                    }
                                                } else {
                                                    // !
                                                    let name = tmp3.path();

                                                    println!("{:?}", name);
                                                    if tmp3.path().is_dir() {
                                                        for path4 in fs::read_dir(&name).unwrap() {
                                                            let tmp4 = path4.unwrap();
                                                            if !tmp4.path().is_dir()
                                                                && exceptions.contains(
                                                                    &tmp4
                                                                        .file_name()
                                                                        .to_str()
                                                                        .unwrap(),
                                                                )
                                                            {
                                                                if !options.contains(
                                                                    &name
                                                                        .to_str()
                                                                        .unwrap()
                                                                        .to_string(),
                                                                ) {
                                                                    options.push(
                                                                        name.to_str()
                                                                            .unwrap()
                                                                            .to_string(),
                                                                    );
                                                                }
                                                            } else {
                                                                // !
                                                                let name = tmp4.path();
                                                                println!("{:?}", name);
                                                                if tmp4.path().is_dir() {
                                                                    for path5 in
                                                                        fs::read_dir(&name).unwrap()
                                                                    {
                                                                        let tmp5 = path5.unwrap();
                                                                        if !tmp5.path().is_dir()
                                                                            && exceptions.contains(
                                                                                &tmp5
                                                                                    .file_name()
                                                                                    .to_str()
                                                                                    .unwrap(),
                                                                            )
                                                                        {
                                                                            if !options.contains(
                                                                                &name
                                                                                    .to_str()
                                                                                    .unwrap()
                                                                                    .to_string(),
                                                                            ) {
                                                                                options.push(
                                                                                    name.to_str()
                                                                                        .unwrap()
                                                                                        .to_string(
                                                                                        ),
                                                                                );
                                                                            }
                                                                        } else {
                                                                            // !
                                                                            let name = tmp5.path();
                                                                            println!("{:?}", name);
                                                                            if tmp5.path().is_dir()
                                                                            {
                                                                                for path6 in
                                                                                    fs::read_dir(
                                                                                        &name,
                                                                                    )
                                                                                    .unwrap()
                                                                                {
                                                                                    let tmp6 =
                                                                                        path6
                                                                                            .unwrap(
                                                                                            );
                                                                                    if !tmp6.path().is_dir()
                                                                            && exceptions.contains(
                                                                                &tmp6
                                                                                    .file_name()
                                                                                    .to_str()
                                                                                    .unwrap(),
                                                                            )
                                                                        {
                                                                            if !options.contains(
                                                                                &name
                                                                                    .to_str()
                                                                                    .unwrap()
                                                                                    .to_string(),
                                                                            ) {
                                                                                options.push(
                                                                                    name.to_str()
                                                                                        .unwrap()
                                                                                        .to_string(
                                                                                        ),
                                                                                );
                                                                            }
                                                                        }
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }

            if options.len() > 0 {
                options.sort();

                let selection = Select::with_theme(&ColorfulTheme::default())
                .with_prompt(format!(
                    "{}",
                    "Couldn't find any files, that change the way firefox behaves, we searched 7 directories deep, to find something, here is what we found.\nPick your profile, to install into (navigate with arrow keys)".yellow()
                ))
                .default(0)
                .items(&options[..])
                .interact()
                .unwrap();
                for file in fs::read_dir(Path::new(&options[selection])).unwrap() {
                    let tmp = &file.unwrap().path();

                    syslinks(&tmp);
                }
            } else {
                println!("{}", "Warning: The file doesn't have any files, that change the way firefox looks/behave. Unfortunately we couldn't find anything in the subdirectories".yellow())
            }
        }
    }
}

fn recursive(dir: &PathBuf) -> io::Result<()> {
    //println!("{:?}", dir);
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let foobar = entry.path();
        let mut path = PathBuf::new();
        // Set this to i8, since we don't need a high number. something is definitly wrong, and it will panic
        let mut run: i8 = 0;
        for i in foobar.components() {
            run += 1;
            if run > 3 {
                println!("{:?} parents {}", i, run);
                path.push(i);
            }
        }
        //println!("{:?}", path);
        let name = path.file_name().unwrap();
        if foobar.is_dir() {
            fs::create_dir(name).expect(&format!(
                "{}{:?}",
                "Failed to copy contents from tmp dir".red(),
                name
            ));
            recursive(&path)?;
        } else {
            let mut file = File::create(path.file_name().unwrap())
                .expect(&format!("{}", "Failed to copy contents from tmp dir".red()));
            //fs::copy(&path, name).expect(&format!("{}", "Failed to copy contents from tmp dir".red()));;
            io::copy(
                &mut File::open(path)
                    .expect(&format!("{}", "Failed to copy contents from tmp dir".red())),
                &mut file,
            )
            .expect(&format!("{}", "Failed to copy contents from tmp dir".red()));
        }
    }

    Ok(())
}

#[cfg(target_os = "linux")]
fn download_git(file: &str) -> &str {
    fs::create_dir_all("/tmp/chrome").expect(&format!("{}", "Failed to create the tmmp dir".red()));
    Command::new("git")
        .arg("clone")
        .arg(file)
        .arg("/tmp/chrome/")
        .status()
        .expect(&format!(
            "{}",
            "Error: git failed to complete. Do you have it installed?".red()
        ));
    return "/tmp/chrome/";
}

#[cfg(target_os = "windows")]
fn download_git(file: &str) {
    use git2::Repository;
    let _repo = match Repository::clone(file, ".") {
        Ok(repo) => repo,
        Err(e) => panic!("failed to clone: {}", e),
    };
}

#[cfg(target_os = "macos")]
fn download_git(file: &str) {
    use git2::Repository;
    let _repo = match Repository::clone(file, ".") {
        Ok(repo) => repo,
        Err(e) => panic!("failed to clone: {}", e),
    };
}

#[cfg(unix)]
fn syslinks(tmp: &std::path::PathBuf) {
    std::os::unix::fs::symlink(tmp, tmp.file_name().unwrap()).expect("Failed to create systemlink");
}
#[cfg(windows)]
fn syslinks(tmp: &std::path::PathBuf) {
    if tmp.is_dir() {
        std::os::windows::fs::symlink_dir(tmp, tmp.file_name().unwrap())
            .expect("Failed to create syslinks");

        std::os::windows::fs::symlink_file(tmp, tmp.file_name().unwrap())
            .expect("Failed to create syslinks");
    }
}
fn manual_profile_path() -> String {
    eprintln!("Error: We can not seem to find your firefox folder. \nIf you ran this application with elevated permissions, please try again without. \nYou can find your profile folder by typing about:profiles in the adress bar and then select the button open directory on the first one. Then navigate back one directory and thats the path you should enter\n" );
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

fn install(os: &str, matches: clap::ArgMatches) {
    println!("Performing first time setup and installing, configuring stuff, so that this application will work.");
    if os == "linux" {
        firefox_dir(&matches);
    } else if os == "macos" {
        env::set_current_dir(dirs::home_dir().unwrap())
            .expect(&format!("{}", "failed to cd into home dir"));
        env::set_current_dir("Library/Application Support/Mozilla")
            .expect(&format!("{}", "failed to cd into mozilla dir dir"));
    }
    let mut name = "native-messaging-hosts";
    if os == "macos" {
        name = "NativeMessagingHosts";
    }
    if fs::create_dir(name).is_err() {
        if !Path::new(name).exists() {
            panic!("Failed to mkdir the native messaging dir in firefox dir, do we have enough permissions?".red())
        } else {
            println!("You already had the native-messaging-hosts directory.")
        }
    }

    env::set_current_dir(name).expect(&format!("{}", "Failed changing dir".red()));

    let file = Command::new("curl")
                .arg("https://raw.githubusercontent.com/alx365/Themefox-Manager/master/files/themefox-manager.json")
                //.arg("-o")
                //.arg("themefox_manager.json")
                .output()
                .expect(&format!("{}", "Error: curl failed to complete".red()));
    let mut user = "F U Windows";
    if os == "linux" {
        user = "/home";
    } else if os == "macos" {
        user = "/Users";
    }
    let output = str::from_utf8(&file.stdout).unwrap().replace(
        "$USER",
        &format!("{}/{}", user, &std::env::var("USER").unwrap()),
    );
    fs::File::create("themefox_manager.json")
        .expect(&format!(
            "{}",
            "Error: failed creating the themefox_manager.json file".red()
        ))
        .write_all(output.as_bytes())
        .expect(&format!(
            "{}",
            "Error: failed to write to json config firefox file"
        ));
    install_helper(os);

    if Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt("Do you want to install the browser addon now?")
        .interact()
        .unwrap()
    {
        println!("You will have to press the add to firefox button");
        if os == "macos" {
            Command::new("open")
                .arg("https://addons.mozilla.org/en-US/firefox/addon/themefox-manager/")
                .status()
                .expect(&format!("{}", "\'open\' failed to spawn".red()));
        } else if os == "linux" {
            Command::new("firefox")
                .arg("--new-tab")
                .arg("https://addons.mozilla.org/en-US/firefox/addon/themefox-manager/")
                .status()
                .expect(&format!("{}", "\'firefox\' failed to spawn".red()));
        }
    }
    succes("Finished installing Enjoy!");
}
//#[cfg(linux)]
fn install_helper(os: &str) {
    env::set_current_dir(dirs::home_dir().unwrap()).expect(&format!(
        "{}",
        "failed to cd into the homdir in the helper function".red()
    ));
    //let dir
    if fs::create_dir_all(".local/bin").is_err() {
        if !Path::new(".local/bin").exists() {
            panic!(
                "Couldn't create the .local/bin directory, do we have enough permissions?".red()
            );
        } else {
            println!("You already had the .local/bin directory")
        }
    }
    let mut url = "error";
    if os == "linux" {
        url = "https://github.com/alx365/Themefox-Manager/releases/download/v0.9.9.9/stdin-themefox-manager"
    } else if os == "macos" {
        url = "https://github.com/alx365/Themefox-Manager/releases/download/v0.9.9.9/stdin-themefox-manager-mac";
    }
    Command::new("curl")
        .arg("-L")
        .arg(format!("{}", url))
        .arg("-o")
        .arg(".local/bin/stdin-themefox-manager")
        //.output()
        .status()
        .expect(&format!("{}", "Error: curl failed to spawn".red()));

    if os == "linux" || os == "macos" {
        Command::new("chmod")
            .arg("+x")
            .arg(".local/bin/stdin-themefox-manager")
            .status()
            .expect(&format!("{}", "Error: chmod failed to complete"));
    }
}

fn succes(msg: &str) {
    println!("{}", format!("✔  {}", &msg).green());
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
                eprintln!(
                    "You will have to enable them manually, if you do not yet have them enabled."
                );
            } else {
                succes("Enabled stylesheets in your browsers settings");
            }
        }
    }
}

fn get_firefox_linux(reset: bool, matches: clap::ArgMatches, download_url: String) {
    firefox_dir(&matches);
    env::set_current_dir("firefox")
        .expect(&format!("{}", "failed to cd into the firefox dir".red()));

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
        println!("Error: We cannot find your last used or your default profile. because the file is missing, with which we can find out.\nPlease report this issue on github (https://github.com/alx365/Themefox-Manager)");
        panic!("{}", "Quitting...".red());
    }
    succes("Found your default profile");
    let v: Vec<&str> = contents
        .split(|c| c == '=' || c == ']' || c == '\n')
        .collect();

    default_profile = v[3].to_string();

    if !default_profile.contains(".") {
        println!("{}", "You seem to be using a very old firefox version. Consider updating. \nWe do not support such old versions. \nIf you want, you can try again with the --profile flag".red());
        panic!("Quitting...".red());
    }
    let default_profile_path: Vec<&str> = default_profile.split('/').collect();
    let mut new_path = PathBuf::new();
    for el in &default_profile_path {
        new_path.push(el.trim_end());
    }
    env::set_current_dir(new_path).expect(&format!(
        "{}",
        "failed to cd. \nPlease report this issue on GitHub".red()
    ));
}

fn ask_for_profile() {
    let mut options: Vec<String> = Vec::new();
    let exceptions = ["Pending Pings", "Crash Reports", "Caches", ".mozilla"];

    if env::consts::OS == "macos" || env::consts::OS == "windows" {
        env::set_current_dir("Profiles").expect(&format!(
            "{}",
            "Failed to cd into the Profiles dir (windows macos)".red()
        ));
    }
    let paths = fs::read_dir(".").unwrap();
    for path in paths {
        let tmp = path.unwrap();
        if tmp.path().is_dir() && !exceptions.contains(&tmp.file_name().to_str().unwrap()) {
            options.push(tmp.file_name().to_str().unwrap().to_string());
        }
    }
    options.sort();
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

fn firefox_dir(matches: &clap::ArgMatches) {
    let os = env::consts::OS;

    // It gets your home directory
    let home_dir: PathBuf = dirs::home_dir().unwrap();
    // It changes the directory in which it is being executed to the previously set variable (in this case it is the homedir)
    env::set_current_dir(home_dir).expect("Error: failed to cd");
    // Makes a new variable
    let mut complete_path = PathBuf::new();
    if os == "linux" {
        // The next part is that the program tries to understand with which package manager you have firefox installed
        // The native package manager installs the config files of firefox to /home/USER/.mozilla/firefox
        let native = Path::new(".mozilla/").exists();
        // The snap one to /home/USER/snap.firefox/common/.mozilla/firefox
        let snap = Path::new("snap/firefox/common/.mozilla/").exists();
        // checks If native is true, which is being set to true/false further up
        if native == true && !matches.is_present("path") {
            complete_path.push(".mozilla/");

        // Checks if the variable that determines if firefox was installed via snap is true
        } else if snap == true && !matches.is_present("path") {
            complete_path.push("snap/firefox/common/.mozilla/");
        } else {
            complete_path.push(manual_profile_path());
        }
    } else if os == "macos" {
        let native = Path::new("Library/Application Support/Firefox/Profiles").exists();
        if native == true && !matches.is_present("path") {
            complete_path.push("Library/Application Support/Firefox");
        } else {
            complete_path.push(manual_profile_path());
        }
    } else if os == "windows" {
        let native = Path::new("AppData\\Roaming\\Mozilla\\Firefox\\Profiles").exists();
        // checks If native is true, which is being set to true/false further up
        if native == true && !matches.is_present("path") {
            complete_path.push("AppData\\Roaming\\Mozilla\\Firefox");
        } else {
            complete_path.push(manual_profile_path());
        }
    }

    succes("Got your firefox directory");
    env::set_current_dir(complete_path).expect(&format!("{}", "Error: unable to cd".red()));
}
