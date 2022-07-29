use crate::entry::EntryType;

pub fn get_prefix(entry_type: EntryType) -> String {
    match entry_type {
        EntryType::Hostname => "👤".to_string(),
        EntryType::Distro => "🖥️ ".to_string(),
    }
}
