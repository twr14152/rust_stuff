//Having reference to a value is called borrowing
//
//At any given time you can have either one mutable reference or multiple immutable references
//References must always be valid
//Quoted from "the book"

fn main() {
    let s1 = String::from("complex string variable");
    // &s creates a reference to the value of s1
    // It does not own it.
    // The value it points to will not be dropped when ref 
    // goes out of scope
    let len = calculate_length(&s1); //&s1 creates reference to val at s1

    println!("The length of '{}' is {}.\n", s1, len);
    println!("\n");
    println!("Have borrower modify a value.....\n");
    let mut s2 = String::from("Inital value");
    println!("Value of s2 to start:\n {} \n", s2);
    change(&mut s2);
    println!("Value of s2 after change():\n {}\n", s2);
    println!("\nMore fun will mutable vars and reference (borrowers)\n\n");
    let mut s3 = String::from("Whats up?");
    {
    let r1 = &mut s3;
    println!("{}", r1)
    } //r1 goes out of scope - so new ref can be made
    let r2 = &mut s3; 
    println!("{}", r2);

    println!("\nThis next section talks about immutable borrowers vs mutable\n\n");
    println!("\nYou cannot have mutable references while you have immutable references\n");
    let mut t1 = String::from("Hellllloooooo");

    let r3 = &t1; 
    let r4 = &t1;
    let r5 = &t1;

    println!("{}, {}, and {}\n", r3, r4, r5);
    
    let r6 = &mut t1;
    //let r7 = &mut t1; //cannot borrow `t1` as mutable more than once at a time
    //println!("{}, and {}\n", r6, r7); //cannot borrow `t1` as mutable more than once at a time
    println!("{}\n", r6);
}
fn calculate_length(s: &String) -> usize { // s is a ref to a string
    s.len()
} // s goes out of scope but since its a reference (not ownership) nothing happens


fn change(some_var: &mut String) {
    some_var.push_str(" + added secondary value through ref");
}

