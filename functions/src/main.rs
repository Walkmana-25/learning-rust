fn main() {
    println!("Hello, World!");
    another_function();
    another_function_2(89);
    print_labelded_measurement(5, 'h');

    let y = {
        let x = 3;
        x + 1
    };
    println!("The value of y is:{}",y);
    println!("The value of 5 is:{}",five());
    // comment line test
    println!("The value of 5 + 1 is:{}",plus_one(5)); //comment line test 3
}

/// 私だよ
/// watasidayo 
/// 
/// 
/// 
fn another_function(){
    println!("Another function");
}

fn another_function_2(x : i32){
    println!("The value of x is : {}", x);
}
fn print_labelded_measurement(value: i32, unit_lavel: char){
    println!("The measurement is : {}{}", value, unit_lavel);
}
fn five() -> i32 {
    5
}
fn plus_one(x: i32) -> i32 {
   x + 1 
}
