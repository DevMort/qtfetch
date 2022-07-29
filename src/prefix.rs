use crate::entry::EntryType;

pub fn get_prefix(entry_type: EntryType) -> String {
    match entry_type {
        EntryType::Hostname => {}
        EntryType::Distro => {}
    }

    String::from("")
}
