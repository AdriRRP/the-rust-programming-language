use std::collections::HashMap;
use std::io;

enum Command {
    Retrieve(String),
    RetrieveAll,
    Add(String, String),
    Quit,
    Unknown,
}

fn main() {
    let mut db = HashMap::new();

    loop {
        let cmd = request();
        execute(cmd, &mut db);
    }
}

fn request() -> Command {
    println!("\n\n\nAvailable commands:\n\n\tadd [employee] to [department]");
    println!("\n\tretrieve [department]");
    println!("\n\t\t- retrieve a list of all people in a department");
    println!("\n\tretrieve all");
    println!("\n\t\t- retrieve a list of all people in the company");
    println!("\n\tquit\n\n\n");

    let mut command = String::new();

    io::stdin()
        .read_line(&mut command)
        .expect("Failed to read line");

    let mut cmd_iter = command.split_whitespace();

    match cmd_iter.next() {
        Some("add") => match cmd_iter.next() {
            Some(employee) => match cmd_iter.next() {
                Some("to") => match cmd_iter.next() {
                    Some(department) => match cmd_iter.next() {
                        None => Command::Add(employee.to_string(), department.to_string()),
                        _ => Command::Unknown,
                    },
                    _ => Command::Unknown,
                },
                _ => Command::Unknown,
            },
            _ => Command::Unknown,
        },
        Some("retrieve") => match cmd_iter.next() {
            Some("all") => Command::RetrieveAll,
            Some(department) => match cmd_iter.next() {
                None => Command::Retrieve(department.to_string()),
                _ => Command::Unknown,
            },
            _ => Command::Unknown,
        },
        Some("quit") => Command::Quit,
        _ => Command::Unknown,
    }
}

fn execute(c: Command, db: &mut HashMap<String, Vec<String>>) -> () {
    match c {
        Command::Retrieve(department) => match db.get(&department) {
            Some(vec) => {
                let mut sorted_vec = vec[..].to_vec();

                sorted_vec.sort();

                println!("DEPARTMENT - {}\n{:?}", department, sorted_vec);
            }
            _ => println!("Department {} not found", department),
        },
        Command::RetrieveAll => {
            let mut departs_sorted: Vec<&String> = db.keys().collect();

            departs_sorted.sort();

            for department in departs_sorted {
                match db.get(department) {
                    Some(vec) => {
                        let mut sorted_vec = vec[..].to_vec();

                        sorted_vec.sort();

                        println!("DEPARTMENT - {}\n{:?}", department, sorted_vec);
                    }
                    _ => println!("Department {} not found", department),
                }
            }
        }
        Command::Add(employee, department) => {
            let dep = db.entry(department).or_insert(Vec::new());
            dep.push(employee);
        }
        Command::Quit => std::process::exit(0x0),
        Command::Unknown => println!("Unknown command"),
    }
}
