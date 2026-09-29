//  Rust Bytes Challenges Issue #137
//
// 给定字符串word1和word2, 返回把word1变为word2的最少操作步数。
// 你可以任意执行以下操作：
// 插入一个字符，删除一个字符， 或者替换一个字符。
// Given two strings word1 and word2,
// return the minimum number of operations required to convert word1 to word2.
//
// You may perform the following operations any number of times:
// Insert a character, Delete a character or Replace a character

pub fn min_distance(word1: String, word2: String) -> i32 {
    // TODO: implement
    if word1 == word2 {
        return 0;
    }

    if word1.starts_with(word2.as_str()) {
        return word1.len() as i32 - word2.len() as i32;
    }

    if word2.starts_with(word1.as_str()) {
        return word2.len() as i32 - word1.len() as i32;
    }

    let word1_chars = word1.chars().collect::<Vec<char>>();
    let word2_chars = word2.chars().collect::<Vec<char>>();
    let word1_len = word1_chars.len();
    let word2_len = word2_chars.len();
    if word1_len == 0  {
        return word2_len as i32;
    }else if word2_len == 0 {
        return word1_len as i32;
    } else {
        let len_min = word1_len.min(word2_len);
        for i in 0..len_min {
            if word1_chars[i] != word2_chars[i] {
                let word1_insert = {
                    let mut word1_chars = word1_chars.clone();
                    word1_chars.insert(i, word2_chars[i]);
                    word1_chars
                };
                let word1_replace = {
                    let mut word1_chars = word1_chars.clone();
                    word1_chars[i] = word2_chars[i];
                    word1_chars
                };
                let word1_remove = {
                    let mut word1_chars = word1_chars.clone();
                    word1_chars.remove(i);
                    word1_chars
                };
                let word1_insert_suffix = word1_insert[i .. ].iter().collect::<String>();
                let word1_replace_suffix = word1_replace[i .. ].iter().collect::<String>();
                let word1_remove_suffix = word1_remove[i .. ].iter().collect::<String>();
                let word2_suffix = word2_chars[i .. ].iter().collect::<String>();
                return 1 +
                     min_distance(word1_insert_suffix, word2_suffix.clone())
                    .min(min_distance(word1_replace_suffix, word2_suffix.clone()))
                    .min(min_distance(word1_remove_suffix, word2_suffix.clone()));
            }
        }


    }


    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(min_distance("horse".to_string(), "ros".to_string()), 3);
    }

    #[test]
    fn example_2() {
        assert_eq!(
            min_distance("intention".to_string(), "execution".to_string()),
            5
        );
    }

    #[test]
    fn both_empty() {
        assert_eq!(min_distance("".to_string(), "".to_string()), 0);
    }

    #[test]
    fn first_empty() {
        assert_eq!(min_distance("".to_string(), "abc".to_string()), 3);
    }

    #[test]
    fn second_empty() {
        assert_eq!(min_distance("abc".to_string(), "".to_string()), 3);
    }

    #[test]
    fn identical() {
        assert_eq!(min_distance("abc".to_string(), "abc".to_string()), 0);
        assert_eq!(min_distance("a".to_string(), "a".to_string()), 0);
    }

    #[test]
    fn single_char_diff() {
        assert_eq!(min_distance("a".to_string(), "b".to_string()), 1);
    }

    #[test]
    fn insert_only() {
        assert_eq!(min_distance("abc".to_string(), "abxc".to_string()), 1);
    }

    #[test]
    fn delete_only() {
        assert_eq!(min_distance("abxc".to_string(), "abc".to_string()), 1);
    }

    #[test]
    fn replace_only() {
        assert_eq!(min_distance("abc".to_string(), "adc".to_string()), 1);
    }

    #[test]
    fn completely_different() {
        assert_eq!(min_distance("abc".to_string(), "def".to_string()), 3);
    }

    #[test]
    fn one_char_vs_long() {
        assert_eq!(min_distance("a".to_string(), "abcdef".to_string()), 5);
        assert_eq!(min_distance("abcdef".to_string(), "a".to_string()), 5);
    }

    #[test]
    fn prefix() {
        assert_eq!(min_distance("abc".to_string(), "abcdef".to_string()), 3);
    }

    #[test]
    fn suffix() {
        assert_eq!(min_distance("def".to_string(), "abcdef".to_string()), 3);
    }

    #[test]
    fn classic_extra() {
        assert_eq!(min_distance("kitten".to_string(), "sitting".to_string()), 3);
    }

    #[test]
    fn longer_diff() {
        assert_eq!(
            min_distance("algorithm".to_string(), "altruistic".to_string()),
            6
        );
    }

    #[test]
    fn repeated_chars() {
        assert_eq!(min_distance("aaa".to_string(), "aa".to_string()), 1);
        assert_eq!(min_distance("aa".to_string(), "aaa".to_string()), 1);
        assert_eq!(min_distance("aaa".to_string(), "aaa".to_string()), 0);
    }
}
