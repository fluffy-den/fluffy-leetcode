use std::collections::HashMap;

pub fn length_of_longest_substring(s: String) -> i32 {
    let mut occurences: HashMap<char, usize> = HashMap::new();
    let mut slide_pos: usize = 0;
    let mut max_len: usize = 0;

    for (i, c) in s.chars().enumerate() {
        if let Some(&char_index) = occurences.get(&c)
            && char_index >= slide_pos
        {
            slide_pos = char_index + 1;
        }

        occurences.insert(c, i);
        max_len = max_len.max(i - slide_pos + 1);
    }

    max_len as i32
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;

    #[test]
    fn unit1() {
        let str = String::from_str("abcabcbb");
        assert_eq!(length_of_longest_substring(str.unwrap()), 3);
    }

    #[test]
    fn unit2() {
        let str = String::from_str("bbbbb");
        assert_eq!(length_of_longest_substring(str.unwrap()), 1);
    }

    #[test]
    fn unit3() {
        let str = String::from_str("pwwkew");
        assert_eq!(length_of_longest_substring(str.unwrap()), 3);
    }
}
