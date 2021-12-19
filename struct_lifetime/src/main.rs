// Changing the name property from String to a slice, the Shuttle instance no longer owns the
// name propertyi because the property is now a reference. Therefore it is no longer clear how the lifetime of the string value in the heap
// relates to the lifetime of the struct. If the string is dropped while the struct is still in
// scope, and the struct tries to reference the string, it will be using an invalid reference to
// nothing. Therefore, we need to add explicit lifetime annotations to describe the lifetimes.

//struct Shuttle {
//    // This compiles
//    //name: String
//    // This causes a lifetime expected error
//    name: &str
//}
//
//impl Shuttle {
//    fn send_transmission(&self, msg: &str) -> &str {
//        println!("Transmitting message: {}", msg);
//        &self.name
//    }
//}

// This struct has a generic lifetime for property 'name'. Doing this modifies the data type, so
// the implementation needs to be modified as well
struct Shuttle<'a> {
    name: &'a str
}

// The lifetime tag after impl introduces the tag to the entire code block so that we can use it
// The lifetime tag after the struct name is present so that it matches the struct declaration
//impl<'a> Shuttle<'a> {
//    fn send_transmission(&self, msg: &str) -> &str {
//
// Notice how below we have added a second lifetime to the tag following impl, so it may be used in
// this block, but we have not added a second lifetime to the tag following Shuttle. That is
// because the tag following shuttle is there only to match the Shuttle struct declaration as found
// earlier in this file.
impl<'a, 'b> Shuttle<'a> {
    // Notice that we now have to explicitly assign a lifetime to self, msg, and the output, but
    // the output and msg lifetimes are the same
    fn send_transmission(&'a self, msg: &'b str) -> &'b str {
        println!("Transmitting message: {}", msg);
        // This compiles
        //&self.name
        // The reason the following line throws a compiler error, is because Elision Rul #3 assigns
        // the outputs the same lifetime as the self reference. This worked fine before when we
        // were returning a property of self, but because we are now returning the input msg, whose
        // lifetime is unknown, the compiler can't tell what the lifetime of the output is. Note:
        // The compiler infers the lifetime of the self parameter, because it is referring to the
        // Shuttle instance, and the Shuttle instance has a lifetime provided through the impl
        // definition.
        msg
    }
}

fn main() {
    let vehicle = Shuttle {
        // This compiles
        // name: String::from("Endeavour")
        name: "Endeavour"
    };

    let sender = vehicle.send_transmission("Greetings from orbit!");
    println!("sender is {}", sender);
}
