fn main() {
    println!("Hello, world!");

    another_function();
    another_function_with_param(15);
    another_function_with_params(10, 15);
    println!("Call function five() resusts in {}", five());
    println!("Call function plus_one(5) resusts in {}", plus_one(5));
}

fn another_function() {
    println!("Another function.");
}

fn another_function_with_param(x: i32) {
    println!("The value of x is {}", x);
}

fn another_function_with_params(x: i32, y: i32) {
    println!("The value of x is {}", x);
    println!("The value of y is {}", y);
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
