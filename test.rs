
use std::io;
fn main() {
    // Statements here are executed when the compiled binary is called

    // Print text to the console
    println!("Hello World!");

    // do not need claim the type
    let test_string: &'static str = "([glad to see you])";
    println!("String: {}", test_string);

    // reverse the string
    println!("reverse the words based on the white space");
    for word in test_string.split_whitespace().rev() {
        println!("> {}", word);
    }

    // cpoy one char to the vector, and sort the string, then remove the duplicated words
    let mut chars: Vec<char> = test_string.chars().collect();
    //chars.sort();
    //chars.dedup(); // remove the duplicated words


    // create a string
    let mut string = String::new();
    for c in chars {
        // insert a char at the last index
        string.push(c);
        // insert a "string" at the last index
        string.push_str(", ");
    }

    println!("string: {}", string);

    //
    let trimmed_str: &str = string.trim_end_matches(", ], ), ");
    let trimmed_str2: &str = trimmed_str.trim_start_matches("(, [,");

    println!("Used characters: {}", trimmed_str2);

    // test
    let a = String::from("glad to see");
    let b: String = a.replace("see", "punch");

    println!("a: {}", a);
    println!("b: {}", b);
}



