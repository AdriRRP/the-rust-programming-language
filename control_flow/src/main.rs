fn main() {
    let number = 3;
    
    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    if number != 0 {
        println!("number was something other than zero");
    }

    if number % 2 == 0 {
        println!("number is divisible by 2");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 4 == 0 {
        println!("number is divisible by 4");
    } else {
        println!("number is not divisible by 2, 3 or 4");
    }

    let condition = false;
    let number = if condition { 5 } else { 6 };

    println!("the value of number is {}", number);

    //loop {
    //    println!("again");
    //}

    let mut counter = 0;

    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
 
    println!("the value of result if {}", result);

    let mut number = 3;

    while number != 0 {
        println!("{}!", number);
 
        number -= 1;
    }

    println!("LITOFF!!!");

    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", a[index]);
        index += 1;
    }
     
    for element in a.iter() {
        println!("the value is {}", element);
    }
   
    for element in (1..4).rev() {
        println!("the value is {}", element);
    }
}
