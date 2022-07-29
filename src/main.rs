mod entry;
mod prefix;

fn main() {
    let entry = entry::Entry::new();

    println!("{}", entry.get_string_hostname());
    println!("{}", entry.get_string_distro());
}
