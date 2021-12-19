// We are returning a borrowed reference, but it is impossible to know at compile time what that
// value will be, x or y. Because of this, the compiler throws an error, asking for an 'expected
// named lifetime parameter'
//fn best_fuel(x: &str, y: &str) -> &str {
// The following syntax includes a lifetime parameter.
// It is similar to the syntax of generics, but instead of defining a generic data type, it defines
// a generic lifetime for the input and output parameters
// This syntax tells the compiler that it can expect the lifetime of two inputs and the output to
// be the same. If these lifetimes are in fact different, the compiler will presume the most
// restrictive lifetime of the inputs to be the templated lifetime
fn best_fuel<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

//fn best_fuel<'a>(x: &'a str, y: &'a str) -> &'a str {
// Using the nested block in main, the above function signature still results in the smaller
// and insufficient lifetime of propellant2 being used for this function. Using the following
// signature results in y being treated as having a different lifetime than x or the returned item.
// Since we only return x, and x/propellant1's lifetime persists to the println at the bottom of
// main, the following function compiles. We could have omitted the lifetime of y altogether, but
// that is ambiguous. Including it is more clear to read.
fn best_fuel2<'a, 'b>(x: &'a str, y: &'b str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        x
    }
}

fn main() {
    let result;
    let propellant1 = String::from("RP-1");
    let propellant2 = String::from("LNG");
    result = best_fuel(&propellant1, &propellant2);
    // With the following code block instead of the two lines above, the compiler will consider the
    // templated lifetime of best_fuel to be the lifetime of propellant2, since it is the most
    // restrictive lifetime. Because that lifetime ends before 'result' is printed, the compiler
    // recognizes that there could be a dangling reference, and fails to compile. Note that
    // best_fuel does not return a distinct String, but rather a reference to one of the Strings
    // passed to it as arguments, which is why the compiler pays attention to the reference
    // lifetimes.
    //{
    //    let propellant2 = String::from("LNG");
    //    result = best_fuel(&propellant1, &propellant2);
    //}
    println!("result is {}", result);
}
