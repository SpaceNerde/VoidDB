use std::io::{self, BufRead, Write};

fn main() {
    // Basic REPL for now

    // Intro message
    println!();
    println!("------------------------------------------");
    println!("       Void DB                            ");
    println!("       0.0.1                              ");
    println!("------------------------------------------");
    println!();

    // Command Loop
    loop {
        // setup console and get inputs
        print!("db >> ");
        io::stdout().flush().unwrap();
        let mut buffer = String::new();
        let stdin = io::stdin();
        let mut handle = stdin.lock();
        handle.read_line(&mut buffer).unwrap();

        // handle inputs
        if buffer.contains("exit") {
            break;
        } else {
            println!("  Unkown Command!");
        }
    }
}
