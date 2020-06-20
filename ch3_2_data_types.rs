// This sections covers data types
//
#![allow(unused_variables)]

fn main() {
    const MAX_POINTS: u32 = 100_000;
    println!("{}", MAX_POINTS);
    let guess: u32 = "42".parse().expect("Not a number");

    let t = true;
    //Need the : after var name to assign type
    let f: bool = false;
    println!("True: {}, False {}", t, f);

    //Varilable result of an expression
    let sum = 392 + 30894;
    println!("392 + 30894 = {}",sum);
    println!("392 + 30894 = {}", 392 + 30894);

    let c = 'z';
    let z = 'ℤ';
    let heart_eyed_cat = '😻';
    println!("{}",heart_eyed_cat);

    //Tuple
    let tup = (500.0, "abc", 123);
    let (a, b, c) = tup;
    println!("The value of y is: {}", b);

    //Array
    let d: [i32; 5] = [1, 2, 3, 4, 5];
    let e = [6; 10];
    println!("{:?}", d);
    println!("{:?}", e);



}

