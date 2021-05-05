use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    match args.len() {
        1 => {
            usage();
        }
        2 => {
            let input = &args[1];
            println!("{:?}", to_pig_latin(&input[..]));
        }
        _ => {
            println!("error: arguments doesn't match");
            usage();
            return;
        }
    };
}

fn is_vowel(c: &char) -> bool {
    match c {
        'a' | 'e' | 'i' | 'o' | 'u' => true,
        'A' | 'E' | 'I' | 'O' | 'U' => true,
        'á' | 'é' | 'í' | 'ó' | 'ú' => true,
        'Á' | 'É' | 'Í' | 'Ó' | 'Ú' => true,
        _ => false,
    }
}

fn is_consonant(c: &char) -> bool {
    match c {
        'b' | 'c' | 'd' | 'f' | 'g' | 'h' | 'j' | 'k' | 'l' | 'm' | 'n' | 'ñ' | 'p' | 'q' | 'r'
        | 's' | 't' | 'v' | 'w' | 'x' | 'y' | 'z' => true,
        'B' | 'C' | 'D' | 'F' | 'G' | 'H' | 'J' | 'K' | 'L' | 'M' | 'N' | 'Ñ' | 'P' | 'Q' | 'R'
        | 'S' | 'T' | 'V' | 'W' | 'X' | 'Y' | 'Z' => true,
        _ => false,
    }
}

fn to_pig_latin(s: &str) -> String {
    let mut result = String::new();

    if s.len() > 0 {
        let first_char: &char = &s[..1].chars().next().unwrap();

        if is_consonant(first_char) {
            result.push_str(&s[1..]);
            result.push('-');
            result.push(*first_char);
            result.push_str("ay");
        } else {
            result.push_str(&s);

            if is_vowel(first_char) {
                result.push_str("-hay");
            }
        }
    }

    result
}

fn usage() {
    println!("USAGE: string2piglatin [utf8-string]");
}
