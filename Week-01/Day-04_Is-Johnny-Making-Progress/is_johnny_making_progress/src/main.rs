fn progress_days(days: &[i32]) -> i32 {
    let mut days_of_progress = 0;
    for i in 0..days.len() {
        if i > 0 && days[i] > days[i-1] {
            days_of_progress += 1;
        }
    }
    return days_of_progress;
}


fn main() {
    assert_eq!(2, progress_days(&[3, 4, 1, 2]));
    assert_eq!(3, progress_days(&[10, 11, 12, 9, 10]));
    assert_eq!(1, progress_days(&[6, 5, 4, 3, 2, 9]));
    assert_eq!(0, progress_days(&[9, 9]));
    println!("Tests pass.");
}
