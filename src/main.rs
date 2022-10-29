use std::{
    env, fs,
    io::{self, BufRead, Write},
};

use bibim::run;

fn main() {
    let stdin = io::stdin();
    if let Some(file_path) = env::args().nth(1) {
        let code = fs::read_to_string(file_path).unwrap();
        match run(code) {
            Ok(_) => {}
            Err(e) => println!("Error: {}", e),
        }
    } else {
        loop {
            print!(">>> ");
            io::stdout().flush().ok();
            match stdin.lock().lines().next() {
                Some(Ok(ref l)) => match run(l.to_string()) {
                    Ok(_) => {}
                    Err(e) => println!("Error: {}", e),
                },
                _ => break,
            }
        }
    }
}
