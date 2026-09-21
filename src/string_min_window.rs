//  Rust Bytes Challenges Issue #136

use std::collections::HashMap;

pub fn min_window(s: String, t: String) -> String {
    // TODO: implement solution here
    let s_chars = s.chars().collect::<Vec<char>>();
    let t_chars = t.chars().collect::<Vec<char>>();
    let s_len = s_chars.len();
    let t_len = t_chars.len();
    if s_len < t_len || s_len == 0 || t_len == 0 {
        return String::new();
    }

    let t_chars_count_map = t_chars
        .iter()
        .fold(HashMap::with_capacity(t_len), |mut acc, c| {
            *acc.entry(c).or_insert(0) += 1;
            acc
        });
    for win_size in t_len ..= s_len {
        for window in s_chars.windows(win_size) {
            let s_window_char_count_map =
                window
                    .iter()
                    .fold(HashMap::with_capacity(win_size), |mut acc, c| {
                        *acc.entry(c).or_insert(0) += 1;
                        acc
                    });
            if t_chars_count_map
                .iter()
                .all(|(c, count)| s_window_char_count_map.get(c).unwrap_or(&0) >= count)
            {
                return window.iter().collect();
            }
        }
    }

    String::new()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_s() {
        assert_eq!(min_window("".to_string(), "A".to_string()), "");
    }

    #[test]
    fn empty_t() {
        assert_eq!(min_window("ABC".to_string(), "".to_string()), "");
    }

    #[test]
    fn both_empty() {
        assert_eq!(min_window("".to_string(), "".to_string()), "");
    }

    #[test]
    fn no_window() {
        assert_eq!(min_window("ABCDEF".to_string(), "XYZ".to_string()), "");
        assert_eq!(min_window("A".to_string(), "AA".to_string()), "");
    }

    #[test]
    fn classic() {
        assert_eq!(
            min_window("ADOBECODEBANC".to_string(), "ABC".to_string()),
            "BANC"
        );
    }

    #[test]
    fn single_char() {
        assert_eq!(min_window("a".to_string(), "a".to_string()), "a");
        assert_eq!(min_window("a".to_string(), "b".to_string()), "");
    }

    #[test]
    fn exact_match() {
        assert_eq!(min_window("ABC".to_string(), "ABC".to_string()), "ABC");
        assert_eq!(
            min_window("ABCDEF".to_string(), "ABCDEF".to_string()),
            "ABCDEF"
        );
    }

    #[test]
    fn duplicates_in_t() {
        assert_eq!(
            min_window("ADOBECODEBANC".to_string(), "AABC".to_string()),
            "ADOBECODEBA"
        );
        assert_eq!(min_window("aa".to_string(), "aa".to_string()), "aa");
        assert_eq!(
            min_window("aaaaaaaaaaaabbbbbcdd".to_string(), "abcdd".to_string()),
            "abbbbbcdd"
        );
    }

    #[test]
    fn multiple_possible() {
        // should pick the shortest one
        assert_eq!(min_window("aabdec".to_string(), "abc".to_string()), "abdec");
        assert_eq!(
            min_window("aaflslflsldkalskaaa".to_string(), "aaa".to_string()),
            "aaa"
        );
    }

    #[test]
    fn window_at_start() {
        assert_eq!(min_window("ABCXYZ".to_string(), "ABC".to_string()), "ABC");
    }

    #[test]
    fn window_at_end() {
        assert_eq!(min_window("XYZABC".to_string(), "ABC".to_string()), "ABC");
    }

    #[test]
    fn overlapping_candidates() {
        assert_eq!(
            min_window("cabwefgewcwaefgcf".to_string(), "cae".to_string()),
            "cwae"
        );
    }

    #[test]
    fn long_with_repeats() {
        assert_eq!(min_window("bbaac".to_string(), "aba".to_string()), "baa");
    }

    #[test]
    fn case_sensitive() {
        // problem is case-sensitive
        assert_eq!(min_window("aBc".to_string(), "abc".to_string()), "");
        assert_eq!(min_window("aBc".to_string(), "aBc".to_string()), "aBc");
    }
}
