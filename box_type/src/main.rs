use std::mem;

struct Shuttle {
    name: String,
    crew_size: u8,
    propellant: f64
}

fn main() {
    let vehicle = Shuttle {
        name: String::from("Atlantis"),
        crew_size: 7,
        propellant: 835958.0
    };
    println!("vehicle size on stack: {} bytes", mem::size_of_val(&vehicle)); // Should be 40 bytes

    // At this point, the name property of vehicle is a String, so it is on the heap, but the
    // crew_size and propellant properties are on the stack with the variable's refernce to the
    // name property's location in the heap.
    let boxed_vehicle: Box<Shuttle> = Box::new(vehicle);
    // The above creation of the Box variable allocates enough space on the heap to store the
    // Shuttle instance. Remember, the String property 'name' was already on the heap and has not
    // moved. The Shuttle instance's refrence to that String is still the same. However, instead of
    // the String reference, crew_size, and propellant properties being stored on the stack, that
    // is now all stored on the heap. Because the memory is being transferred from the stack to the
    // heap, this is not a copy or clone operation, but a move operation. So, the vehicle object no
    // longer has ownership of the struct and will no longer be valid
    //
    // The following would throw an error since the vehicle object is no longer valid
    //println!("vehicle size on stack: {} bytes", mem::size_of_val(&vehicle));
    //
    // This will give us the size of the box pointer on the stack
    println!("boxed_vehicle size on stack: {} bytes", mem::size_of_val(&boxed_vehicle));
    // Passing the address of the dereferenced pointer will give us the size of the Shuttle object
    // on the heap
    println!("vehicle size on heap: {} bytes", mem::size_of_val(&*boxed_vehicle));
    // This reallocates everything within the struct that the box is pointing to, back onto the
    // stack, effectively undoing the unboxing. The String type name property, however, remains on
    // the heap, and the Shuttle struct's reference to that String stays the same
    let unboxed_vehicle: Shuttle = *boxed_vehicle;
    println!("unboxed_vehicle size on stack: {} bytes", mem::size_of_val(&unboxed_vehicle)); // Should be 40 bytes
}
