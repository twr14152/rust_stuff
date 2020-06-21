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
    println!("{}", t)

    //copying vars - copy of the variable in the stack
    let x = 5;
    let y = x;
    println!("{} {}",x,y);
    //You dont copy data in the heap memory you copy the pointer to that data
    //along with the Name Length and Capacity
    let s1 = String::from("Some where in memory");
    let s2 = s1; 
    println!("{} {}", s1, s2)
}

