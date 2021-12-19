// The following trait was derived to resolve the println!() issue discussed at the bottom of the
// file

// The default implementation of PartialEq will compare two instances of the same struct and
// determine them to be equal only if all of their fields are equal
// 
// The default implementation of PartialOrd will compare two instances of the same struct and
// cycle through their fields in the same order as in the struct declaration, and choose the
// greater one to be the first struct with a field greater than the other's. In the case of the
// Satellite struct, the PartialOrd trait would compare the name property before comparing the
// velocity property
#[derive(PartialEq, PartialOrd)] 
struct Satellite {
    name: String,
    velocity: f64
}

fn main() {
    let hubble = Satellite {
        name: String::from("Hubble Telescope"),
        velocity: 4.72
    };
    let gps = Satellite {
        name: String::from("GPS"),
        velocity: 2.42
    };
    // The following will throw a compiler error because there is no way to compare two Satellite
    // structs. The compiler recommends implementing the ParialEq trait to fix this.
    println!("hubble == gps is {}", hubble == gps);
    // The following will throw a compiler error because although we derived the PartialEq trait,
    // that trait does not allow us to determine if one struct is greater than the other
    println!("hubble > gps is {}", hubble > gps);
}
