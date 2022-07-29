//! qtfetch is yet another system information program
//! with the goal of making it fast, configurable, and minimal.

mod entry;
mod prefix;

fn main() {
    let entry = entry::Entry::new();

    println!("");
    println!("{}", entry.get_string_user());
    println!("{}", entry.get_string_distro());
    println!("{}", entry.get_string_package_count());
}
