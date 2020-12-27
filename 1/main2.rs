use std::fs;
use std::collections::HashMap;

fn main() {

    let input_file = "input.txt";
    let target = 2020;

    // --snip--
    println!("In file {}", input_file);

    let contents = fs::read_to_string(input_file)
        .expect("Something went wrong reading the file");

    let mut i = 0;
    for line in contents.split("\n") {

        let mut combinations = HashMap::new();

        let line_int = line.parse::<i32>().unwrap();
        let new_target = target - line_int;
        let mut j = 0;
        for line2 in contents.split("\n") {
            if j <= i {
                j = j + 1;
                continue;
            }
            let line2_int = line2.parse::<i32>().unwrap();
            combinations.insert(line2_int, line2_int);
            let remaining = new_target - line2_int;
            if combinations.contains_key(&remaining) {
                println!("We've got a match!");
                println!("{} + {} + {}", line2_int, remaining, line_int);
                println!("{}", line2_int * remaining * line_int);
            }
        }

        i = i + 1;
    }
}