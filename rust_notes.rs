// My notes from reading the Rust Book 2018
// Reread Chapter on enums
////use std::io;

#[derive(Debug)] // Needed to see output from println!()
struct Router {
    hostname: String,
    vendor: String,
    model: String,
    mgmt_ip: String,
    active: bool,
}
#[derive(Debug)] // added line to see contents in println! cmd
struct Rectangle {
    width: u32,
    height: u32,
}
// Method
// This is the implementation block
// area method (function) is defined with in it
// self parameter is assigned (in this case &self = Rectangle)
// &self will borrow data from the Rectangle struct to read into
// memory not to change it. If we wanted to change data it would 
// be changed to fn area (&mut self) -> u32 {
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

//Enum
//This is a rather lack luster example of this feature
//This ones on me
#[derive(Debug)]
enum Books {
    Pbook(String),
    Ebook(String),
}
fn main() {
    println!("Hello world!");
    println!("Testing another_function(5)");
    another_function(5);
    println!("Testing mutability of vars");
    let mut a = 4;
    let b = 5;
    println!("a = {}, b = {}", a, b);
    println!("Testing var formating and addition");
    a = 89;
    println!("a which is {} + b which is {} = {}", a, b, a + b);
    println!("This is testing spacing on vars 98_222 should = 98222");
    let y = 98_222;
    println!("y = {}", y);
    println!("This is a tuple (a, b, c, d)");
    let tup = ("a", "b", "c", "d");
    println!("This is the value of tup.2 = {}", tup.2);
    println!("This is the value of tup.3 = {}", tup.3);
    println!("This is an array [1, 2, 3, 4] ");
    let a_rey = [1, 2, 3, 4];
    let index = 3;
    let element = a_rey[index];
    println!("This should print 4 out of [1, 2, 3, 4]...{}", element);
    println!("More function fun!");
    println!("second_function(10, 15)");
    second_function(10, 15);
    println!("To return value with func we use -> value type ");
    println!("In next example let t = five(); in fn five() -> i32");
    println!("In fn five() in the body of the fn there is no ;");
    let t = five();
    println!("The value of t is: {}", t);
    println!("next is Control flow, if/else statements");
    let number = 7;
    if number < 5 {
        println!("number {} is less than 5", number);
    } else {
        println!("condition {} is not less than 5", number);
    }
    println!("Rust has 3 loops (loop, for, while)");
    println!("loop will execute for ever or until you tell it to stop");
    let num = 10;
    let mut i = 0;
    loop {
        i = i + 1;
        println!("{}", i);
        if i == num {
            println!("num = 10 break!");
            break;
        };
    }
    println!("While loops will run while condition is true");
    let mut number5 = 5;
    while number5 != 0 {
        println!("{}", number5);
        number5 -= 1;
    }
    println!("Blastoff!");
    println!("Using for loops to iterate through collections");
    let calendar = [
        "Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec",
    ];
    for month in calendar.iter() {
        println!("The month is: {}", month);
    }
    println!("Using for loop with rev method");
    for number_a in (1..5).rev() {
        println!("{}", number_a);
    }
    println!("Liftoff!!!");
    println!("Stack memory like a stack of dishes LIFO, known fixed size");
    println!(
        "Heap memory is less organized, use ptr to provide address, follow the ptr to address"
    );
    println!("string literal immutable stored in stack");
    println!("another type of string thats stored in heap");
    let str = r##"let mut s = String::from("Hello")"##;
    println!("{}", str);
    println!("(::) is an operator that allows us to use namespace from");
    println!("using this string type allows mutation");
    let mut s = String::from("hello");
    let str = r##"s.push_str(", world")"##;
    println!("{}", str);
    s.push_str(", world");
    let str = r##"println("{}", s);"##;
    println!("{}", str);
    println!("{}", s);
    println!("Strings can be mutated literals cannot.");
    println!("String literals use stack memory is a fixed size.");
    println!("String type allocate memory on the heap unknown at compile time");
    println!("Memory is requested from OS at runtime.");
    println!("We need to return this memory to the OS when we are done w/ String");
    println!("Returning memory is done with its out of scope or on closing bracket");
    println!("Rust calls drop function automatically, at the curly bracket.");
    println!("s2 = s, does a shallow copy. Heap data is shared, while stack data copied");
    println!("Rust calls the shallow copy a move");
    println!("When s1 goes out of scope stack is removed but shared heap still works for s2");
    println!("S2 will free memory when it goes out of scope.");
    println!("Shallow copies or moves are inexpensive to memory, and are automatic.");
    println!("Clone can be used to perform deep copy of heap data");
    let str = r##"let s1 = String::from("Hello");"##;
    println!("{}", str);
    println!("let s2 = s1.clone();");
    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("s1 = {}, s2 = {}", s1, s2);
    println!("Stack only data copy let x1 = 5; let x2 = x1;");
    let x1 = 5;
    let x2 = x1;
    println!("x1 = {}, x2 = {}", x1, x2);
    println!("'&' = Reference '*'= Dereference ");
    println!("Referencing allows you to refer to a value without owning it");
    println!(
        "This means the value referenced will not be dropped when reference goes out of scope."
    );
    let s1 = String::from("This is some cool shit");
    let len = calc_len(&s1);
    println!("The length of '{}' is {}.", s1, len);
    println!("What happens when you try and modify a borrowed value change(&s)? Nothing it errors out, immutable.... to change that you say change(&mut s)");
    let mut s = String::from("Hello");
    println!("The starting value of s is: {}", s);
    change(&mut s);
    println!("One restriction on mutable data,  only one reference per piece of data in scope");
    println!("Meaning let r1 = &mut s; only cannot follow with");
    println!("        let r2 = &mut s; Not allowed!");
    println!("The way around this is to use curly brackets around these to start and end scope");

    let mut ts1 = String::from("This is a test");
    println!("value ts1: {}", ts1);
    {
        let ts2 = &mut ts1;
        println!("value ts2: {}", ts2);
    } // ts2 goes out of scope here so ts3 is now safe to start referencing
    let ts3 = &mut ts1;
    println!("value ts3: {}", ts3);
    println!("Another restriction you cannot combine mut and immut in one statement");
    println!(
        "Rules to references: you can have 1 mutable refence, or any number of immutable refences."
    );
    println!("Refences must always be valid, you cant return -> '&string' '&s'  of something thats gone out of scope");
    println!("It would be pointing to invalid string");
    println!("It needs to return String directly '-> String' and 's'");
    println!("Using Structs");
    let core_r1 = Router {
        hostname: String::from("core_r1"),
        vendor: String::from("arista"),
        model: String::from("7050"),
        mgmt_ip: String::from("10.0.1.20"),
        active: true,
    };
    println!("{:?}", core_r1);
    println!("core_r1 mgmt_ip: {}",core_r1.mgmt_ip);
    let replacement = Router {
        ..core_r1
    };
    println!("replacement :{:?}", replacement);
    let rect1 = Rectangle {width: 30, height: 50};
    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rect1)
    );
    println!("This is a test to print the contents of rect1");
    println!("rect1 is {:#?}", rect1);
    println!("rect1 is {:?}", rect1);
    println!("**************************");
    println!("Switching topic to methods");
    println!("**************************");
    // This is the method syntax for calling area method
    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
        );
    let rect1 = Rectangle { width: 30, height: 50 };
    let rect2 = Rectangle { width: 10, height: 40 };
    let rect3 = Rectangle { width: 60, height: 45 };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
    println!("******************************");
    println!("Topic enum                    ");
    println!("******************************");
    let rust_book = Books::Pbook(String::from("The Rust Book"));
    let rust_e_book = Books::Ebook(String::from("The Rust e-Book"));
    println!("{:?}", rust_book);
    println!("{:?}", rust_e_book);
    let tgpl = Books::Pbook(String::from("The Go Programming Language"));
    println!("Very boring book {:?}",(tgpl));


}

fn another_function(x: i32) {
    println!("The value of x is: {}", x);
}
fn second_function(x: i32, y: i32) {
    println!("The value of x is : {}", x);
    println!("The value of y is : {}", y);
    println!("x * y = {}", x * y);
}

fn five() -> i32 {
    5
}

fn calc_len(s: &String) -> usize {
    s.len()
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
    println!("The value of s after change(): {}", some_string)
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
