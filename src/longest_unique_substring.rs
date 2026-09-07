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

fn longest_unique_substring(s: &str) -> &str {
    let chars:Vec<(usize, char)> = s.char_indices().collect();
    let len = chars.len();
    if len < 2 {
        return s;
    }
    let mut longest_start_idx = 0;
    let mut longest_end_idx = 0;
    let mut longest_len = 0;

    // TODO: implementation here
    for (i, (byte_idx, c)) in chars.iter().enumerate() {
        let mut sub_string_chars:Vec<&char> = vec![];
        sub_string_chars.push(&c);

        for j in (i + 1) .. len {
            let next_char = &chars[j];
            println!("next_char {:?}", next_char);
            if sub_string_chars.contains(&&next_char.1) {
                break;
            }else {
                sub_string_chars.push(&next_char.1);
            }
        }
        if sub_string_chars.len() > longest_len {
            longest_start_idx = chars[i].0;
            longest_end_idx = if i + sub_string_chars.len() >= len {
                s.len()
            } else {
                chars[i + sub_string_chars.len()].0
            };
            longest_len = sub_string_chars.len();
        }
    }
    &s[longest_start_idx.. longest_end_idx]
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
