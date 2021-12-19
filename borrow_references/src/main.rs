fn main() {
    let rocket_fuel = String::from("RP-1");
    /*
    let (rocket_fuel, length) = process_fuel_using_value(rocket_fuel);
    println!("rocket_fuel is {} and length is {}", rocket_fuel, length);
    */
    let length = process_fuel_using_borrow(&rocket_fuel);
    println!("rocket_fuel is {} and length is {}", rocket_fuel, length);
    let mut rocket_fuel_mut = String::from("RP-1");
    let mut_length = process_fuel_using_borrow_with_mut(&mut rocket_fuel_mut);
    println!("rocket fuel is {} and length is {}", rocket_fuel, length);
}

fn process_fuel_using_value(propellant: String) -> (String, usize) {
    println!("processing propellant {}...", propellant);
    let length = propellant.len();
    (propellant, length)
}

fn process_fuel_using_borrow(propellant: &String) -> usize {
    println!("processing propellant {}...", propellant);
    let length = propellant.len();
    length
}

fn process_fuel_using_borrow_with_mut(propellant: &mut String) -> usize {
    println!("processing propellant {}...", propellant);
    propellant.push_str(" is highly flammable!");
    let length = propellant.len();
    length
}
