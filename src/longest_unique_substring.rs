//  Rust Bytes Challenges Issue #134
//
// Task:
//   Given a string slice s, return a substring (as &str) that is the
//   longest contiguous substring containing only unique characters
//   (no repeated characters).
//
//   - If there are multiple substrings of the same maximum length, return
//     the leftmost one.
//   - The returned reference must borrow from the original s (zero-copy).
//   - Work correctly with Unicode (operate on char`s, not bytes).
//   - Handle all edge cases correctly (empty string, single char, all
//     identical characters, mixed ASCII + non-ASCII, etc.).
//
// Constraints / expectations for a strong solution:
//   - O(n) time
//   - O(min(n, alphabet size)) extra space
//   - No unnecessary allocations of String
//   - Correct lifetime (the returned &str lives as long as `s`)

use std::collections::HashMap;

fn longest_unique_substring(s: &str) -> &str {
    let chars: Vec<(usize, char)> = s.char_indices().collect();
    let chars_count = chars.len();

    let mut longest_start_idx = 0;
    let mut max_count = 0;
    for i in 0..chars_count {
        let mut seen_chars = HashMap::new();
        seen_chars.insert(&chars[i].1, true);
        for j in (i + 1)..chars_count {
            if seen_chars.contains_key(&chars[j].1) {
                break;
            }else {
                seen_chars.insert(&chars[j].1, true);
            }
        }
        if seen_chars.len() > max_count {
            longest_start_idx = chars[i].0;
            max_count = seen_chars.len();
        }
    }

    let longest_end_char_idx = longest_start_idx + max_count;
    let longest_end_idx = if longest_end_char_idx >= chars_count {
        s.len()
    }else {
        chars[longest_end_char_idx].0
    };

    &s[longest_start_idx..longest_end_idx]
}

#[cfg(test)]
mod tests {
    use crate::longest_unique_substring::longest_unique_substring;
    #[test]
    fn test_longest_unique_substring() -> () {
        //Basic cases
        assert_eq!(longest_unique_substring(""), "");
        assert_eq!(longest_unique_substring("a"), "a");
        assert_eq!(longest_unique_substring("au"), "au");
        assert_eq!(longest_unique_substring("aab"), "ab");
        assert_eq!(longest_unique_substring("abcabcbb"), "abc");
        assert_eq!(longest_unique_substring("bbbbb"), "b");
        assert_eq!(longest_unique_substring("pwwkew"), "wke");
        assert_eq!(longest_unique_substring("dvdf"), "vdf");
        assert_eq!(longest_unique_substring(" "), " ");
        assert_eq!(longest_unique_substring("  "), " ");

        //Longer / trickier cases
        assert_eq!(longest_unique_substring("abcde"), "abcde");
        assert_eq!(longest_unique_substring("abba"), "ab");
        assert_eq!(longest_unique_substring("tmmzuxt"), "mzuxt");
        assert_eq!(longest_unique_substring("ohvhjdml"), "vhjdml");
        assert_eq!(longest_unique_substring("anviaj"), "nviaj");

        // Unicode / multi-byte characters
        assert_eq!(longest_unique_substring("你好世界你好"), "你好世界");
        assert_eq!(longest_unique_substring("a😊b😊c"), "a😊b");
        assert_eq!(longest_unique_substring("日本語"), "日本語");
        assert_eq!(longest_unique_substring("aaあaa"), "aあ");

        // Edge: all unique vs. early repeats
        assert_eq!(longest_unique_substring("abcdefg"), "abcdefg");
        assert_eq!(longest_unique_substring("aabcdefg"), "abcdefg");
        assert_eq!(longest_unique_substring("abcdeaf"), "bcdeaf");

        println!("All tests passed!");
    }
}
