fn main() {
    // Fizz-buzz-thingy
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
    println!("***************************");
    println!("If condition is True - issue body of clause");
    let number = 2;

    if number == 5 {
        println!("Number is 5");
    } else if number == 10 {
        println!("Number is 10");
    } else {
        println!("Number was {}", number);
    }
    println!("**************************");
    let condition = false; // if true == 10 if false == 6
    let nums = if condition { 10 } else { 6 };
    println!("The value of number is: {}", nums);
    println!("**********LOOP************");
    let mut counter = 0;
    let result = loop {
        counter += 2;

        if counter == 100 {
            break counter * 2
        }
    };
    println!("Break counter number is {}", result);
    println!("*********While***********");
    let mut numbers = 10;
    while numbers != 0 {
        println!("{}", numbers);
        numbers -= 1;
    }
    println!("LIFTOFF!!!");
    println!("**********FOR************");
    for i in (1..11).rev() {
        println!("{}", i);
        }
    println!("Blast off!!!");
}


