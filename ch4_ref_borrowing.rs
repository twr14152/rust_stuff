//Having reference to a value is called borrowing

fn main() {
    let s1 = String::from("complex string variable");
    // &s creates a reference to the value of s1
    // It does not own it.
    // The value it points to will not be dropped when ref 
    // goes out of scope
    let len = calculate_length(&s1); //&s1 creates reference to val at s1

    println!("The length of '{}' is {}.", s1, len);

    println!("Try and have borrower modify a value");
    let mut s2 = String::from("Inital value");
    println!("value of s2 to start: {} ", s2);
    change(&mut s2);
    println!("value of s2 after change(): {}", s2);

}
fn calculate_length(s: &String) -> usize { // s is a ref to a string
    s.len()
} // s goes out of scope but since its a reference (not ownership) nothing happens


fn change(some_var: &mut String) {
    some_var.push_str(" + added secondary value through ref");
}
