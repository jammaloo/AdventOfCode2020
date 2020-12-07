use std::fs;
use std::collections::HashMap;

fn main() {
    let input_file = "input.txt";
    let target = 2020;
    let mut combinations = HashMap::new();

    // --snip--
    println!("In file {}", input_file);

    let contents = fs::read_to_string(input_file)
        .expect("Something went wrong reading the file");

    for line in contents.split("\n") {
        let line_int = line.parse::<i32>().unwrap();
        combinations.insert(line_int, line_int);
        let remaining = target - line_int;
        if combinations.contains_key(&remaining) {
            println!("We've got a match!");
            println!("{}", line_int * remaining);
        }
    }
}