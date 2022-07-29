use crate::prefix;
use colored::Colorize;
use std::fs::File;
use std::io::prelude::*;
use std::process::Command;

/// Holds the entire fetch information.
pub struct Entry {
    username: String,
    hostname: String,
    banner: String,
    distro: String,
    package_count: String,
}

pub enum EntryType {
    User, // ex. anon@pc
    PackageCount,
    Distro,
}

impl Entry {
    pub fn new() -> Self {
        Self {
            package_count: read_package_num(),
            username: read_username(),
            banner: read_banner(),
            hostname: read_hostname(),
            distro: read_distro(),
        }
    }

    pub fn get_string_user(&self) -> String {
        format!(
            "\t\t{}{}@{}",
            prefix::get_prefix(EntryType::User),
            self.username,
            self.hostname,
        )
        .yellow()
        .bold()
        .italic()
        .to_string()
    }

    pub fn get_string_distro(&self) -> String {
        format!(
            "\t{} {} {}",
            prefix::get_prefix(EntryType::Distro),
            "DISTRO:".green().bold(),
            self.distro
        )
    }

    pub fn get_string_package_count(&self) -> String {
        format!(
            "\t{} {} {}",
            prefix::get_prefix(EntryType::PackageCount),
            "PKGS:".green().bold(),
            self.package_count,
        )
    }

    pub fn get_string_banner(&self) -> String {
        format!("{}", self.banner)
    }
}

/// Finds what kind of distro the user
/// is currently using by reading the
/// file called /etc/os-release.
fn read_distro() -> String {
    let mut file = match File::open("/etc/os-release") {
        Ok(f) => f,
        Err(e) => panic!("{e}"),
    };
    let mut contents = String::new();

    match file.read_to_string(&mut contents) {
        Ok(_) => {}
        Err(e) => panic!("{e}"),
    };

    find_pretty_name(contents)
}

/// In tandem with the function read_distro(),
/// this function takes the contents of /etc/os-release
/// and narrows down the line with the name
/// of the distro and returns it.
fn find_pretty_name(s: String) -> String {
    let line = s
        .lines()
        .filter(|l| l.find("PRETTY").is_some())
        .collect::<Vec<&str>>();
    let line = match line.iter().next() {
        Some(s) => s,
        None => panic!("Encountered some problems while finding hostname!"),
    };

    line.rsplit('=')
        .next()
        .unwrap()
        .to_string()
        .replace("\"", "")
}

/// Runs and gets the output of the command
/// `hostname` and then returns it.
fn read_hostname() -> String {
    let hostname = Command::new("hostname")
        .output()
        .expect("Encountered problems while finding hostname!");

    String::from_utf8(hostname.stdout)
        .unwrap()
        .trim()
        .to_string()
}

/// Gets the current user using the command
/// `whoami` and then returns it.
fn read_username() -> String {
    let username = Command::new("whoami")
        .output()
        .expect("Encountered problems while finding username!");

    String::from_utf8(username.stdout)
        .unwrap()
        .trim()
        .to_string()
}

/// Counts the packages on the system.
fn read_package_num() -> String {
    let distro = read_distro();

    // Void linux
    if distro.contains("Void") || distro.contains("void") {
        let package_list = Command::new("xbps-query")
            .arg("-l")
            .output()
            .expect("Having problems finding package count.");

        return String::from_utf8(package_list.stdout)
            .unwrap()
            .trim()
            .lines()
            .count()
            .to_string();
    }
    // Arch and Artix linux
    else if distro.contains("Artix")
        || distro.contains("artix")
        || distro.contains("Arch")
        || distro.contains("arch")
    {
        let package_list = Command::new("pacman")
            .arg("-Q")
            .output()
            .expect("Having problems finding package count.");

        return String::from_utf8(package_list.stdout)
            .unwrap()
            .trim()
            .lines()
            .count()
            .to_string();
    }
    // Debian and Ubuntu linux
    else if distro.contains("Debian")
        || distro.contains("debian")
        || distro.contains("Ubuntu")
        || distro.contains("ubuntu")
    {
        let package_list = Command::new("dpkg")
            .arg("-l")
            .output()
            .expect("Having problems finding package count.");

        return String::from_utf8(package_list.stdout)
            .unwrap()
            .trim()
            .lines()
            .count()
            .to_string();
    }

    String::new()
}

/// Gets the banner depending on the distro.
fn read_banner() -> String {
    let distro = read_distro();
    // let distro = "gentoo";

    // Void linux
    if distro.contains("Void") || distro.contains("void") {
        return String::from(
            r"                      __     __ 
        .--.--.-----.|__|.--|  |
        |  |  |  _  ||  ||  _  |
         \___/|_____||__||_____|
                        ",
        )
        .bright_green()
        .bold()
        .to_string();
    }
    // Artix linux
    else if distro.contains("Artix") || distro.contains("artix") {
        return String::from(
            r"
                     __   __        
        .---.-.----.|  |_|__|.--.--.
        |  _  |   _||   _|  ||_   _|
        |___._|__|  |____|__||__.__|
            ",
        )
        .bright_blue()
        .bold()
        .to_string();
    }
    // Arch linux
    else if distro.contains("Arch") || distro.contains("arch") {
        return String::from(
            r"
                          __    
        .---.-.----.----.|  |--.
        |  _  |   _|  __||     |
        |___._|__| |____||__|__|
            ",
        )
        .bright_blue()
        .bold()
        .to_string();
    }
    // Debian linux
    else if distro.contains("Debian") || distro.contains("debian") {
        return String::from(
            r"
        __         __     __              
    .--|  |.-----.|  |--.|__|.---.-.-----.
    |  _  ||  -__||  _  ||  ||  _  |     |
    |_____||_____||_____||__||___._|__|__|
            ",
        )
        .bright_red()
        .bold()
        .to_string();
    }
    // Ubuntu linux
    else if distro.contains("Ubuntu") || distro.contains("ubuntu") {
        return String::from(
            r"
            __                 __         
    .--.--.|  |--.--.--.-----.|  |_.--.--.
    |  |  ||  _  |  |  |     ||   _|  |  |
    |_____||_____|_____|__|__||____|_____|
            ",
        )
        .bright_red()
        .bold()
        .to_string();
    }
    // Gentoo
    else if distro.contains("Gentoo") || distro.contains("gentoo") {
        return String::from(
            r"
                        __               
    .-----.-----.-----.|  |_.-----.-----.
    |  _  |  -__|     ||   _|  _  |  _  |
    |___  |_____|__|__||____|_____|_____|
    |_____|                                          ",
        )
        .magenta()
        .bold()
        .to_string();
    }

    String::from(
        r"
         __ __                    
        |  |__|.-----.--.--.--.--.
        |  |  ||     |  |  |_   _|
        |__|__||__|__|_____|__.__|
        ",
    )
    .bright_yellow()
    .bold()
    .to_string()
}
