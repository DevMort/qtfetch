use std::fs::File;
use std::io::prelude::*;

pub fn read_distro() -> String {
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
