fn main() {
    let planet = "Earth";
    if true {
        //let planet = "Earth";
        println!("planet is {}", planet);
    }

    println!("planet is {}", planet);

    // Shadowing variables
    // Declare a new variable with the same name as an existing variable
    // New variable "shadows" the previous variable
    let planet = 4;
    println!("planet is {}", planet);

    let planet = "Mars";
    {
        println!("planet is {}", planet);
        // This is where shadowing can actually be useful
        let mut planet = 4;
        println!("planet is {}", planet);
    }
    println!("planet is {}", planet);
}
