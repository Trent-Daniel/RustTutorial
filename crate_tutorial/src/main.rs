// The following imports the entire prelude of the rand crate
use rand::prelude::*;

fn main() {
    let number = random::<f64>();
    println!("number is {}", number);

    let number = thread_rng().gen_range(1..11);
    println!("number is {}", number);
}


//use rand::random;
//
//fn main() {
//    let number = random::<f64>();
//    println!("number is {}", number);
//}

// The above is equivalent to the following

//use rand;
//
//fn main() {
//    let number = rand::random::<f64>();
//    println!("number is {}", number);
//}

// The following will cause a compiler error if
// use rand::random is imported, because the names are confusing to the compiler
//fn random() -> f64 {
//    1.0
//}
