mod commands {
    pub mod hello {
        pub fn run(args: &Vec<String>) -> Result<(), String> {
            if args.is_empty() {
                return Err("hello requires a name".into());
            }
            let mut name = args[0].clone();
            let yell = args.iter().any(|s| s == "--yell");
            if yell {
                name = name.to_uppercase();
            }
            println!("Hello, {}!", name);
            Ok(())
        }
    }

    pub mod sum {
        pub fn run(args: &Vec<String>) -> Result<(), String> {
            if args.is_empty() {
                return Err("sum requires at least one integer".into());
            }
            let mut total: i64 = 0;
            for s in args {
                let n: i64 = s.parse().map_err(|_| format!("Invalid integer: {}", s))?;
                total += n;
            }
            println!("{}", total);
            Ok(())
        }
    }

    pub mod guess {
        pub fn run(_args: &Vec<String>) -> Result<(), String> {
            // Minimal stub for the guessing game
            println!("Guess game not implemented in this stub.");
            Ok(())
        }
    }

    pub mod read {
        use std::fs;
        pub fn run(args: &Vec<String>) -> Result<(), String> {
            let path = args.get(0).ok_or("read requires a path".to_string())?;
            let content = fs::read_to_string(path)
                .map_err(|e| format!("Failed to read {}: {}", path, e))?;
            println!("{} bytes", content.len());
            Ok(())
        }
    }
}

use std::env;

fn print_usage() {
    eprintln!(
        "bug-free-funicular — a tiny Rust CLI for learning

Usage:
  bug-free-funicular <command> [args]

Commands:
  hello <name> [--yell]   Greet someone (optionally in ALL CAPS)
  sum <n1> <n2> ...       Sum a list of integers
  guess                   Play a number guessing game (1..=100)
  read <path>             Read a file and show a short summary

Examples:
  bug-free-funicular hello Alice
  bug-free-funicular hello Bob --yell
  bug-free-funicular sum 4 10 -3
  bug-free-funicular guess
  bug-free-funicular read README.md"
    );
}

fn main() {
    let mut args = env::args().skip(1).collect::<Vec<_>>();

    if args.is_empty() {
        print_usage();
        return;
    }

    let cmd = args.remove(0);
    let result = match cmd.as_str() {
        "hello" => commands::hello::run(&args),
        "sum" => commands::sum::run(&args),
        "guess" => commands::guess::run(&args),
        "read" => commands::read::run(&args),
        _ => {
            eprintln!("Unknown command: {}\n", cmd);
            print_usage();
            return;
        }
    };

    if let Err(e) = result {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}
