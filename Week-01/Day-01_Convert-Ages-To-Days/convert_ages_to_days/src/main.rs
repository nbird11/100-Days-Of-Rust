fn calc_days(age: u32) -> u32 {
    return age * 365;
}

fn main() {
    let mut running = true;

    while running {
        println!("Please input your age in years!");

        let mut input_age = String::new();

        std::io::stdin()
            .read_line(&mut input_age)
            .expect("Input value should be a string");

        let years: i32 = match input_age.trim().parse() {
            Ok(parsed_age) => {
                if parsed_age < 0 {
                    running = false;
                    continue;
                }
                parsed_age
            },
            Err(_) => {
                println!("Invalid age! Please input a positive whole number!");
                continue;
            }
        };

        let days: u32 = calc_days(years as u32);

        println!("You are roughly {days} days old!");
    }
}

