use std::env;
use std::fs;
use std::process;

/// Solves https://www.smartgames.eu/uk/one-player-games/jumpin[JumpIN'] puzzles.

fn main() {
    let args: Vec<String> = env::args().collect();
    let file: &str = match &args[..] {
        [_, file] => file,
        _ => {
            println!("Invalid arguments");
            process::exit(1);
        }
    };
    let src: String = match fs::read_to_string(file) {
        Ok(src) => src,
        Err(err) => {
            eprintln!("Failed to read {} [{}]", file, err);
            process::exit(1);
        }
    };
    println!("{}", src);
    let jumpin: jumpin::JumpIN = match src.parse() {
        Ok(jumpin) => jumpin,
        Err(_) => {
            println!("Failed to parse JumpIN");
            process::exit(1);
        }
    };
    match puzzle::solve(jumpin) {
        None => println!("no solution"),
        Some((mvs, _)) => {
            println!("solution:");
            let mut last_obj = None;
            for (obj, dir) in &mvs {
                if last_obj != Some(obj) {
                    last_obj = Some(obj);
                    print!("{}", obj)
                }
                print!("{}", dir);
            }
            println!()
        }
    }
}
