fn main() {
    let message = String::from("Greetings from Earth!");
    println!("message is {}", message);

    let last_word_full_string = &message; // This creates a reference to the entire string
    let last_word = &message[15..15+5];
    println!("last_word is {}", last_word);

    let planets = [1,2,3,4,5,6,7,8];
    let inner_planets: &[i32] = &planets[2..];
    println!("inner_planets are {:?}", inner_planets);
    println!("first_word is {}", get_first_word(&message));
    // The following will not work because the function expects a reference to a String, rather
    // than a slice, which is a reference to a section of a String. We can fix this by having
    // get_first_word expect a splice instead of a String reference
    //println!("first_word is {}", get_first_word(&message[10..]));
    let test_str = String::from("      Trent       ");
    println!("trim_spaces is {}, tada!", trim_spaces(&test_str));
    let test_str2 = String::from("      ");
    println!("trim_spaces is {}, tada!", trim_spaces(&test_str2));
    let test_str3 = String::from("");
    println!("trim_spaces is {}, tada!", trim_spaces(&test_str3));
}

fn get_first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (index, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..index]; // found a space!
        }
    }

    &s // no spaces found; input is a single word
}

/* CHALLENGE */
fn trim_spaces(s: &String) -> &str {
    if s == "" {
        return "";
    }
    let bytes = s.as_bytes();

    let mut first_index = 0;
    let mut last_index = s.len() - 1;

    for (index, &item) in bytes.iter().enumerate() {
        if item != b' ' {
            first_index = index;
            break;
        }
    }

    if first_index == 0 {
        // All spaces
        return "";
    }

    for (index, &item) in bytes.iter().enumerate().rev() {
        if item != b' ' {
            last_index = index + 1;
            break;
        }
    }
    
    let slice = &s[first_index..last_index];
    
    slice
}
