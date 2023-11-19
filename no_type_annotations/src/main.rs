#![allow(unused)]
use std::io;
fn main() {
    let guess :u32 = "42".parse().expect("not a number");
    let test = 100_100_100;
    let x = 2.0;
    let y : f32 = 3.0;
    
    let sum = 5 + 10;

    let difference = 95.5 - 4.3;

    let product = 4 * 30;

    let quotient = 56.7 / 32.2;
    let floored = 2_f64 / 3_f64;
    let floored = 2 / 3 ;

    let remainer = 43 % 5;

    let t = true;

    let f : bool = false;

    let c = 'z';
    let z = 'ℤ';
    let heart_eyed_cat = '😻';  

    let tup: (i32, f64, u8) = (500, 6.4, 1);

    let tup = (500, 6.4, 1);

    let (x, y, z) = tup;

    println!("The value of y is: {}", y);

    let x = tup;

    let five_hundred = x.0;

    let six_point_four = x.1;

    let one = x.2;

    let a = [1, 2, 3, 4, 5];

    
    let months = ["January", "February", "March", "April", "May", "June", "July",
    "August", "September", "October", "November", "December"];    



    let a = [3; 5];
    let a: [i32; 5]   = [1, 2, 3, 4, 5];
   
    let  first = a[0];
    let second = a[1];

    let a = [1, 2, 3, 4, 5];

    println!("Please enter an array index.");
           // 配列の何番目の要素にアクセスするか指定してください

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");
              // 値の読み込みに失敗しました

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!(
        "The value of the element at index {} is: {}",
        // {}番目の要素の値は{}です
        index, element
    );

}
