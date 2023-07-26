const DROPS: [char; 7] = ['a', 'e', 'h', 'i', 'o', 'u', 'w'];

/// The Soundex algorithm
pub fn soundex(input: &str) -> String {
    // 1. drop vowels and h, w
    let res1 = input
        .to_lowercase()
        .chars()
        .enumerate()
        .filter(|(i, c)| *i == 0 || !DROPS.contains(c))
        .enumerate()
        .map(|(i, (_, c))| {
            let lowered = c.to_ascii_lowercase();
            if i == 0 {
                lowered.to_ascii_uppercase()
            } else {
                consonants_digit_map(lowered)
            }
        })
        .collect::<String>();

    // 2. map consonants to digits
    let res2 = res1
        .chars()
        .enumerate()
        .map(|(i, c)| {
            let lowered = c.to_ascii_lowercase();
            if i == 0 {
                lowered.to_ascii_uppercase()
            } else {
                consonants_digit_map(lowered)
            }
        })
        .collect::<String>();

    // 3. fold serial digits
    let res3 = fold_serial_digits(res2);

    // 4. round or pad
    round_or_pad(res3)
}

fn consonants_digit_map(c: char) -> char {
    match c {
        'b' | 'f' | 'p' | 'v' => '1',
        'c' | 'g' | 'j' | 'k' | 'q' | 's' | 'x' | 'z' => '2',
        'd' | 't' => '3',
        'l' => '4',
        'm' | 'n' => '5',
        'r' => '6',
        _ => '0',
    }
}

fn fold_serial_digits(code: String) -> String {
    code.chars()
        .fold(String::new(), |mut acc, c| match acc.chars().last() {
            Some(last) if last == c => acc,
            _ => {
                acc.push(c);
                acc
            }
        })
}

fn round_or_pad(code: String) -> String {
    let mut padded = code;
    padded.truncate(4);
    padded.extend(std::iter::repeat('0').take(4 - padded.len()));
    padded
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_soundex() {
        assert_eq!(soundex("Robert"), "R163");
        assert_eq!(soundex("Rupert"), "R163");
        assert_eq!(soundex("Rubin"), "R150");
        assert_eq!(soundex("Ashcraft"), "A261");
        assert_eq!(soundex("Ashcroft"), "A261");
        assert_eq!(soundex("Tymczak"), "T052");
    }
}
