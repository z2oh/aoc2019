use std::io::{self, prelude::*};

fn main() {
    let sum: i32 = io::stdin()
        .lock()
        .lines()
        .map(|line| {
            let parsed = line
                .expect("Bufread::lines failed to return line.")
                .parse()
                .expect("Parsing to int failed.");

            // This is for part 1.
            // get_mass(parsed)

            // This is for part 2.
            get_mass_with_fuel(parsed)
        })
        .sum();

    println!("{}", sum);
}

#[inline(always)]
fn get_mass(module: i32) -> i32 {
    (next / 3) - 2
}

fn get_mass_with_fuel(module: i32) -> i32 {
    let mut sum = 0;
    let mut next = module;
    loop {
        next = (next / 3) - 2;
        if next < 0 {
            break sum
        } else {
            sum += next
        }
    }
}
