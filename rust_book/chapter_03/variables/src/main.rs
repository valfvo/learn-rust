const _THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

fn main() {
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");

    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");

    let _spaces = "   ";
    let _spaces = _spaces.len();

    let _n = 98_222;
    let _n = 0xffu16;
    let _n = 0o77;
    let _n = 0b1111_0000;
    let _n = b'A';

    let _x = 2.0; // f64
    let _y: f32 = 3.0; // f32

    let _truncated = -5 / 3; // Results in -1

    // Rust’s char type is 4 bytes in size and represents a Unicode scalar value
    let _c = 'z';
    let _z: char = 'ℤ'; // with explicit type annotation
    let _heart_eyed_cat = '😻';

    // Compound types can group multiple values into one type.
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    let (_x, _y, _z) = tup;
    let _five_hundred = tup.0;
    let _six_point_four = tup.1;
    let _one = tup.2;

    // The tuple without any values has a special name, unit.
    let _u: () = ();

    // arrays in Rust have a fixed length, allocated on the stack
    let _a: [i32; 5] = [1, 2, 3, 4, 5];
    let _a = [3; 5];

    let _first = _a[0];
    let _second = _a[1];
}

use std::io;

fn _main2() {
    let a = [1, 2, 3, 4, 5];

    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!("The value of the element at index {index} is: {element}");
}

// Rust doesn’t care where you define your functions, only that they’re defined
// somewhere in a scope that can be seen by the caller.
fn _another_function(x: i32) {
    println!("Another function. The value of x is: {x}");
}

fn _print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

// Function bodies are made up of a series of statements optionally ending in an expression.
// - Statements are instructions that perform some action and do not return a value.
// - Expressions evaluate to a resultant value.
// Function definitions are statements
// A new scope block created with curly brackets is an expression

fn _main3() {
    let y = {
        let x = 3;
        x + 1
    };

    let _r: () = _another_function(y);

    println!("The value of y is: {y}");  // 4
}

// Expressions do not include ending semicolons.
// If you add a semicolon to the end of an expression, you turn it into a statement.

// In Rust, the return value of the function is synonymous with the value of the final expression
// in the block of the body of a function.
// You can return early from a function by using the return keyword

fn _five() -> i32 {
    5
}
