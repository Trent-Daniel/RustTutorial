// The Debug macro is added to the struct so that we can print it using the debug formatter
// The Clone macro is added to the struct so that we can clone instances of the struct. This
// works around the issue of trying to copy a Struct with a String property. For more info on that
// problem, search for vehicle3 in this file.
#[derive(Debug)]
#[derive(Clone)]
struct Shuttle {
    name: String,
    crew_size: u8,
    propellant: f64
}

impl Shuttle {
    // This is a method since it gets passed the &self parameter
    fn get_name(&self) -> &str {
        &self.name
    }

    // This is a method since it gets passed the &self parameter
    fn add_fuel(&mut self, gallons: f64) {
        self.propellant += gallons;
    }

    // This is a function since it is not passed the &self parameter. This function is used as a
    // constructor for the class
    fn new(name: &str) -> Shuttle {
        Shuttle {
            name: String::from(name),
            crew_size: 7,
            propellant: 0.0
        }
    }
}

fn main() {
    let mut vehicle = Shuttle {
        name: String::from("Endeavour"),
        crew_size: 7,
        propellant: 835958.0
    };

    let vehicle2 = Shuttle {
        name: String::from("Discovery"),
        // This will copy the rest of the field values from the named instance
        ..vehicle
    };

    // Since we instantiate vehicle with property name of type String, a String is allocated on the
    // heap, and vehicle owns that String. A String can't have more than one owner, so we can't
    // copy it into this third struct. This error does not occur with the other properties, because
    // u8 and f64 are both types with a fixed size, so they can be allocated on the stack and can
    // be copied to new instances on the stack. However, Strings cannot simply be copied, they must
    // be explicitly cloned, hence the borrow during partial move error
    //let vehicle3 = Shuttle {
    //    ..vehicle
    //};

    // Since we added the Clone macro to the struct definition, we can instatiate this struct as a
    // clone of a previous struct
    let vehicle4 = Shuttle {
        ..vehicle.clone() 
    };

    println!("vehicle.name is {}", vehicle.name);
    vehicle.name = String::from("Atlantis");
    //notice that after instantiation the structs, a change to the first struct does not affect the
    //second one, even if the second stuct was instantiated as a partial copy of the first struct
    vehicle.crew_size = 6;
    println!("vehicle is {:?}", vehicle);
    println!("vehicle2 is {:?}", vehicle2);
    //println!("vehicle3 is {:?}", vehicle3);
    println!("vehicle4 is {:?}", vehicle4);

    let vehicle_name = vehicle.get_name();
    println!("vehicle_name is {}", vehicle_name);
    println!("starting propellant of vehicle is {}", vehicle.propellant);
    vehicle.add_fuel(1000.0);
    println!("ending propellant of vehicle is {}", vehicle.propellant);

    let vehicle_from_constructor = Shuttle::new("Endeavour");
    println!("vehicle_from_constructor is {:?}", vehicle_from_constructor);
}
