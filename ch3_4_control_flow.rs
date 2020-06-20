fn main() {
    // Fizz-buzz-like-thingy
    for num in 1..100 {
        if num % 3 == 0 && num % 5 == 0 {
            println!("fizz buzz");
        } else if num % 3 == 0 || num % 5 == 0 {
            println!("fizz");
        } else  {
            println!("{}", num);
        }
    }

}
