//fn get_biggest<T>(a: T, b: T) -> T {

// After running this file with the above signature, the compiler throws an error, as discussed
// below, and recommends that we restrict the possible types that T can be through the use of the
// PartialOrd trait. Traits are discussed later on in this tutorial, but we will use it here simply
// to show how to fix the current problem
fn get_biggest<T: PartialOrd>(a: T, b: T) -> T {
    if a > b {
        a
    } else {
        b
    }
}

fn main() {
    // This will throw an error, because the data type could be anything, int, float, or custom.
    // The get_biggest function is not capable of comparing necessarily every data type through the
    // basic > operator, so if a Strings were passed instead of integers, at runtime this would
    // cause an error. The compiler throws an error warning us of this
    println!("biggest is {}", get_biggest(1,2));
}
