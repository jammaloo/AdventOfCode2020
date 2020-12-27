extern crate regex;
use std::fs;
use regex::Regex;


fn main() {
    let input_file = "input.txt";

    // --snip--
    println!("In file {}", input_file);

    let contents = fs::read_to_string(input_file)
        .expect("Something went wrong reading the file");
    let parse_re = Regex::new(r"(\d+)-(\d+) ([a-z]): ([a-z]+)").unwrap();

    let mut valid_passwords = 0;
    for cap in parse_re.captures_iter(contents.as_ref()) {
        // 1-7 j: vrfjljjwbsv
        let pos1 = &cap[1].parse::<i32>().unwrap() - 1;
        let pos2 = &cap[2].parse::<i32>().unwrap() - 1;
        let character = &cap[3];
        let password = &cap[4];
        let char1 = password.chars().nth(pos1 as usize).unwrap().to_string();
        let char2 = password.chars().nth(pos2 as usize).unwrap().to_string();
        if (char1 == character || char2 == character) && char1 != char2 {
            valid_passwords = valid_passwords + 1;
        }
    }
    println!("Valid passwords: {}", valid_passwords);
}