fn main() {
    println!("Hello, world!");

    let number = 7;

    if number < 5 {
        println!("The condition was true");
    } else {
        println!("condition was false");
    }

    let number = 17;

    if number % 4 == 0{
        println!("number is divisible by 4");
    } else if number % 3 == 0{        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    let condition = false;

    let number = if condition { 5 } else { 6 };

    println!("The value of number is : {}", number);

    println!("Start Loop");

    let mut num :u16 = 0;
    loop {
        println!("again! {}", num);
        num += 1;
        if num == 100 {
            break;
        }
    }
    println!("End loop");

    num = 20;

    println!("Start while loop");

    while num != 0 {
        println!("Reaming:{}", num);
        num -= 1;
    }

    println!("End while loop");

    let loop_list = 1..600;

    println!("for loop start");

    for i in loop_list {
        println!("the value is : {}", i);
    }
    println!("end for loop");

}
