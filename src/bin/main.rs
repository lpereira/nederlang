extern crate nederlang;

use std::fs;
use std::io::{self, Write};
use std::mem::size_of;
use std::path::Path;

use nederlang::eval::eval_program;
use nederlang::object::NlObject;
use nederlang::vm::run_str;
use std::env::args;

fn run_repl(use_vm: bool) {
    let mut buffer = String::with_capacity(512);
    loop {
        buffer.clear();
        print!(">>> ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut buffer).unwrap();

        if use_vm {
            match run_str(&buffer) {
                Ok(obj) => println!("{}", obj),
                Err(e) => eprintln!("{:?}", e),
            }
        } else {
            match eval_program(&buffer, None) {
                Ok(obj) => println!("{}", obj),
                Err(e) => eprintln!("{:?}", e),
            }
        }
    }
}

fn run_from_file(f: &Path, use_vm: bool) {
    let program = fs::read_to_string(f).unwrap();

    if use_vm {
        match run_str(&program) {
            Ok(obj) => println!("{}", obj),
            Err(e) => eprintln!("{:?}", e),
        }
    } else {
        match eval_program(&program, None) {
            Ok(obj) => println!("{}", obj),
            Err(e) => eprintln!("{:?}", e),
        }
    }
}

fn main() -> io::Result<()> {
    assert!(
        size_of::<NlObject>() <= 16,
        "NlObject is larger than 16 bytes. Don't cut corners Danny boy."
    );

    let file = args().skip(1).find(|a| !a.starts_with("--"));
    let use_vm = args().any(|a| a == "--vm");

    if let Some(file) = file {
        let path = Path::new(&file);
        run_from_file(path, use_vm);
    } else {
        run_repl(use_vm);
    }
    Ok(())
}
