use std::fs;

fn main() {
    let input_file = "input.txt";

    // --snip--
    println!("In file {}", input_file);

    let mut x = 0;

    let contents = fs::read_to_string(input_file)
        .expect("Something went wrong reading the file");

    let mut tree_count = 0;
    for line in contents.lines() {
        let line_length = line.chars().count();
        let mut chars = line.chars();
        if chars.nth((x % line_length) as usize).unwrap() == '#' {
            tree_count = tree_count + 1;
        }
        x = x + 3;
    }

    println!("Trees: {}", tree_count);
}