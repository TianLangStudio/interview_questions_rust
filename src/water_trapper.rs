//  Rust Bytes Challenges Issue #135
//
// Implement trap; given a Vec representing bar heights of width 1,
// return the total units of water that can be trapped after raining.

pub fn trap(heights: Vec<i32>) -> i32 {
    // TODO: implement
    let len = heights.len();
    if len <= 2 {
        return 0;
    };
    let mut pre_maxes = vec![];
    let mut pre_max = 0;
    let mut post_maxes = vec![];
    let mut post_max = 0;
    for  h in heights.iter() {
        pre_maxes.push(pre_max);
        pre_max = pre_max.max(*h);
    }
    for  h in heights.iter().rev() {
        post_maxes.insert(0, post_max);
        post_max = post_max.max(*h);
    }
    let mut total = 0;
    for i in 0..len {
        let pre_max = pre_maxes[i];
        let post_max = post_maxes[i];
        let h = heights[i];
        if h < pre_max  && h < post_max {
            total += post_max.min(pre_max) - h;
        }
    }
    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        assert_eq!(trap(vec![]), 0);
    }

    #[test]
    fn single_bar() {
        assert_eq!(trap(vec![5]), 0);
    }

    #[test]
    fn two_bars() {
        assert_eq!(trap(vec![3, 1]), 0);
        assert_eq!(trap(vec![1, 3]), 0);
    }

    #[test]
    fn strictly_increasing() {
        assert_eq!(trap(vec![1, 2, 3, 4, 5]), 0);
    }

    #[test]
    fn strictly_decreasing() {
        assert_eq!(trap(vec![5, 4, 3, 2, 1]), 0);
    }

    #[test]
    fn classic_valley() {
        //   ■
        // ■ ■ ■
        // ■ ■ ■ ■
        // 0 1 0 2 → traps 1
        assert_eq!(trap(vec![0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1]), 6);
    }

    #[test]
    fn flat_bottom() {
        assert_eq!(trap(vec![4, 2, 0, 3, 2, 5]), 9);
    }

    #[test]
    fn multiple_peaks() {
        assert_eq!(trap(vec![5, 4, 1, 2]), 1);
        assert_eq!(trap(vec![2, 0, 2]), 2);
        assert_eq!(trap(vec![4, 2, 3]), 1);
    }

    #[test]
    fn all_zeros() {
        assert_eq!(trap(vec![0, 0, 0, 0]), 0);
    }

    #[test]
    fn high_walls_low_middle() {
        assert_eq!(trap(vec![9, 1, 1, 1, 9]), 24);
    }

    #[test]
    fn asymmetric() {
        assert_eq!(
            trap(vec![6, 4, 2, 0, 3, 2, 0, 3, 1, 4, 5, 3, 2, 7, 5, 3, 0, 1]),
            44
        );
    }

    #[test]
    fn single_deep_well() {
        assert_eq!(trap(vec![5, 0, 0, 0, 5]), 15);
    }

    #[test]
    fn decreasing_then_spike() {
        assert_eq!(trap(vec![5, 4, 3, 2, 1, 0, 6]), 15);
    }
}
