fn main() {
    let rocket_fuel = produce_fuel();
    println!("rocket_fuel is {}", rocket_fuel);
}
/*
// This creates a dangling reference because no variable is being passed to the function, so there
// is no variable to be referenced inside the function, and any variables produced inside the
// function will be destroyed from going out of scope after the function completes, so returning a
// reference to the variable will return a reference to nothing.
fn produce_fuel() -> &String {
    let new_fuel = String::from("RP=1");
    &new_fuel
}
*/

fn produce_fuel() -> String {
    let new_fuel = String::from("RP-1");
    new_fuel
}
