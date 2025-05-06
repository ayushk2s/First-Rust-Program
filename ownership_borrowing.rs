fn main() {
    let s1 = String::from("hello"); // s1 owns the String
    let s2 = s1;                    // ownership moves to s2, s1 is no longer valid

    // println!("{}", s1); // Error! s1 no longer owns the value

    println!("Ownership moved to s2: {}", s2);

    let s3 = String::from("world");

    // Pass a reference (&s3), this is borrowing, s3 keeps ownership
    print_length(&s3);

    println!("After borrowing, s3 is still valid: {}", s3);

    // === Mutable Borrowing Example ===
    let mut s4 = String::from("Rust");

    // Pass a mutable reference
    append_exclamation(&mut s4);

    println!("After mutable borrow: {}", s4);
}

// This function borrows the string immutably
fn print_length(s: &String) {
    println!("Length is: {}", s.len());
    // s is only borrowed, not moved
}

// This function borrows the string mutably
fn append_exclamation(s: &mut String) {
    s.push_str("!");
}
