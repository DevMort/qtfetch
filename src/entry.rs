use crate::prefix;
use colored::Colorize;
use std::fs;
use std::process::Command;

/// Holds the entire fetch information.
pub struct Entry {
    username: String,
    hostname: String,
    banner: String,
    distro: String,
    cpu: String,
    temperature: String,
    package_count: String,
    memory: (f32, f32), // current, total
}

pub enum EntryType {
    User, // ex. anon@pc
    CPU,
    PackageCount,
    Temperature,
    Memory,
    Distro,
}

impl Entry {
    pub fn new() -> Self {
        Self {
            package_count: read_package_num(),
            username: read_username(),
            temperature: read_temperature(),
            banner: read_banner(),
            cpu: read_cpu(),
            hostname: read_hostname(),
            distro: read_distro(),
            memory: read_memory(),
        }
    }

    pub fn get_string_user(&self) -> String {
        format!(
            "\t\t{} {}@{}",
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
            "DIST:".green().bold(),
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

    pub fn get_string_cpu(&self) -> String {
        format!(
            "\t{} {} {}",
            prefix::get_prefix(EntryType::CPU),
            "CPU :".green().bold(),
            self.cpu,
        )
    }

    pub fn get_string_temperature(&self) -> String {
        format!(
            "\t{} {} {}",
            prefix::get_prefix(EntryType::Temperature),
            "TEMP:".green().bold(),
            self.temperature,
        )
    }

    pub fn get_string_memory(&self) -> String {
        format!(
            "\t{} {} {}M / {}G",
            prefix::get_prefix(EntryType::Memory),
            "MEM :".green().bold(),
            self.memory.0.ceil().to_string(),
            self.memory.1.ceil().to_string(),
        )
    }
}

/// Finds what kind of distro the user
/// is currently using by reading the
/// file called /etc/os-release.
fn read_distro() -> String {
    let contents = match fs::read_to_string("/etc/os-release") {
        Ok(s) => s,
        Err(_) => String::from(""),
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

    line.rsplit('=') // splits into two ex: PRETTY_NAME=Void Linux
        .next()
        .unwrap()
        .to_string()
        .replace("\"", "")
}

/// Runs and gets the output of the command
/// `hostname` and then returns it.
fn read_hostname() -> String {
    match Command::new("hostname").output() {
        Ok(output) => String::from_utf8(output.stdout).unwrap().trim().to_string(),
        Err(_) => match fs::read_to_string("/etc/hostname") {
            Ok(hostname) => hostname,
            Err(e) => panic!("{e}"),
        },
    }
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

/// Find temperature in celsius.
fn read_temperature() -> String {
    let temp = match fs::read_to_string("/sys/class/thermal/thermal_zone0/temp") {
        Ok(t) => t.trim().parse::<f64>().unwrap() / 1000.0,
        Err(e) => panic!("Trouble finding temperature. {e}"),
    };

    format!("{}{}", temp, "°C")
}

/// Find CPU info.
fn read_cpu() -> String {
    let model_info = match fs::read_to_string("/proc/cpuinfo") {
        Ok(s) => s,
        Err(e) => panic!("Trouble finding CPU. {e}"),
    };

    // Variable `line` gives us something like:
    // model name    : Intel(R) Core(TM) i3-8100 CPU @ 3.60GHz
    let line = model_info
        .lines()
        .collect::<Vec<&str>>()
        .get(4)
        .unwrap()
        .to_string();
    line.rsplit(": ")
        .collect::<Vec<&str>>()
        .iter()
        .next()
        .unwrap()
        .to_string()
}

/// Reads the current and total memory.
fn read_memory() -> (f32, f32) {
    let meminfo = match fs::read_to_string("/proc/meminfo") {
        Ok(s) => s,
        Err(e) => panic!("Encountered a problem when finding memory. {e}"),
    };
    let meminfo = meminfo.lines().collect::<Vec<&str>>();

    let total = meminfo
        .iter()
        .next()
        .unwrap()
        .split_ascii_whitespace()
        .collect::<Vec<&str>>()
        .get(1)
        .unwrap()
        .parse::<f32>()
        .unwrap()
        / 1000000.0;
    let current = meminfo
        .iter()
        .next()
        .unwrap()
        .split_ascii_whitespace()
        .collect::<Vec<&str>>()
        .get(1)
        .unwrap()
        .parse::<f32>()
        .unwrap()
        / 1000.0;

    (current, total)
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

    // Should the distro not be found, just use "linux".
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
