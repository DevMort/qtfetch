use std::process::Command;

pub fn read_hostname() -> String {
    let hostname = Command::new("hostname")
        .output()
        .expect("Encountered problems while finding hostname!");

    String::from_utf8(hostname.stdout).unwrap()
}
