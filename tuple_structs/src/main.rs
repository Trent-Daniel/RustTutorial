// Struct for colour
struct Colour(u8, u8, u8); // RGB
struct Point(u8, u8, u8); // XYZ

fn get_y(p: Point) -> u8 {
    p.1
}

fn main() {
    let red = Colour(255, 0, 0);
    println!("First value is {}", red.0);

    // This will fail because the function is expecting a Pont type object, but receives a Colour
    // type object.
    //let y = get_y(red);
    let coord = Point(4,5,6);
    // The following will succeed because the function is being passed the correct data type
    let y = get_y(coord);
    println!("y is {}", y);


    /* CHALLENGE */
    test_routine();
}


/* CHALLENGE */
struct Rectangle {
    width: f64,
    height: f64
}

impl Rectangle {
    fn get_area(&self) -> f64 {
        return self.width * self.height
    }

    fn scale(&mut self, scale: f64) {
        self.width *= scale;
        self.height *= scale;
    }

    fn new(passed_width: f64, passed_height: f64) -> Rectangle {
        return Rectangle {
            width: passed_width,
            height: passed_height
        };
    }
}

fn test_routine() {
    let mut rect = Rectangle::new(1.2, 3.4);
    assert_eq!(rect.get_area(), 4.08);
    rect.scale(0.5);
    assert_eq!(rect.get_area(), 1.02);
    println!("Tests passed!");
}






