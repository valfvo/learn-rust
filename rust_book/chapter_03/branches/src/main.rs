fn main() {
    let number = 3;

    // blocks of code associated with the conditions in if expressions are sometimes called arms,
    // just like the arms in match
    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    if number != 0 {
        println!("number was something other than zero");
    }

    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    let condition: bool = true;
    let number = if condition { 5 } else { 6 };
    println!("The value of number is: {number}");

    // Repetition with Loops
    loop {
        println!("again!");
        break;
    }

    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {result}");

    // Loop labels must begin with a single quote
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");

    // while
    let mut number = 3;

    while number != 0 {
        println!("{number}!");

        number -= 1;
    }

    println!("LIFTOFF!!!");

    // Looping Through a Collection with for
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", a[index]);

        index += 1;
    }

    for element in a {
        println!("the value is: {element}");
    }

    // using Range
    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");

    _convert_degrees(100, true);
    _fibonacci(10);
    _lyrics();
}

fn _convert_degrees(degrees: i32, is_celsius: bool) -> i32 {
    if is_celsius {
        (degrees * 9 / 5) + 32
    } else {
        (degrees - 32) * 5 / 9
    }
}

fn _fibonacci(n: u32) -> u32 {
    match n {
        0 => 0,
        1 => 1,
        _ => _fibonacci(n - 1) + _fibonacci(n - 2),
    }
}

fn _lyrics() {
    let days = [
        "first",
        "second",
        "third",
        "fourth",
        "fifth",
        "sixth",
        "seventh",
        "eighth",
        "ninth",
        "tenth",
        "eleventh",
        "twelfth",
    ];

    let gifts = [
        "A Partridge in a Pear Tree.",
        "Two Turtle Doves,",
        "Three French Hens,",
        "Four Calling Birds,",
        "Five Gold Rings,",
        "Six Geese-a-Laying,",
        "Seven Swans-a-Swimming,",
        "Eight Maids-a-Milking,",
        "Nine Ladies Dancing,",
        "Ten Lords-a-Leaping,",
        "Eleven Pipers Piping,",
        "Twelve Drummers Drumming,",
    ];

    for i in 0..days.len() {
        println!("On the {} day of Christmas my true love sent to me:", days[i]);

        for j in (0..=i).rev() {
            if i != 0 && j == 0 {
                print!("And ");
            }
            println!("{}", gifts[j]);
        }

        println!();
    }
}
