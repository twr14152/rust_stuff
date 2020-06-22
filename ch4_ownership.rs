// Data stored on the stack - known fixed size data
// Data stored on the heap -  unknown size at compile time or size that might change
// Accessing data in the heap is slower than stack
// Heap uses pointers to get data
// A function's local variables get pushed to the stack
// when the function ends the values get popped
//


fn main() { 
    // string literal var is stored on the stack LIFO 
    // string literal we know size at time of compile 
    // string literals are in code at compile time
    let s = "good bye!"; //--varible declared and in scope
    println!("{}", s); 
    secondary()
}//--variable out of scope


fn secondary() {
    // String Type - common use would take user input and store data
    // This data size would be unknown at compile time 
    // stored in heap
    // string can be mutated where string literal cannot
    // memory is requested at run time
    // memory must be returned when were done with our string
    // The drop function is used to return memory to our system
    // The closing '}' tells Rust to call the drop function
    let mut t = String::from("hello"); // allocated on heap - think restaurant seating
    t.push_str(", world!"); //appends a literal to the String
    println!("{}", t); 

    //copying vars - copy of the variable in the stack
    let x = 5;
    let y = x;
    println!("{} {}", x, y);
    //You dont copy data in the heap memory you copy the pointer to that data
    //along with the Name,  Length, and Capacity
    let s1 = String::from("Some where in memory");
    let s2 = s1.clone(); // without the .clone() a move (shallow copy) 
    // without the .clone() s1 data will have moved to s2 leaving s1 in error state
    // with this print statement. Only s2 would be valid
    // deep copy '.clone()' solves this.
    println!("original: {}\ncopy: {}", s1, s2); 
    //Summary stack only data can simply do a copy
    //Types that are of known size at time of compile can simply 'let y = x;'
    //To copy heap data must do a deep copy using .clone() method
    //
    //
    //All ints, bools, floats, char, tuples have copy trait. 'let y = x'
    //

    let s3 = String::from("String data for take_ownership"); // s3 comes into scope
    takes_ownership(s3); //s3 value moves into function 
                         //and is no longer valid here!

    let s4 = 5; // s4 comes into scope
    make_copy(s4); //s4 is i32 type and is a Copy in function
                   //s4 can still be used as its still in scope
    
    last_example()
    
} //s4 goes out of scope

fn takes_ownership(some_string: String) { //a string comes into scope
    println!("{}", some_string);
} // here the string goes out of scope and 'drop' is called memory is freed

fn make_copy(some_integer: i32) { //an integer comes into scope
    println!("{}", some_integer);
} // integer goes out of scope nothing special is called


fn last_example() {
    let s5 = String::from("Hello");
    let (s6, len) = calculate_length(s5);

    println!("The length of '{}' is {}.", s6, len);
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); 
    (s, length)
}
/*
good bye!
hello, world!
5 5
original: Some where in memory
copy: Some where in memory
String data for take_ownership
5
The length of 'Hello' is 5.
*/
