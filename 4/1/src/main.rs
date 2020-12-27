use std::fs;

struct PassportFields {
    byr: bool,
    iyr: bool,
    eyr: bool,
    hgt: bool,
    hcl: bool,
    ecl: bool,
    pid: bool,
    // cid: bool, Not needed, optional field
}

fn main() {
    let input_file = "input.txt";

    // --snip--
    println!("In file {}", input_file);

    let contents = fs::read_to_string(input_file)
        .expect("Something went wrong reading the file");

    let mut valid_passports = 0;
    // Double newline separates passports
    for passport_line in contents.split("\n\n") {
        // Passports can span multiple lines, so merge them into a single string
        let passport = str::replace(passport_line, "\n", " ");
        let mut passport_data = PassportFields { byr: false, iyr: false, eyr: false, hgt: false, hcl: false, ecl: false, pid: false, };
        // Passport fields are separated by a space
        let passport_fields = passport.split(" ");
        for passport_field in passport_fields {
            // Passport key values are separated by a colon
            let mut key_value = passport_field.split(":");

            let key = key_value.next().unwrap();

            match key {
                "byr" => passport_data.byr = true,
                "iyr" => passport_data.iyr = true,
                "eyr" => passport_data.eyr = true,
                "hgt" => passport_data.hgt = true,
                "hcl" => passport_data.hcl = true,
                "ecl" => passport_data.ecl = true,
                "pid" => passport_data.pid = true,
                _ => (),
            }
        }
        if passport_data.byr && passport_data.iyr && passport_data.eyr && passport_data.hgt && passport_data.hcl && passport_data.ecl && passport_data.pid {
            valid_passports = valid_passports + 1;
        }
    }

    println!("Valid Passports: {}", valid_passports);
}