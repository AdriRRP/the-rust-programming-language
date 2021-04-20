fn main() {
    println!("{}", the_twelve_days_of_christmas());
}

fn the_twelve_days_of_christmas() -> String {
    let mut poem = String::new();

    for day in 1..13 {
        poem.push_str("On the ");
        poem.push_str(&ordinal(day));
        poem.push_str(" day of Christmas my true love sent to me\n");

        for prev_day in (2..day + 1).rev() {
            poem.push_str(&present(prev_day));
            poem.push_str(",\n");
        }

        if day > 1 {
            poem.push_str("and ");
        }

        poem.push_str(&present(1));
        poem.push_str(".\n\n");
    }
    return poem;
}

fn present(n: u8) -> String {
    return match n {
        1 => "a partridge in a pear tree",
        2 => "two turtle doves",
        3 => "three french hens",
        4 => "four calling birds",
        5 => "five golf rings",
        6 => "six geese a-laying",
        7 => "seven swans a-swimming",
        8 => "eight maids a-milking",
        9 => "nine ladies dancing",
        10 => "ten lords a-leaping",
        11 => "eleven pipers piping",
        12 => "twelve drummers drumming",
        _ => "???",
    }
    .to_string();
}

fn ordinal(n: u8) -> String {
    return match n {
        1 => "first",
        2 => "second",
        3 => "third",
        4 => "fourth",
        5 => "fifth",
        6 => "sixth",
        7 => "seventh",
        8 => "eighth",
        9 => "ninth",
        10 => "tenth",
        11 => "eleventh",
        12 => "twelfth",
        _ => "???",
    }
    .to_string();
}
