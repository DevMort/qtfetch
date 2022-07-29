use crate::prefix;
use std::fs::File;
use std::io::prelude::*;
use std::process::Command;
// use ansi_term;

/// Holds the entire fetch information.
pub struct Entry {
    hostname: String,
    distro: String,
}

pub enum EntryType {
    Hostname,
    Distro,
}

impl Entry {
    pub fn new() -> Self {
        Self {
            hostname: read_hostname(),
            distro: read_distro(),
        }
    }

    pub fn get_string_hostname(&self) -> String {
        format!(
            "{} {} {}",
            prefix::get_prefix(EntryType::Hostname),
            "HOST:",
            self.hostname
        )
    }

    pub fn get_string_distro(&self) -> String {
        format!(
            "{} {} {}",
            prefix::get_prefix(EntryType::Distro),
            "DISTRO:",
            self.distro
        )
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

    String::from_utf8(hostname.stdout).unwrap()
}
