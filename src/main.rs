/******************************************************************************
 * @Author                : iustus<lindgreia@gmail.com>                       *
 * @CreatedDate           : 2023-03-09 11:33:54                               *
 * @LastEditors           : iustus<lindgreia@gmail.com>                       *
 * @LastEditDate          : 2023-03-10 11:07:02                               *
 * @FilePath              : variables/src/main.rs                             *
 * @CopyRight             : MerBleueAviation                                  *
 *****************************************************************************/

/*
   const THREE_HOURS_IN_SECONDS : u32 = 60 * 60 * 3;
   We can define constants in the global scope.
*/
use std::io;


fn main() {
    let mut x = 5;
    println!("The value of x is : {x}");
    x = 6;
    println!("The value of x is : {x}");

    //this is shadowing because we reuse the `let` keyword
    let y = 4;

    let y = y + 1;

    {
        let y = y * 2;

        println!("The value of y in the inner scope is : {y}");
    }

    println!("The value of y is {y}");

    let spaces = "    ";
    let spaces = spaces.len();
    println!("{spaces}");
    // use shadowing for type casting, and use `mut` for normal val replacements

    // TYPES:

    //Scalar Types : a single value.
    //Integer Scalar Types:

    // let eightbitint:i8 = 1;
    // let sixteenbitint:i16 = 1;
    // let thirtytwobitint:i32 = 1;
    // let sixtyfourbitint:i64 = 1;
    // let onehundredandtwentyeightbitint:i128 = 1;

    let x = 2.0; // f64

    let y: f32 = 3.0; // f32

    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3; // Results in -1

    // remainder
    let remainder = 43 % 5;

    let t = true;
    let f: bool = false;

    let c = 'z';
    let z: char = 'ℤ'; // with explicit type annotation
    let heart_eyed_cat = '😻';

    //Compound Types
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;

    println!("The value of y is: {y}");

    let x: (i32, f64, u8) = (500, 6.4, 1);

    let five_hundred = x.0;

    let six_point_four = x.1;

    let one = x.2;

    let a = [1, 2, 4, 5, 6];
    let months = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];
    
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    let a = [3; 5];

    let a = [1,2,3,4,5];

    println!("Please enter an array index.");
    
    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");
    
    let index: usize = index
        .trim()
        .parse()
        .expect("index entered was not a number");
    
    let element = a[index];

    println!("The value of element at index {index} is {element}");




}
