use std::io::prelude::*;
use std::fs::File;
use std::env;
use std::process;

fn read_file(filename : &str) -> Vec<String>{
    let mut file = match File::open(filename) {
        Ok(file) => file,
        Err(_) => panic!("no such file"),
    };
    let mut file_contents = String::new();
    file.read_to_string(&mut file_contents)
        .ok()
        .expect("failed to read!");
    let lines: Vec<String> = file_contents.split("\n")
        .map(|s: &str| s.to_string())
        .collect();
    lines
}

fn count_anagrams(words : Vec<String>) ->u32{
    let word_length = words.len();
    let mut sorted_words : Vec<String> = Vec::with_capacity(word_length);
    
    for word in words {
        let mut a: Vec<&str> = word.split("").collect();
        a.sort();
        sorted_words.push(a.join(""));
    }

    let mut number_of_anagrams = 0;
    
    sorted_words.sort();
    for i in 0..word_length {
        if i == 0 {
            if sorted_words[i] == sorted_words[i + 1] {
                number_of_anagrams+= 1;
            }
        } else if i == word_length - 1 {
            if sorted_words[i] == sorted_words[i - 1] {
                number_of_anagrams+= 1;
            }
        } else {
            if sorted_words[i] == sorted_words[i-1] || sorted_words[i] == sorted_words[i + 1]{
                number_of_anagrams+= 1;
            }
        }
    }

    return number_of_anagrams;
}

fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() < 2 {
        println!("usage: {} file", args[0]);
        process::exit(1);
    }
    
    let ref filename = args[1];
    let words = read_file(filename);
    let number_of_anagrams = count_anagrams(words);

    println!("Number of anagrams: {}", number_of_anagrams);
}
