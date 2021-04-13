use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    match args.len() {
        1 => {
           usage();
        },
        3 => {
            let val: f32 = match args[1].parse() {
                Ok(n) => {
                    n
                },
                Err(_) => {
                    println!("error: first argument not floating point number");
                    usage();
                    return;
                },
            };

            let unit: bool = match &args[2][..] {
                "farenheit" | "f" | "F" => {
                   true 
                },
                "celsius" | "c" | "C" => {
                   false 
                },
                _ => {
                    println!("error: second argument not valid unit");
                    usage();
                    return;
                },
            };
            
            let result_unit: &str = if unit { "celsius" } else { "farenheit" };
            let result: f32 = far2cel(val, unit);

            println!("{} {}", result, result_unit);
        },
        _ => {
            println!("error: arguments doesn't match");
            usage();
            return;
        },
    };
}

fn usage() {
    println!("farenheti2celsius [float] [unit]");
    println!("\t[float]: floating point number i.e 3.25");
    println!("\t[unit]: farenheit, f, F or celsius, c, C");
    println!("\tEXAMPLE: farenheit2celsius 23 F");
}

fn far2cel(val: f32, farenheit: bool) -> f32 {
    if farenheit {
        (val - 32.0) / 1.8
    } else {
        (val * 1.8) + 32.0
    }
}
