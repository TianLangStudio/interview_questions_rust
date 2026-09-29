//  Rust Bytes Challenges Issue #136
//今天在网上看到一个Rust练习题：
//Given two strings s and t, return the minimum window substring of s such that
// every character in t including duplicates is included in the window.
//给定两个字符串s和t, 要求从s中返回一个子字符串，这个子字符串必须包含所有t字符中的字符（包括重复的字符）
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
    let t_char_count_map = char_count(&t_chars);
    for size in t_len ..= s_len {
        for window in s_chars.windows(size) {
            let s_char_count_map = char_count(window);
            if t_char_count_map.iter().all(|(t_char, t_count)| {
                s_char_count_map.get(t_char).unwrap_or(&0) >= &t_count
            }) {
                return window.iter().collect();
            }
        }
    }
    String::new()
}

fn char_count(t_chars: &[char]) -> HashMap<&char, i32> {
    t_chars.iter().fold(HashMap::new(), |mut m, c| {
        *m.entry(c).or_insert(0) += 1;
        m
    })
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
