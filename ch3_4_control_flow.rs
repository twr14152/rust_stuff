fn main() {
    // Fizz-buzz-like-thingy
    for num in 1..20 {
        if num % 3 == 0 && num % 5 == 0 {
            println!("fizz buzz");
        } else if num % 3 == 0 {
            println!("fizz");
        } else if num % 5 == 0 {
            println!("buzz")
        } else  {
            println!("{}", num);
        }
    }
    println!("****************************");
    for num in (1..20).rev() {
        if num % 2 == 0 && num % 5 == 0 {
            println!("{} divisible by 2 and 5", num);
        } else if num % 2 == 0 {
            println!("{} divisible by 2", num);
        } else if num % 5 == 0 {
            println!("{} divisible by 5", num);
        } else {
            println!("{}", num);
        }
}
}
