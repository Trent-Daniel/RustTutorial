use std::fs;
use std::io;

fn main() {
    // the_ultimate_question.txt does not exist, so there will be an error while trying to open
    // this file.
    //let contents = fs::read_to_string("the_ultimate_question.txt");
    //println!("contents is: {:?}", contents);
    //
    // The following will work only if there is no error on file opening. The unwrap() method will
    // only extract the Ok variance from the Result enum, and will panic if it finds the Err
    // variance.
    //let contents = fs::read_to_string("the_ultimate_question.txt").unwrap();
    //
    // The following will panic with a custom message
    //let contents = fs::read_to_string("the_ultimate_question.txt").expect("Nobody knows the ultimate question!");
    //println!("contents is: {:?}", contents);
    let result = fs::read_to_string("the_ultimate_question.txt");

    // The following will allow us to handle the error properly, but only in a general sense
    //let contents = match result {
    //    Ok(message) => message,
    //    Err(error) => String::from("Nobody knows the ultimate question!")
    //};

    // The following will allow us to handle the error properly, and in a more specific way
    let contents = match result {
        Ok(message) => message,
        Err(error) => match error.kind() {
            io::ErrorKind::NotFound => String::from("File not found."),
            io::ErrorKind::PermissionDenied => String::from("Permission denied."),
            _ => panic!("Another type of erro: {:?}", error)
        }
    };

    println!("contents is: {:?}", contents);
}
