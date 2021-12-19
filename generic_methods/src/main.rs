#[derive(Debug)]
struct Rectangle<T, U> {
    width: T,
    height: U
}

// adding the generics tag after the impl keyword tells the compiler that methods defined in this
// impl block can be used for Rectangle objects with any kind of data types given to its members during
// instantiation
impl<T, U> Rectangle<T, U> {
    fn get_width(&self) -> &T {
        &self.width
    }
}

// Ommitting the generics tag after the impl keyword tells the compiler that methods defined in
// this impl block can be used only by Rectangle instantiations with the specific data types
// mentioned in the generics tag after the struct name, in this case u8 and u8
impl Rectangle<u8, u8> {
    fn get_perimeter(&self) -> u8 {
        2 * self.width + 2 * self.height
    }
}

fn main() {
    // The following instantiation will cause an error when running the get_perimeter method,
    // because the get_perimeter method can be accessed only by a Rectangle<u8, u8> instatiation,
    // and not by a Rectangle<u8, u16> instantiation
    //let rect = Rectangle {
    //    width: 1u8,
    //    height: 3u16
    //};
    // The following instantiation will not throw an error on the get_perimeter method call because
    // the get_perimeter method can be accessed by a Rectangle<u8, u8> instatiation
    let rect = Rectangle {
        width: 1u8,
        height: 3u8
    };
    println!("rect is {:?}", rect);
    println!("width is {}", rect.get_width());
    println!("perimeter is {}", rect.get_perimeter());
    // Notice how both the get_perimeter and get_width methods worked on the second instatiation.
    // This is because the get_width method is available to all Rectangle<T, U> instantiations, and
    // the get_perimeter method is available to all Rectangle<u8, u8> instantiations, and the
    // second instantiation is a member of both of those groups
}
