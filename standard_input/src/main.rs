use std::io;

fn main() {
    let mut buffer = String::new();
    println!("Enter a message:");
    io::stdin().read_line(&mut buffer);
    println!("buffer is {}", buffer);

    let number = buffer.trim().parse::<i32>().unwrap();
    // The following is equivalent to the above call
    //let number: i32 = buffer.trim().parse().unwrap(); 
    // The above function calls will both actually return a Result Enum instead of i32. The Result Enum will contain either a number, or an error if one was encountered during parsing
    println!("number + 1 is {}", number + 1);
}
