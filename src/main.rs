use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("ERROR: not provided filepath to map");
        std::process::exit(1);
    }

    let path = Path::new(&args[1]);
    let display = path.display();

    let mut file = match File::open(&path) {
        Err(why) => {
            eprintln!("ERROR: couldn't open {}: {}", display, why);
            std::process::exit(1);
        },
        Ok(file) => file,
    };

    let mut map_s = String::new();
    match file.read_to_string(&mut map_s) {
        Err(why) => {
            eprintln!("ERROR: couldn't read {}: {}", display, why);
            std::process::exit(1);
        },
        Ok(_) => {}
    }

}
