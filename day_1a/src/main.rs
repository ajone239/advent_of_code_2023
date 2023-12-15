use std::io;

fn main() -> io::Result<()> {
    let stdin = io::stdin();

    let mut total = 0;

    for line in stdin.lines() {
        let line = line.unwrap();
        let Some((f, l)) = get_first_and_last2(&line) else {
            panic!();
        };

        // F is the tens place digit
        // L is the ones place digit
        let val = 10 * f + l;

        total += val;
    }

    println!("Total: {}", total);

    Ok(())
}

fn get_first_and_last2(s: &str) -> Option<(u32, u32)> {
    let mut digits = s
        .chars()
        .filter(|c| c.is_ascii_digit())
        .map(|c| c as u32 - '0' as u32);

    let Some(first) = digits.next() else {
        return None;
    };

    let last = digits.last().unwrap_or(first);

    Some((first, last))
}

fn get_first_and_last(s: &str) -> Option<(u32, u32)> {
    let iter = s.chars();
    let riter = s.chars().rev();

    let first = find_digit_in_chars(iter);
    let last = find_digit_in_chars(riter);

    match (first, last) {
        (Some(f), Some(l)) => Some((f, l)),
        _ => None,
    }
}

fn find_digit_in_chars<I>(i: I) -> Option<u32>
where
    I: Iterator<Item = char>,
{
    for c in i {
        if c.is_ascii_digit() {
            return Some(c as u32 - '0' as u32);
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("1abc2", Some(1))]
    #[case("pqr3stu8vwx", Some(3))]
    #[case("a1b2c3d4e5f", Some(1))]
    #[case("treb7uchet", Some(7))]
    #[case("trebuchet", None)]
    fn test_find_digit_in_chars(#[case] test_string: &str, #[case] expected_val: Option<u32>) {
        let iter = test_string.chars();

        assert_eq!(find_digit_in_chars(iter), expected_val);
    }

    #[rstest]
    #[case("1abc2", Some((1, 2)))]
    #[case("pqr3stu8vwx", Some((3, 8)))]
    #[case("a1b2c3d4e5f", Some((1, 5)))]
    #[case("treb7uchet", Some((7, 7)))]
    #[case("trebuchet", None)]
    fn test_get_first_and_last(
        #[case] test_string: &str,
        #[case] expected_val: Option<(u32, u32)>,
    ) {
        assert_eq!(get_first_and_last(test_string), expected_val);
    }
}
