//  Rust Bytes Challenges Issue #139: First Missing Positive
//
// Given an unsorted integer array nums, return the smallest positive integer
// that is *not* present in nums.
//
// You must implement an algorithm that runs in O(n) time and uses constant
// extra space.

pub fn first_missing_positive(nums: Vec<i32>) -> i32 {
    // TODO: implement
    if nums.is_empty() {
        return 1;
    }
    let mut positives = nums.iter().filter(|&n| *n > 0).collect::<Vec<&i32>>();
    positives.sort();
    if positives.is_empty()  || positives[0] > &1{
        return 1;
    }
    let len = positives.len();
    for i in 1..len {
        let prev = positives[i-1];
        let cur = positives[i];
        if cur - prev > 1 {
           return prev + 1;
        }
    }
    **positives.last().unwrap() + 1
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() -> () {
        assert_eq!(first_missing_positive(vec![1, 2, 0]), 3);
        assert_eq!(first_missing_positive(vec![3, 4, -1, 1]), 2);
        assert_eq!(first_missing_positive(vec![7, 8, 9, 11, 12]), 1);

        // Single element
        assert_eq!(first_missing_positive(vec![1]), 2);
        assert_eq!(first_missing_positive(vec![2]), 1);
        assert_eq!(first_missing_positive(vec![-1]), 1);
        assert_eq!(first_missing_positive(vec![0]), 1);

        // All negatives / zeros
        assert_eq!(first_missing_positive(vec![-5, -3, -1, 0]), 1);
        assert_eq!(first_missing_positive(vec![0, 0, 0]), 1);

        // Already contains 1..n
        assert_eq!(first_missing_positive(vec![1, 2, 3, 4]), 5);
        assert_eq!(first_missing_positive(vec![4, 3, 2, 1]), 5);

        // Missing in the middle
        assert_eq!(first_missing_positive(vec![1, 2, 4, 5]), 3);
        assert_eq!(first_missing_positive(vec![2, 1, 4, 3, 6]), 5);

        // Duplicates
        assert_eq!(first_missing_positive(vec![1, 1, 1, 1]), 2);
        assert_eq!(first_missing_positive(vec![1, 2, 2, 3, 3]), 4);
        assert_eq!(first_missing_positive(vec![3, 3, 3, 1]), 2);

        // Large gaps / out-of-range positives
        assert_eq!(first_missing_positive(vec![1000, 2000, 3000]), 1);
        assert_eq!(first_missing_positive(vec![1, 100000, 2]), 3);

        // Mixed signs + duplicates + zero
        assert_eq!(first_missing_positive(vec![-10, -3, 0, 1, 2, 2, 4, 7]), 3);
        assert_eq!(first_missing_positive(vec![1, -1, 3, 4, -5, 2, 0, 1]), 5);

        // Edge: n and n+1 present, missing smaller
        assert_eq!(first_missing_positive(vec![2, 3, 4, 5, 6]), 1);
        assert_eq!(first_missing_positive(vec![1, 3, 4, 5, 6]), 2);

        println!("All tests passed!");
    }
}
