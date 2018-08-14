use std::iter::{Skip, Take};

fn main() {
    let mut my_string = String::new();
    my_string.push('a');
    println!("{}", my_string);

    let my_string_slice = "hey";
    my_string.push_str(my_string_slice);

    println!("{}", my_string);

    print_a_string_slice(&my_string);
    print_a_string_slice(&my_string_slice);
    print_a_string_slice(&my_string);

    let harry = String::from("You are a Rust coder, Harry.");
    harry.split(" ").iter().Skip(1).Take(1);
}

fn print_a_string_slice(print_me: &str) {
    println!("{}", print_me);
}
