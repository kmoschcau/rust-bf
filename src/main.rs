extern crate regex;

use std::io::Read;
use std::path::Path;
use regex::Regex;

mod bf_interpreter;

use bf_interpreter::BfInterpreter;

fn main() {
    std::process::exit(match run() {
        Ok(_) => 0,
        Err(message) => {
            eprintln!("Error: {}", message);
            1
        }
    })
}

fn run() -> Result<(), String> {
    let args = &std::env::args().collect::<Vec<String>>()[1..];

    if args.is_empty() {
        println!(r"Usage:
  brainfuck                   - Show usage
  brainfuck [code file [...]] - Run the passed code file(s)");
        return Ok(());
    }

    let regex = Regex::new(r"[^><+\-.,\[\]]").expect("Could not compile Regular Expression");

    for arg in args {
        let path = Path::new(arg);
        if !path.exists() {
            return Err(format!("Path does not exist: {}", arg));
        }
        if !path.is_file() {
            return Err(format!("Path is not a file: {}", arg));
        }
        match std::fs::File::open(arg) {
            Ok(mut file) => {
                let mut buffer = String::new();
                match file.read_to_string(&mut buffer) {
                    Ok(_) => {
                        BfInterpreter::new().run(&regex.replace_all(&buffer, ""))?
                    },
                    Err(error) => return Err(format!("{}", error))
                }
            },
            Err(error) => return Err(format!("{}", error))
        }
    }
    Ok(())
}
