fn main() {
    // Calling a function that returns a char
    let y = return_fun();
    println!("Returned char: {y}");

    // Using a mutable variable
    let mut z = 10;
    println!("Original z: {z}");
    z = change_value(z);
    println!("Changed z: {z}");

    // Calling a function with a conditional return type
    let result = check_number(15);
    println!("Check number result: {result}");

    // Function with tuple return type
    let coordinates = get_coordinates();
    println!("Coordinates: ({}, {})", coordinates.0, coordinates.1);

    // Passing a reference to a function
    let name = String::from("Rust");
    greet(&name);
}

// Function returning a char
fn return_fun() -> char {
    'h'
}

// Function that changes the value and returns it
fn change_value(x: i32) -> i32 {
    x * 2
}

// Function with conditional logic
fn check_number(x: i32) -> &'static str {
    if x > 10 {
        "Greater than 10"
    } else {
        "Less or equal to 10"
    }
}

// Function returning a tuple
fn get_coordinates() -> (i32, i32) {
    (5, 10)
}

// Function taking a reference to a String
fn greet(name: &str) {
    println!("Hello, {name}!");
}
