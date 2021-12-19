# General Conventions #
In rust, file names cannot include spaces
rust is freeform, like C. However, it is convention in rust to have indentation of 4 spaces.

# Variables #
variables are immutable by default
variables can contain letters, numeric digits, and underscores
variables must begin with a letter or an underscore
variables are case sensitive
variables cannot be keywords
refer to RFC 430 for more info on variable naming schemes, like camel vs snake

# Data Types #
Defines what the programmer intends data to mean
Defines how data should be stored and interpreted
Defines which operatins can be used to manipulate data
Rust is statically typed: All variable data types must be known at compile time.
Four basic scalar data types in Rust:
    - integers
        - integers are stored as signed integers by default
    - floating points
    - booleans
    - characters
        - all characters are 4 bytes

Arrays:
    - 2D arrays must be square

Tuples:
    - Tuples are like arrays, but can be multiple items of
mixed data types
    - Elements are ordered, although the order is not usual
ly the focus
    - Sored in a fixed-length, contiguous section of memory
    - Data types of items must be known at compile time

Statement:
    - Performs an action without returning a value
    - Ends with a semicolon
    - e.g.: x = 1, works; y = x = 1, will not work
Expression:
    - Evaluates to a resulting value
    - Does not end with a semicolon
    - e.g.: 1 + 2, is an expression
    - e.g.: 1 + 2; is a statement, not an expression
If you don't return a value from a function, Rust will implicitly return the Unit Data Type

Unit Data Type:
    - Used when there is no other meaningful values that could be returned
    - Represented with ()
    - e.g.: fn do_something(input: i32) -> () {}

loops:
    - loop:
        - Repeat a block of code forever
	- Need the loop to return a value
    - while:
        - Continue repeating a block of code as long as a condition is true
    - for:
        - iterate over each item in a collection
	- repeat a block of code N times -> iterate over range 0..N

Memory:
    - Stack:
        - LIFO
	- Push and pop data very quickly
	- Access data very quickly
	- Small size
	- All data must have a know, fixed size
    - HEAP:
        - ROM
	- Pointers
	    - Data type that stores a memory address
	- Adding and accessing data is slower than the stack
	- Dynamically add and access data
	- A lot more space than the stack

String Literal:
    - Hard-coded into the executable
    - Immutable
    - Must be known before compilation

String Type:
    - Allocated memory on the heap
    - Mutable
    - Dynamically generated at runtime

Memory Management:

Explicit Allocation and Deallocation:
    - Programmer is responsible for memory management
        - Ex: C/C++ malloc() and free() function
    - Advantage:
        - Programmer has lots of control and can write very efficient code
    - Disadvantage:
        - Memory leaks
        - Invalid memory access

Garbage Collection:
    - Garbage collector automatically cleans up memory
        - Ex: Java, Python, C#, Ruby, Go
    - Advantage:
        - It's easy
    - Disadvantage:
        - Wasteful of memory
	- Can run at inconvenient times

Ownership:
    - Variables are responsible for freeing their own resources
    - Rules:
        - Every value is 'owned' by one, and only one, variable at a time
	- When the ownging variable goes out of scope the value is dropped
    - Advantage:
        - Safe
	- Efficient
    - Disadvantage:
        - Requires understanding of ownership
    - Rust uses Ownership
    - Copy:
        - Done for stack-only data types, such as integer and floating point
	- Copying occurs implicitly; cloning must be done explicitly
    - Borrow:
        - Access data without taking ownership of it
	- Create references using the borrow operator (&)
Restriciton to borrow reference:
    - Once you create a mutable reference to a variable, you cannot create other references to that variable
    - Prevents data races
Slice:
    - Reference to a contiguous section of a collection
    - Commonly encountered as the string slice data type: &str
    - String literals are slices
    - Length is number of bytes, not characters
    - Range indices must occur at valid UTF-8 boundaries

Rust Standard Library:
    - Core data types
    - Functions
    - Macros
    - Many other things
    - Available to all Rust programs by default
use Statement:
    - Bring a module path into scope
    - Usually located at the top
The Prelude:
    - List of things automatically imported into every Rust program
    - Does not include the entire Rust std library
Standard Input:
    - Read command-line inputs from the user
    - Part of the std::io module
Crates:
    - A collection of Rust source code files
    - Binary crates compile to produce ana executable program
    - Library crates contain code for other programs to use
    - crates.io is the crates index for Rust
std::env::args:
    - Returns an iterator over arguments passed to the program
    - First argument is traditionally the executable path, but that is not guaranteed
    - is not included in Rust prelude
std::fs::write:
    - Simple to use
    - Will replace contents of existing files
    - Writes entire contents of the file
Structs:
    - Similar to tuple in that is a collection of differently typed members
    - Difference: Tuples have ordered, unnamed members; whereas, structs have unordered, named members
    - Unless explicitly placed on the heap, struct data is put on the stack
    - If struct contains heap-only data, like a string of non-fixed length, the variable is kept on the stack and the value of the variable is kept on the heap. The struct owns the String, which owns the heap memory
    - This gets tricky when you want to store a reference data type within a struct, like a slice. Rust requires that you annotate the lifetime of the struct to ensure that the data it references with be valid as long as the struct is alive
    - Methods:
        - Subroutines which are associated with a struct
        - Can have input parameters and a return value
        - Declared using fn keyword
	- First parameter is a reference to the struct instance, which allows the method to alter the data of the struct instance
    - Functions:
        - Function associated with a struct data type
	- Does not have a &self parameter
	- Cannot use a Function to alter data of a struct instance
	- Provides subroutines related to the Struct, but not necessarily related to an instantiation of the struct
	- Functions can be used like constructors
Tuple Structs:
    - Store a collection of mixed data without named fields
    - Distinguishable as a unique data type
Generic Structs:
    - Abstract stand-ins for concrete data types or other propertes
    - Can be used with structs, functions, methods, and more
    - Defined with <T>
    - Generics in Rust are 0-cost, and are compiled inline using monomorphization
Box<T> Data Type:
    - Store data of generic type T on the heap, even if T is a data type that would normally be stored on the stack
    - The box consists of a pointer on the stack that points to a portion of allocated memory large enough to accomodate whatever data has been stored
    - Considered a smart pointer because:
        - Provide additional functionality beyond references:
	    - Box<T> has ownership of the data it points to
	    - When Box<T> goes out of scope it deallocates the memory it was pointing to
    - Reasons to use a Box<T>:
        - Store a type whose size cannot be known at compile time
	    - e.g. recursive types
	- Transfer of ownership of data rather than copy it on the stack
	    - Avoids copying large amounts of data from the stack, which could improve performance
Traits:
    - A collection of methods
    - Data types can implement a trait
    - Generics use traits to specify the capabilities of unknown data types
    - Similar to interfaces in other programming languages
    - In certain situations it may be useful to have a default implementation for one or more methods in a trait. Useful for traits with many methods and you don't want to have to implement every single one of them for every single data type with that trait
Derive Traits:
    - Can be an easier way to use a trait than writing a custom implementation
    - The Rust compiler provides default implementations for serveral common traits, like Eq, PartialEq, Ord, PartialOrd, Clone, Copy, Hash, Default, and Debug
Trait bounds:
    -  Require a generic type to implement specific traits
    - Guarantees the generic type will have necessary behaviours
Borrow Checker:
    - Compares scopes to determine whether all borrows are valid
Lifetime Annotation:
    - Explicitly defines a generic lifetime for parameters
    - Must begin with an apostrophe (') symbol
    - Names are conventionally single lowercase letters
Lifetime Elision Rules:
    - Set of rules for the compiler to analyze reference lifetimes
    - Describes situations that do not require explicit lifetime annotations
    - If any ambiguity remains, explicit annotation will be required
    - There are currently three rules:
        - Each input parameter that is references is assigned its own lifetime
	- If there is exactly one input lifetime, assign it to all output lifetimes
	- If there is a &self or &mut self input parameter, its lifetime will be assigned to all output lifetimes
'static Lifetime:
    - Indicates references available for entire duration of a program. This data never gets dropped, so you can reference it safely at any time.
    - String literals never get dropped because they are stored in the program's binary.
    - Example: string literals
        let s: &'static str = "Greetings from Neptune!";
    - Can be coerced to more restrictive lifetime if the reference is passed as the output parameter from a function with another lifetime annotation
    - Can be used as a trait bound
        - Ensures the data type will only contain 'static elements
	- E.g. T: Display + 'static
Enum:
    - Defines a data type with multiple possible variants
Match Operator:
    - Compares a value to a series of patterns to determine which code to execute
    - Analogy is a coin sorting machine which sorts coins based on their size
How do you represent Nothing?:
    - Many languages use a null value to indicate "no value"
    - Errors often occur when using a null value in a not-null context
    - Rust does not have a conventional null value
    - Instead, Rust has the Option enum:
        - enum Option<T> {Some(T), None}
	- Is included in the std prelude, so doesn't need to be imported
	- E.g.:
	    - let something = Some(13);
	    - let something = Some(13.0);
	    - let something = Some("thirteen);
	    - let something = None;
Errors:
    - Unrecoverable:
        - index beyond array bounds
	- handle with panic! macro
	    - Immediately terminate the program and provide feedback
	    - Gives exit code 101
	    - running cargo or rust with the environment variable RUST_BACKTRACE=1 set enables stack tracing for the program
    - Recoverable:
	- Errors that do not cause the program to fail and can be corrected
        - E.g. file not found error
	    - Prompt to user to select a different file
	    - Create missing file
	- handle with Resutl<T, E>:
	    - enum Result<T, E> {
	        Ok(T),
		Err(E)
	      }
Vectors:
    - Collection of elements with the same data type
    - Elements are stored in order
    - Different from array, because array is stored on stack so their size must be known at compile time. In contrast, vectors can change in size dynamically at runtime, and exist on the heap, like Strings.
Hash Map:
    - Stores data in key -> value pairs
    - Use keys to lookup corresponding values
    - Key -> value mapping is one way
    - Use a hash function to determine how to store data, so they can be quickly located
    - Keys and values can be different data types
    - All keys must be the same type
    - All values must be the same type
    - Each key can have only one value associated with it at one time
    - There can be multiple instances of the same value across multiple keys
    - There can not be multiple instances of the same key
    - Ways to update hash map entries:
        - Overwrite an existing key-value pair
	- Insert a new entry if a key does not exist
	- Modify a value based on its existing value
What to learn next?:
    - Managing larger projects using cargo, i.e. making crates
    - Programming for concurrency
    - Writing unsafe code
