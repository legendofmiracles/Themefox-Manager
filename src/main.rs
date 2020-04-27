extern crate dirs;
use std::env;
use std::path::Path;
use std::ffi::OsString;

fn main() {
    //let path = dirs::config_dir();

    let os = std::env::consts::OS;

    if os == "linux" {
        let path = dirs::home_dir();
        env::set_current_dir(path);
        env::set_current_dir("/.mozilla/firefox");
    }
}