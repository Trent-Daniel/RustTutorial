enum Location {
    Unknown,
    Anonymous,
    Known(f64, f64)
}

impl Location {
    fn display(&self) {
        match *self {
            Location::Unknown => println!("Unknown"),
            Location::Anonymous => println!("Anonymous"),
            Location::Known(lat, lon) => println!("lat is {} and lon is {}", lat, lon)
        }
    }
}

fn main() {
    let address = Location::Unknown;
    address.display();
    let address = Location::Anonymous;
    address.display();
    let address = Location::Known(28.608295, -80.604177);
    address.display();
}
