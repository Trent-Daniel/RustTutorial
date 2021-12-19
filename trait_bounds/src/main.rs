use std::any;
use std::fmt;

// Here we are using the Display trait as a boundary to this function's behaviour. In order to
// print something, it needs to be printable, and not all data types are. If the compiler were not
// strict, we could omit the Display trait bound, and pass in a non-printable data type, like an
// array, which would cause a runtime error. Since the compiler IS strict, it fails because it can
// forsee this possible runtime error. By adding the Display trait boundary, we require that any
// argument passed to print_type implement the Display trait, guaranteeing that it can be
// displayed.
fn print_type<T: fmt::Display>(item: T) {
    println!("{} is {}", item, any::type_name::<T>());
}

// Although arrays can't be printed using the default formatter and don't implement the
// Display trait, they do implement the Debug trait and can be printed using the debug formatter
fn print_debug<T: fmt::Debug>(item: T) {
    println!("{:?} is {}", item, any::type_name::<T>());
}

// Multiple trait bounds
// Both 'a' and 'b' need Display so that they can be printed
// Both 'a' and 'b' need PartialEq so that they can be compared for equality
// 'a' needs the From<U> to make sure that 'b' can be cast from its own type to the same type as 'a'
// 'b' needs Copy to make sure that when 'b' is passed to the from function, that 'b' is copied
// instead of being moved.
// That's a lot of traits and is difficult to both read and write, so we'll use an alternative
// means of writing out the multiple trait bounds, using the 'where' keyword
//fn compare_and_print<T: fmt::Display + PartialEq + From<U>, U: fmt::Display + PartialEq + Copy>(a: T, b: U) {
fn compare_and_print<T, U>(a: T, b: U)
    where T: fmt::Display + PartialEq + From<U>,
          U: fmt::Display + PartialEq + Copy
{
    if a == T::from(b) {
        println!("{} is equal to {}", a, b);
    } else {
        println!("{} is NOT equal to {}", a, b);
    }
}

fn main() {
    print_type(13);
    print_type(13.0);
    print_type("thirteen");
    //print_type([13]);
    print_debug([13]);

    compare_and_print(1.0, 1);
    compare_and_print(1.1, 1);
    // The following will throw a compiler error because the slice 'one' cannot be converted into a
    // floating point number, and so the From trait bound is not met in this case
    //compare_and_print(1.1, "one");
}
