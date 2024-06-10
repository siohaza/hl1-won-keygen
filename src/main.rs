use rand::Rng;
use rand::distributions::Uniform;
use std::io;

fn generate_random_digit() -> char {
    let mut rng = rand::thread_rng();
    let digit = rng.sample(Uniform::new_inclusive(0, 9));
    std::char::from_digit(digit, 10).unwrap()
}

fn generate_key() -> String {
    let mut key = String::new();

    for _ in 0..4 {
        key.push(generate_random_digit());
    }
    key.push('-');
    for _ in 0..5 {
        key.push(generate_random_digit());
    }
    key.push('-');
    for _ in 0..4 {
        key.push(generate_random_digit());
    }

    key
}

fn validate_key(key: &str) -> bool {
    let cdkey: String = key.chars().filter(|&c| c != '-').collect();

    if cdkey.len() != 13 {
        return false;
    }

    let mut sum = 3u32;
    for (i, ch) in cdkey.chars().enumerate() {
        let digit = ch.to_digit(10).unwrap();
        if i < 12 {
            let transformed = (sum << 1) ^ digit;
            sum += transformed;
        } else {
            let last_digit = digit;
            sum = (sum % 10).wrapping_sub(last_digit);
            return sum == 0;
        }
    }

    false
}

fn main() {
    println!("How many keys do you want to generate?");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let num_keys: usize = input.trim().parse().expect("Please enter a valid number");

    for _ in 0..num_keys {
        loop {
            let key = generate_key();
            if validate_key(&key) {
                println!("Generated valid key: {}", key);
                break;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_key() {
        // Test valid key
        assert!(validate_key("8188-54856-0626"));

        // Test invalid key
        assert!(!validate_key("1234-12345-1235"));
    }
}
