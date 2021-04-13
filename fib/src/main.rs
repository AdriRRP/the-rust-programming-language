use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    match args.len() {
        1 => usage(),
        2 => {
            let n: u128 = match args[1].parse() {
                Ok(v) => v,
                Err(_) => {
                    println!("error: argument isn't an unsigned int max 128 bits");
                    usage();
                    return;
                },
            };

            println!("{}", fib(n));
        },
        _ => {
            println!("error: arguments doesn'r match");
            usage();
        },
    }
}

fn usage() {
    println!("USAGE");
    println!("\tfib n");
    println!("\t\treturn nth fibonacci number");
    println!("\t\tn must be an unsigned int");
}

fn fib(n: u128) -> u128{
    let mut n_minus_1: u128 = 0;
    let mut n_minus_2: u128 = 1;

    if n < 2 {
        return n;
    }

    for i in 0..n + 1 {
        if i > 2 {
            let n_new = n_minus_2 + n_minus_1;
            n_minus_1 = n_minus_2;
            n_minus_2 = n_new;
        }
    }
    return n_minus_2 + n_minus_1;

}
