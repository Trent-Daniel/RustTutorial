struct Satellite {
    name: String,
    velocity: f64 // miles per second
}

struct SpaceStation {
    name: String,
    crew_size: u8,
    altitude: u32 // miles
}

trait Description {
    // The following is the declaration of the describe method, which would leave the Description
    // trait without a default implementation of the describe method, requiring that it be
    // implemented by every struct which uses the Description trait
    //fn describe(&self) -> String;
    //
    // The following is a default implementation of the describe method, so structs using the
    // Description trait can optionally implement their own describe function, or they can use this
    // default implementation instead
    fn describe(&self) -> String {
        String::from("an object flying through space!")
    }
}

// This will use the default implementation of the describe method
impl Description for Satellite {}

// This has a custom implementation of describe for the SpaceStation struct
impl Description for SpaceStation {
    fn describe(&self) -> String {
        format!("the {} flyting {} miles high with {} crew members aboard!", self.name, self.altitude, self.crew_size)
    }
}

fn main() {
    let hubble = Satellite {
        name: String::from("Hubble Telescope"),
        velocity: 4.72
    };
    let iss = SpaceStation {
        name: String::from("International Space Station"),
        crew_size: 6,
        altitude: 254
    };
    // The following will throw an error because the default formatter can't print a custom type.
    //println!("hubble is {}", hubble);
    //println!("iss is {}", iss);
    // Instead of adding the Debug macro to our structs, we added a description trait which allows
    // us to call a custom formatter
    println!("hubble is {}", hubble.describe());
    println!("iss is {}", iss.describe());
}
