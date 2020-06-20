// Functions

fn main() {
    println!("This is the main function");
    secondary_func();
    let a = return_func(6909, 745);
    println!("{}", a)
}
fn secondary_func() {
    let a = 2;
    let b = 6;
    println!("{} + {} = {}", a, b, a + b);
}
// if you put a ';' after x*y it changes from expr to statement and wont compile
// return is identified by '->'
fn return_func(x: i32, y: i32) -> i32 {
    x * y
}
