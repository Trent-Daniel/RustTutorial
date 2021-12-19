use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Error;

struct Satellite {
    name: String,
    velocity: f64 // miles per second
}

impl Display for Satellite {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "({}, {})", self.name, self.velocity)
    }
}

fn main() {
    let hubble = Satellite {
        name: String::from("Hubble Telescope"),
        velocity: 4.72
    };
    println!("hubble is {}", hubble);
}
