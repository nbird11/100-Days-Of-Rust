fn merge_sorted(into: &mut [i32], m: usize, from: &[i32], n: usize) {
    let mut i = m;
    let mut j = n;
    let mut k = m + n;

    while i > 0 && j > 0 {
        k -= 1;
        if into[i - 1] > from[j - 1] {
            into[k] = into[i - 1];
            i -= 1;
        } else {
            into[k] = from[j - 1];
            j -= 1;
        }
    }

    while j > 0 {
        k -= 1;
        into[k] = from[j - 1];
        j -= 1;
    }
}

fn main() {
    let mut nums1 = [1,2,3,0,0,0];
    let nums2 = [2,5,6];
    merge_sorted(&mut nums1, 3, &nums2, 3);
    println!("{:?}", nums1);
}
