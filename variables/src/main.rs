
fn main() {
    let mut x = 5;
    println!("The value of x is:{}", x);
    x = 6;
    println!("The value of x is:{}", x);
    
    const MAX_POINTS: u32 = 100_000;

    println!("The max point is:{}", MAX_POINTS);

    let x = 5;

    let x: i32 = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {}", x);
    }

    println!("The value of x is:{}", x)


}
