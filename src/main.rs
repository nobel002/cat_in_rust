use std::env;
use std::fs;

fn main() {
    //println!("Hello, world!");
    let args: Vec<String> = env::args().collect();
    let path = match args.get(1){
        Some(n) => {
          n
        }
        None => {
            ""
        }
    };
    let mut contents = String::new();
    if path != "" {
        contents  = match fs::read_to_string(path){
            Ok(n) => {
                n
            }
            Err(err) => {
                println!("{err}");
                String::new()
            }
        }
    }
    println!("{contents}");
}
