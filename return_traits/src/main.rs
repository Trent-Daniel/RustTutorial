use std::fmt;


// The return type indicator states that whatever type is returned will be a type that implements
// the Display trait. The compiler can infer from the code what the actual returned type is because
// only one type is returned.
fn get_displayable() -> impl fmt::Display {
    // This works
    //13
    
    // This works
    // "thirteen"

    // This does not work
    [13]
}

// This does not work because the compiler needs to know what the return type will be. Above, there
// was only ever one possible return type, so the compiler could infer from the code. Below,
// however, the compiler can not infer, because different types could be returned.
// If you really can not know what type the function will return, that gets into the topic of
// dynamic dispatch which goes beyond the scope of this course
fn get_displayable_upgraded(choice: bool) -> impl fmt::Display {
    if choice == true {
        13
    } else {
        "thirteen"
    }
}

fn main() {
    println!("output is {}", get_displayable());
}
