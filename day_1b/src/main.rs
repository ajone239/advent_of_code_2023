use std::io;

fn main() -> io::Result<()> {
    let stdin = io::stdin();

    let mut total = 0;

    for line in stdin.lines() {
        let line = line.unwrap();
        let Some((f, l)) = get_first_and_last(&line) else {
            continue;
        };

        // F is the tens place digit
        // L is the ones place digit
        let val = 10 * f + l;

        total += val;
    }

    println!("Total: {}", total);

    Ok(())
}

fn get_first_and_last(s: &str) -> Option<(u32, u32)> {
    let digits = find_digit_in_chars(s);

    let first = *digits.first().unwrap();
    let last = *digits.last().unwrap();

    Some((first, last))
}

fn find_digit_in_chars(s: &str) -> Vec<u32> {
    let mut rv = vec![];

    for i in 0..s.len() {
        let c = s.chars().nth(i).unwrap();

        if c.is_ascii_digit() {
            let digit = c as u32 - '0' as u32;
            rv.push(digit);
        } else if let Some(digit) = check_for_digit_strings(&s[i..]) {
            rv.push(digit);
        }
    }
    rv
}

fn check_for_digit_strings(s: &str) -> Option<u32> {
    let digit_strs = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];
    for ds in digit_strs {
        if check_for_str_in_slice(s, ds) {
            let digit = string_to_digit(ds);
            return Some(digit);
        }
    }
    None
}

fn check_for_str_in_slice(main_str: &str, pattern: &str) -> bool {
    let pattern_len = pattern.len();

    if main_str.len() < pattern_len {
        return false;
    }

    &main_str[..pattern_len] == pattern
}

fn string_to_digit(s: &str) -> u32 {
    match s {
        "zero" => 0,
        "one" => 1,
        "two" => 2,
        "three" => 3,
        "four" => 4,
        "five" => 5,
        "six" => 6,
        "seven" => 7,
        "eight" => 8,
        "nine" => 9,
        &_ => panic!(),
    }
}
