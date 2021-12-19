fn main() {
    let mut message = String::from("Earth");
    println!("{}", message);
    message.push_str(" is home");
    println!("{}", message);


    let outer_planet: String;
    {
        let mut inner_planet = String::from("Mercury");
        println!("inner_planet is {}", inner_planet);
        // This line moves ownership of string 'Mercury' from inner_planet to outer_planet, so if
        // we try to reference inner_planet again, we will get an error
        //outer_planet = inner_planet;
        // This line clones inner_planet value, so there is not transfer of ownership
        outer_planet = inner_planet.clone();
        println!("inner_planet is {}", inner_planet);
    }
    println!("outer_planet is {}", outer_planet);

    let int_outer_planet: i32;
    {
        let mut int_inner_planet = 1;
        println!("int_inner_planet is {}", int_inner_planet);
        // int has fixed size, so it can exist on the stack, whereas Strings are of variable size
        // and need to be stored on the heap
        // Because the int is of known size and on the stack, it is just as efficient to copy the
        // value from int_inner_planet to int_outer_planet as it is to add a reference to the
        // value.
        int_outer_planet = int_inner_planet;
        println!("int_inner_planet is {}", int_inner_planet);
    }
    println!("int_outer_planet is {}", int_outer_planet);

    let rocket_fuel = 1;
    // Since rocket_fuel is an int, lives on the stack, when passed as an argument to
    // process_fuel(), a copy of the value is passed to the function. The process_fuel() function
    // can modify its passed parameter without modifying the passed argument in the main function
    process_fuel(rocket_fuel);
    println!("rocket fuel is {}", rocket_fuel);

    // In the following case, a String is created on the heap, and assigned to rocket_fuel_str.
    // rocket_fuel_str is then passed to process_fuel_str(), which assigns the value in heap to the
    // new variable propellant. This discards the old variable rocket_fuel in the main function,
    // since the value is now owned by propellant. Once process_fuel_str() completes, the
    // propellant variable goes out of scope, and since there is no owner of the string value in
    // heap, the string value is freed from memory. Alternatively, you could pass a cloned variable
    // to the function to preserve the rocket_fuel variable in the main function
    let rocket_fuel_str = String::from("RP-1");
    process_fuel_str(rocket_fuel_str);
    // The following line won't work because rocket_fuel_str doesn't exist anymore
    //println!("rocket fuel str is {}", rocket_fuel_str);

    // This will return the value in heap to a new variable called rocket_fuel instead of
    // discarding it when propellant goes out of scope
    let rocket_fuel_str2 = String::from("RP-1");
    let rocket_fuel_str2 = process_fuel_str_with_ret(rocket_fuel_str2);

    let rocket_fuel_str3 = String::from("RP-1");
    let rocket_fuel_str3 = process_fuel_str_with_new(rocket_fuel_str3);
}

fn process_fuel(mut propellant: i32) {
    propellant += 1;
    println!("processing propellant {}...", propellant);
}

fn process_fuel_str(propellant: String) {
    println!("processing propellant {}...", propellant);
}

fn process_fuel_str_with_ret(propellant: String) -> String {
    println!("processing propellant {}...", propellant);
    propellant
}

fn process_fuel_str_with_new(propellant: String) -> String {
    println!("processing propellant {}...", propellant);
    let new_fuel = String::from("LNG");
    return new_fuel;
}
