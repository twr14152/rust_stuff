// Var assignment and mutability
fn main() { 
    let x = 5;
    println!("The value of x is: {}. By default its immutable.", x);
    println!("
    reassignment by performing the following:
    x = 6; will cause a compile error
    ");
    println!("The value of x is {}. x = 6 is not allowed.", x);
    let mut y = 6;
    println!("
    let mut y = 6;
    The value of y is: {}. With the mut command this value can now be changed.", y);
    println!("
    y = 7;
    ");
    y = 7;
    println!("The value of y is now: {}.", y);
    println!("*********************");
    println!("If you use let and reassign x everytime it is allowed");
    let z = 10;
    let z = z + 1;
    let z = z + 1;
    println!("
    let z = 10;
    let z = z + 1;
    let z = z + 1;
    ");
    println!("The value of z is {}", z);
    let name = "Todd";
    println!("My name is {}", name);
    println!("There are {} letters in my name.", name.len());
}
/*pi@RaspPi4:~/Coding/Rust_folder/projects/rust_stuff $ ./ch3_var_mut 
The value of x is: 5. By default its immutable.

    reassignment by performing the following:
    x = 6; will cause a compile error
    
The value of x is 5. x = 6 is not allowed.

    let mut y = 6;
    The value of y is: 6. With the mut command this value can now be changed.

    y = 7;
    
The value of y is now: 7.
*********************
If you use let and reassign x everytime it is allowed

    let z = 10;
    let z = z + 1;
    let z = z + 1;
    
The value of z is 12
My name is Todd
There are 4 letters in my name.
*/

