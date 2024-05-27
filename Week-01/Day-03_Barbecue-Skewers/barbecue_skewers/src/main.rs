fn skewer_types(skewers: Vec<String>) -> (i32, i32) {
    let mut veg = 0;
    let mut non_veg = 0;
    for skewer in skewers {
        if skewer.contains("x") {
            non_veg += 1;
        } else {
            veg += 1;
        }
    }
    (veg, non_veg)
}

fn main() {
    let mut result = skewer_types(vec![
        String::from("--xo--x--ox--"),
        String::from("--xx--x--xx--"),
        String::from("--oo--o--oo--"),
        String::from("--xx--x--ox--"),
        String::from("--xx--x--ox--"),
    ]);
    println!("Veg: {}, Non-Veg: {}", result.0, result.1);
    result = skewer_types(vec![
        String::from("--oooo-ooo--"),
        String::from("--xx--x--xx--"),
        String::from("--o---o--oo--"),
        String::from("--xx--x--ox--"),
        String::from("--xx--x--ox--"),
    ]);
    println!("Veg: {}, Non-Veg: {}", result.0, result.1);
    result = skewer_types(vec![
        String::from("--oooo-ooo--"),
        String::from("--xxxxxxxx--"),
        String::from("--o---"),
        String::from("-o-----o---x--"),
        String::from("--o---o-----"),
    ]);
    println!("Veg: {}, Non-Veg: {}", result.0, result.1);
}
