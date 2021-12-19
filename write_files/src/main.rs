use std::fs;
use std::io::prelude::*; // import Write and others

fn main() {
    let mut speech = String::new();
    speech.push_str("we choose to go to the Moon in this decade\n");
    speech.push_str("and do the other things,\n");
    speech.push_str("not because they are easy,\n");
    speech.push_str("but because they are hard.");

    fs::write("speech.txt", speech);

    let mut file = fs::OpenOptions::new().append(true).open("speech.txt").unwrap();
    file.write(b"\nPluto");
}
