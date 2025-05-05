// The entry point of the Rust program.
fn main() {
    // Define a variable `x` with type `i32` and value 5.
    let x: i32 = 5;
    // Print a greeting message and include the value of `x`.
    println!("Hello, world! {x}");

    // Define a variable `user` of type `&str` and assign it the value "Ayush".
    let user = "Ayush";
    // Call the first_function and pass the reference to the string `user`.
    first_function(user);

    // Define a variable `user_second` of type `String` and assign it a string value.
    let user_second = String::from("Krishna");
    // Call the second_function and pass the `String` object `user_second`.
    second_function(user_second);
}

// The first function accepts a string slice reference `&str` as input.
fn first_function(name: &str){
    // This was commented out: The idea was to create a greeting message, but it's incorrect syntax.
    // let greet = "Hello! {name}".to_string();
    // Correctly prints the greeting message using the `name` argument passed by reference.
    println!("Hello! {name}");
}

// The second function accepts a `String` object as input, not a reference.
fn second_function(name: String){
    // Prints the greeting message, using the `name` argument passed by value.
    println!("Hello! {name}");
}
