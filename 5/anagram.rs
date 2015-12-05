use std::io::prelude::*;
use std::fs::File;
use std::env;
use std::process;
use std::io;
use std::path::Path;

fn read_file<P>(filename : P) -> Result<io::Lines<io::BufReader<File>>, io::Error> where P: AsRef<Path>{
    let file = try!(File::open(filename));
    Ok(io::BufReader::new(file).lines())
}

fn print(words : Vec<&str>) {
    for word in words {
        println!("{}", word);
    }
}

fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() < 2 {
        println!("usage: {} file", args[0]);
        process::exit(1);
    }
    
    let ref filename = args[1];
    let words = read_file(filename);
    
    if words.is_err() {
        println!("Could not read file: {}", filename);
        process::exit(1);
    }
    
    words.lines();


    match words {
        Ok(lines) => print(words.iter().cloned().collect()),
        Err(err) => println!("Bad girl"),
    }

}
