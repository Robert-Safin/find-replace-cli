use std::io::Write;
use std::{
    fs::{self, File},
    io,
};
fn main() {
    println!("Specify path to file");

    let mut input: String = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let input = input.trim();

    let contents: String = fs::read_to_string(input).expect("file not found or can not be read");

    println!("File found");
    println!("What word to replace?");
    let mut old_word: String = String::new();
    io::stdin()
        .read_line(&mut old_word)
        .expect("Failed to read line");

    let old_word = old_word.trim();

    println!("Whats the new word?");
    let mut new_word: String = String::new();
    io::stdin()
        .read_line(&mut new_word)
        .expect("Failed to read line");

    let new_word: &str = new_word.trim();

    println!("Scanning...");

    let output = find_and_replace(contents, old_word, new_word);

    let mut file: File = File::create("output.txt").expect("Could not create new file");
    file.write_all(output.as_bytes())
        .expect("Could not write to file");
    print!("Output generated");
}

fn find_and_replace(contents: String, old_word: &str, new_word: &str) -> String {
    let words: std::str::SplitWhitespace<'_> = contents.split_whitespace();

    let mut vec: Vec<String> = Vec::new();
    let mut count = 0;

    for word in words {
        if word == old_word {
            count += 1;
            vec.push(new_word.to_string())
        } else if word == old_word.to_uppercase() {
            count += 1;
            vec.push(new_word.to_uppercase())
        } else {
            vec.push(word.to_string())
        }
    }

    println!("Found {} matches", count);
    vec.join(" ")
}
