use std::fs;
use std::env;

fn main() {
    if env::args().len() <= 2 {
        println!("Program requires as least 2 arguments.");
        return;
    }

    for (index, argument) in env::args().enumerate() {
        println!("argument {} is {}", index, argument);
    }

    let file_name = env::args().nth(1).unwrap();
    let person = env::args().nth(2).unwrap();
    println!("file_name is {}", file_name);
    println!("person is {}", person);
    // We need to clone file_name here so that we can use file_name later on in debugging
    // statements. The clone is necessary because otherwise file_name is moved into file_contents
    // because of some weird type issue. Feel free to removed the clone() function call and see the
    // compiler error if you want to understand it better.
    let file_contents = fs::read_to_string(file_name.clone()).unwrap();

    for line in file_contents.lines() {
        println!("line is {}", line);
        if person == line {
            println!("Found name {} in file {}", person, file_name);
            return;
        }
    }

    println!("Could not find name {} in file {}", person, file_name);
    return;
}
