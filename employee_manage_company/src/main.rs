use std::collections::HashMap;
use std::cmp::Ordering;
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

fn status(db: &HashMap<String, Vec<String>>) {
    println!("COMPANY STATUS\n\n{:?}\n\n", db);
}

fn request() -> Command {
    println!("Available commands:\n\n\tAdd [employee] to [department]");
    println!("\n\tRetrieve [department]");
    println!("\n\t\t- retrieve a list of all people in a department");
    println!("\n\tRetrieve all");
    println!("\n\t\t- retrieve a list of all people in the company");
    println!("\n\tQuit");

    let mut command = String::new();

    io::stdin()
        .read_line(&mut command)
        .expect("Failed to read line");

    let mut cmdIter = command.split_whitespace();

    match cmdIter.next() {
        Some("Add") => {
            match cmdIter.next() {
                Some(employee) => {
                    match cmdIter.next() {
                        Some("to") => {
                            match cmdIter.next() {
                                Some(department) => {
                                    match cmdIter.next() {
                                        None => Command::Add(employee.to_string(), department.to_string()),
                                        _ => Command::Unknown,
                                    }
                                },
                                _ => Command::Unknown,
                            }
                        },
                        _ => Command::Unknown,
                    }
                },
                _ => Command::Unknown,
            }
        },
        Some("Retrieve") => {
            match cmdIter.next() {
                Some("all") => Command::RetrieveAll,
                Some(department) => {
                    match cmdIter.next() {
                        None => Command::Retrieve(department.to_string()),
                        _ => Command::Unknown,
                    }
                },
                _ => Command::Unknown,
            }
        },
        Some("Quit") => Command::Quit,
        _ => Command::Unknown,
    }
}

fn execute(c: Command, db: &mut HashMap<String, Vec<String>>) -> () {
    match c {
        Command::Retrieve(department) => {
            match db.get(&department) {
                Some(vec) => {
                    let mut sorted_vec = vec[..].to_vec();

                    sorted_vec.sort();

                    println!("DEPARTMENT - {}\n{:?}", department, sorted_vec);
                },
                _ => println!("Department {} not found", department),
            }
        },
        Command::RetrieveAll => {
            let departs_sorted: Vec<String> = db.keys().collect().to_vec();
            departs_sorted.sort();

            println!("\n\n{:?}\n\n", db)
        },
        Command::Add(employee, department) => {
            let dep = db.entry(department).or_insert(Vec::new());
            dep.push(employee);
        },
        Command::Quit => std::process::exit(0x0),
        Command::Unknown => println!("Unknown command"),
    }
}
