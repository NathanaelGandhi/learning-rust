// https://doc.rust-lang.org/stable/book/ch03-01-variables-and-mutability.html
const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

pub fn demo_variables() {
    println!("--- Variables ---");
    let mut x = THREE_HOURS_IN_SECONDS;
    println!("The value of x is: {x}");
    x = 2;
    println!("The value of x is: {x}");

    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");

    let spaces = "   "; // 3x space characters
    let spaces = spaces.len();

    println!("The number of spaces is: {spaces}");
}
