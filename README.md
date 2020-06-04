# Themefox-Manager
![Rust](https://github.com/alx365/Themefox-Manager/workflows/Rust/badge.svg?branch=master)
[![works badge](https://cdn.jsdelivr.net/gh/nikku/works-on-my-machine@v0.2.0/badge.svg)](https://github.com/nikku/works-on-my-machine)[![Open Source Love](https://badges.frapsoft.com/os/v1/open-source.png?v=103)](https://github.com/ellerbrock/open-source-badges/)

[![forthebadge](https://forthebadge.com/images/badges/fuck-it-ship-it.svg)](https://forthebadge.com) [![forthebadge](https://forthebadge.com/images/badges/you-didnt-ask-for-this.svg)](https://forthebadge.com)

[![HitCount](http://hits.dwyl.com/themefox/Themefox-Manager.svg)](http://hits.dwyl.com/themefox/Themefox-Manager) [![Maintenance](https://img.shields.io/badge/Maintained%3F-yes-green.svg)](https://GitHub.com/Naereen/StrapDown.js/graphs/commit-activity)![Works](https://img.shields.io/badge/May%20not%20work-on%20your%20machine-red)





This is my first project in rust, so please excuse the terrible code quality and code logic.

A minimal and super fast theme manager for Firefox written in 100% rust.


## Usage
```bash
Installs themes to your firefox, from a valid themefox url, or git url

USAGE:
    themefox-manager [FLAGS] [URL]

FLAGS:
    -g, --git        Installs from git repo, must be specified in a full URL. For example:
                     https://githost.domain/foo/bar.git. Will remove all other files in the dir
    -h, --help       Prints help information
    -p, --path       Sets the path to install to, will automaticly trigger if no path is being found
        --reset      Resets firefox theme by deleting all chrome files
    -V, --version    Prints version information

ARGS:
    <URL>    Sets the URL to install from
```

## Coming features: 

    - Very close support with the themefox website (yet to be released)
    
    - A addon, to manage the themes, running on native messaging.

## Already implemented features:

    - Git support with the `--git` flag
    
    - Downloading and installing (would work if the website would be up)
      
    - Mac, GNU/Linux support and Windows support (Mac builds rather less, since i can't compile them on my main machine)
    
    - A flag for resetting the firefox themes and make firefox look normal
      
    - Automatically enables toolkit.legacyUserProfileCustomizations.stylesheets, in the settings.
This program runs on curl/git, so make sure you have that installed, if you want to use that.
You can install git from here: https://git-scm.com/book/en/v2/Getting-Started-Installing-Git.
Curl is installed on windows by default.

## Installing

There are several ways to install themefox-manager, via the crates.io index, the binaries that you find here and if you are a linux user, maybe even in your package manager. 

Note that installing from crates.io/package manager is the best way, since you get updates through that. There is currently no update support planned for binaries downloded from the web.

### crates.io

​	Install `cargo`, and then run `cargo install themefox-manager`, to install it from crates.io.

​	Then to install it, run the program once, with administrator/root priviliges, without any args.

### Package manager

​	AUR version will be released, once the version is in 1.0

### Binaries online

​	Download a binary, for your OS, and run it once without any arguments. Note that root/administator privileges are required for the install.



## New features: 

Please leave a pull request, if you want to have new features added.



## Compiling from Source (Not recommended, only to this if you have another architecture)
This branch is development, so it will be buggy.

  1. Install rust (best with rustup, see https://www.rust-lang.org/learn/get-started), and cargo (comes installed with rustup)
  2. Clone this project, and then go to the root of the project
  3. Build it with `cargo build --release`
  4. Enjoy!



## Issues (Since there aren't any issues with this, there actually shouldn't be any need for this :))

  If you experience any issues (i hope not), please leave a issue, on the issues tab, on the top of the page.

  

## Contributing:

I am open to contributions. Please feel free to creaete a pull request and somewhat describe what you changed. 
