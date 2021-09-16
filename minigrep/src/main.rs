use std::process;
use std::env;
use std::fs;
use std::error::Error;

fn main() {
  let args: Vec<String> = env::args().collect();

  let config  = Config::new(&args).unwrap_or_else(|err| {
      println!("Problem parsing arguments: {}", err);
      process::exit(1);
  });

  println!("Searching for {}", config.query);
  println!("In file {}", config.filename);

  run(config);

}

struct Config {
  query: String,
  filename: String,
}

impl Config {
  fn new(args: &[String]) -> Result<Config, &str> {
    if args.len() < 3 {
      return Err("not enought arguments");
    }
    
    let query = args[1].clone();
    let filename = args[2].clone();
  
    Ok(Config { query, filename })
  }
}

fn run(config: Config) -> Result<(), Box<dyn Error>> {
  let contents = fs::read_to_string(config.filename)?;

  println!("With text:\n{}", contents);

  Ok(())
}
