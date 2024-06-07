fn trap(height: Vec<i32>) -> i32 {
    let (mut left, mut right, mut max_left, mut max_right, mut ans) =
        (0, height.len() - 1, 0, 0, 0);
    while left < right {
        if height[left] < height[right] {
            if height[left] > max_left {
                max_left = height[left];
            } else {
                ans += max_left - height[left];
            }
            left += 1;
        } else {
            if height[right] > max_right {
                max_right = height[right];
            } else {
                ans += max_right - height[right];
            }
            right -= 1;
        }
    }
    ans
}

fn main() {
    assert_eq!(trap(vec![0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1]), 6);
    assert_eq!(trap(vec![4, 2, 0, 3, 2, 5]), 9);
    println!("Tests pass.")
}
