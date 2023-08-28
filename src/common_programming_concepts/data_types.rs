// https://doc.rust-lang.org/stable/book/ch03-02-data-types.html
pub fn demo_data_types() {
    println!("--- Data Types ---");
    // Floating-Point Types
    let x = 2.0; // f64
    let y: f32 = 3.0; // f32

    // addition
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

    // The Boolean Type
    let t = true;
    let f: bool = false; // with explicit type annotation

    // The Character Type
    let c = 'z';
    let z: char = 'ℤ'; // with explicit type annotation
    let heart_eyed_cat = '😻';

    // The Tuple Type
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let tup = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("The value of y is: {y}");

    let x: (i32, f64, u8) = (500, 6.4, 1);
    let five_hundred = x.0;
    let six_point_four = x.1;
    let one = x.2;

    // The Array Type
    let a = [1, 2, 3, 4, 5];
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
    // declare and initialise
    let a = [3; 5]; // 5 elements of value 3
    let a = [1, 2, 3, 4, 5];
    let first = a[0];
    let second = a[1];

    // When you attempt to access an element using indexing, Rust will check
    // that the index you’ve specified is less than the array length. If the
    // index is greater than or equal to the length, Rust will panic. This
    // check has to happen at runtime
}
