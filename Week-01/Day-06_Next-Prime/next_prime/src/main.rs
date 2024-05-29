fn is_prime(num: i32) -> bool {
    match num {
        0 | 1 => false,
        2 => true,
        _ => {
            let mut primes: Vec<i32> = (3..=num).collect();
            for i in 2..=(num as f64).sqrt().ceil() as i32 {
                for j in ((i*2)..=num).step_by(i as usize) {
                    primes.retain(|&x| x != j);
                }
            }
            primes[primes.len() - 1] == num
        }
    }
}

fn next_prime(num: i32) -> i32 {
    if is_prime(num) {
        return num;
    }

    let mut next_num = num + 1;
    while !is_prime(next_num) {
        next_num += 1;
    }
    next_num
}

fn main() {
    assert_eq!(13, next_prime(12));
    assert_eq!(29, next_prime(24));
    assert_eq!(11, next_prime(11));
    println!("Test pass");
}
