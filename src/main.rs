// using https://www.youtube.com/watch?v=BpPEoZW5IiY

fn main() {
    println!("Hello, world!");
    let a_number: i32 = 6;
    println!("Printing a number: {}", a_number);
    my_function();
    println!("a number + 10: {}", add(a_number, 10))
}

fn my_function(){
    println!("now in 'my_function'");
}

fn add(a: i32, b: i32)-> i32{
    return a + b;
}