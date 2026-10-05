//#![feature(never_type)]
// Problem:
// Given a vector of integers `nums`, return all unique triplets [a, b, c]
// such that a + b + c == 0. The solution must not contain duplicate triplets.
// Order of triplets and order inside a triplet does not matter (tests normalize).
//
// Constraints (interview-style):
// - 0 <= nums.len() <= 3000
// - -10^5 <= nums[i] <= 10^5
// Aim for O(n^2) time after sorting.

mod first_missing_positive;
mod longest_unique_substring;
mod never;
mod string_min_distance;
mod string_min_window;
mod water_trapper;

fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let len = nums.len();
    let mut triplets = vec![];
    for i1 in 0..len {
        for i2 in (i1 + 1)..len {
            for i3 in (i2 + 1)..len {
                let mut triplet = vec![nums[i1], nums[i2], nums[i3]];
                triplet.sort_unstable();
                if triplet.iter().sum::<i32>() == 0 {
                    if triplets.iter().find(|t| triplet.eq(*t)).is_none() {
                        triplets.push(triplet);
                    }
                }
            }
        }
    }
    triplets
}

#[cfg(test)]
mod tests {
    use super::*;

    fn normalize(mut res: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        for t in &mut res {
            t.sort_unstable();
        }
        res.sort_unstable();
        res
    }

    #[test]
    fn example_basic() {
        let res = three_sum(vec![-1, 0, 1, 2, -1, -4]);
        let expected = vec![vec![-1, -1, 2], vec![-1, 0, 1]];
        assert_eq!(normalize(res), normalize(expected));
    }

    #[test]
    fn empty_and_small() {
        assert_eq!(three_sum(vec![]), vec![] as Vec<Vec<i32>>);
        assert_eq!(three_sum(vec![0]), vec![] as Vec<Vec<i32>>);
        assert_eq!(three_sum(vec![0, 0]), vec![] as Vec<Vec<i32>>);
        assert_eq!(three_sum(vec![1, 2]), vec![] as Vec<Vec<i32>>);
    }

    #[test]
    fn all_zeros() {
        let res = three_sum(vec![0, 0, 0, 0]);
        assert_eq!(normalize(res), vec![vec![0, 0, 0]]);
    }

    #[test]
    fn no_solution() {
        assert_eq!(three_sum(vec![1, 2, 3, 4]), vec![] as Vec<Vec<i32>>);
        assert_eq!(three_sum(vec![-5, -4, -3, -2]), vec![] as Vec<Vec<i32>>);
    }

    #[test]
    fn with_duplicates() {
        let res = three_sum(vec![-2, 0, 0, 2, 2]);
        let expected = vec![vec![-2, 0, 2]];
        assert_eq!(normalize(res), normalize(expected));
    }

    #[test]
    fn many_duplicates() {
        let res = three_sum(vec![-1, -1, -1, 0, 0, 0, 1, 1, 1, 2]);
        let expected = vec![vec![-1, -1, 2], vec![-1, 0, 1], vec![0, 0, 0]];
        assert_eq!(normalize(res), normalize(expected));
    }

    #[test]
    fn already_sorted() {
        let res = three_sum(vec![-4, -1, -1, 0, 1, 2]);
        let expected = vec![vec![-1, -1, 2], vec![-1, 0, 1]];
        assert_eq!(normalize(res), normalize(expected));
    }

    #[test]
    fn reverse_sorted() {
        let res = three_sum(vec![4, 1, 0, -1, -1, -4]);
        let expected = vec![vec![-1, -1, 2], vec![-1, 0, 1]];
        // Note: 2 is not present, wait — adjust expected properly
        // Actual expected from this input: [-1,0,1] and [-4,0,4]? No 2.
        // Correct expected:
        let expected = vec![vec![-1, 0, 1], vec![-4, 0, 4]];
        assert_eq!(normalize(res), normalize(expected));
    }

    #[test]
    fn large_negatives_and_positives() {
        let res = three_sum(vec![-10, -5, -2, 0, 3, 7, 12]);
        let expected = vec![vec![-10, -2, 12], vec![-5, -2, 7], vec![-5, 0, 5]];
        // Correct:
        let expected = vec![vec![-10, -2, 12], vec![-10, 3, 7], vec![-5, -2, 7]];
        // Precise:
        // -10 + -2 + 12 = 0
        // -5 + -2 + 7 = 0
        // -5 + 0? +5 no
        // -2 + 0 + 2 no
        assert_eq!(normalize(res), normalize(expected));
    }

    #[test]
    fn single_valid_triplet_with_noise() {
        let res = three_sum(vec![5, -2, 3, 1, -1, 0, 8, -3]);
        let expected = vec![
            vec![-3, -2, 5],
            vec![-3, 0, 3],
            vec![-2, -1, 3],
            vec![-1, 0, 1],
        ];
        assert_eq!(normalize(res), normalize(expected));
    }

    #[test]
    fn extreme_values() {
        let res = three_sum(vec![-100000, 50000, 50000, 0, 1, -1]);
        let expected = vec![vec![-100000, 50000, 50000], vec![-1, 0, 1]];
        assert_eq!(normalize(res), normalize(expected));
    }
}
