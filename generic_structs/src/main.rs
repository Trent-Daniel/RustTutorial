// The following struct only takes f64 values, so we can't pass it integers
//#[derive(Debug)]
//struct Rectangle {
//    width: f64,
//    height: f64
//}
//
// Instead, we will use a Generic Struct so that we can instantiate different structs with
// different typed parameters

#[derive(Debug)]
struct Rectangle<T, U> {
    width: T,
    height: U
}

fn main() {
    let rect = Rectangle {
        width: 1u8,
        height: 3u16
    };
    println!("rect is {:?}", rect);
}
