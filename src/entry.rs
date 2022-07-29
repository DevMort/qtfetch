use crate::entry_distro::read_distro;
use crate::entry_hostname::read_hostname;
use crate::prefix;
use ansi_term;

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
