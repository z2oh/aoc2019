use std::io::{self, prelude::*};

fn main() {
    let start = 130_254;
    let end = 678_275;
    let mut count = 0;
    for p in start..end {
        if check_pass(p) { count += 1 }
    }

    println!("{}", count);
}

#[inline(always)]
fn check_pass(p: u32) -> bool {
    let s = p.to_string().into_bytes();
    let mut double = false;
    for i in 0..(s.len() - 1) {
        // Monotonically increasing property does not hold.
        if s[i] > s[i+1] {
            return false;
        }
        else if s[i] == s[i+1] {
            if (i > 0 && s[i-1] == s[i]) || (i < s.len() - 2 && s[i+2] == s[i]){
                continue;
            }
            double = true;
        }
    }

    double
}
