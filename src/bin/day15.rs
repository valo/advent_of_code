use std::{char, io::{self, BufRead}};

fn hash_string(s: &str) -> u64 {
    let mut result: u64 = 0;
    for char in s.bytes() {
        result += char as u64;
        result *= 17;
        result %= 256;
    }

    result
}

fn main() {
    let stdin = io::stdin();
    let mut result = 0;
    for line in stdin.lock().lines() {
        for seq in line.unwrap().split(",").map(|x| x.trim()) {
            result += hash_string(seq);
        }
    }

    println!("{}", result);
}

// Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash_string() {
        assert_eq!(hash_string("HASH"), 52);
    }
}