use std::{
    env, fs,
    io::{self, BufRead, Read, Write},
    sync::{Arc, Mutex},
};

use bibim::{env::Env, run};

fn main() {
    if let Some(file_path) = env::args().nth(1) {
        let input = Arc::new(Mutex::new(io::stdin()));
        let output = Arc::new(Mutex::new(io::stdout()));
        let mut env = Env {
            cursor: None,
            mem: vec![],
            is_debug: false,
            on_read_io: Box::new(|| {
                let mut buffer = Vec::new();
                input.lock().unwrap().read_to_end(&mut buffer).unwrap();
                buffer
            }),
            on_write_io: Box::new(|data| {
                output.lock().unwrap().write(data.as_slice()).unwrap();
                output.lock().unwrap().flush().ok();
            }),
        };
        let code = fs::read_to_string(file_path).unwrap();
        match run(code, &mut env) {
            Ok(_) => {}
            Err(e) => println!("Error: {}", e),
        }
    } else {
        loop {
            print!(">>> ");
            io::stdout().flush().ok();
            let input = Arc::new(Mutex::new(io::stdin()));
            let output = Arc::new(Mutex::new(io::stdout()));
            let code = input.lock().unwrap().lock().lines().next();
            let mut env = Env {
                cursor: None,
                mem: vec![],
                is_debug: false,
                on_read_io: Box::new(|| {
                    let mut buffer = Vec::new();
                    input.lock().unwrap().read_to_end(&mut buffer).unwrap();
                    buffer
                }),
                on_write_io: Box::new(|data| {
                    output.lock().unwrap().write(data.as_slice()).unwrap();
                    output.lock().unwrap().flush().ok();
                }),
            };
            match code {
                Some(Ok(ref l)) => match run(l.to_string(), &mut env) {
                    Ok(_) => {}
                    Err(e) => println!("Error: {}", e),
                },
                _ => break,
            }
        }
    }
}
