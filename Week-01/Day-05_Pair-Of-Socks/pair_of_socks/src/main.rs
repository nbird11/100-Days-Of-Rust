use std::collections::hash_map;

fn sock_pairs(socks: &str) -> i32 {
    let mut hash_map: hash_map::HashMap<char, i32> = hash_map::HashMap::new();
    for sock in socks.chars() {
        *hash_map.entry(sock).or_insert(0) += 1;
    }
    let mut pairs = 0;
    for count in hash_map.values() {
        pairs += count / 2;
    }
    pairs
}

fn main() {
    assert_eq!(1, sock_pairs("AA"));
    assert_eq!(2, sock_pairs("ABABC"));
    assert_eq!(4, sock_pairs("CABBACCC"));
    println!("Tests pass.")
}
