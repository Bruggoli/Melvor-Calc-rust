use std::io;
use rand::Rng;
use std::cmp::Ordering;
use num_format::{Locale, ToFormattedString};


fn main() {
    println!("Starting resources: ");
    let starting_resources: i32 = input();
    println!("Chance to double resources: ");
    let double_chance: i32 = input();
    println!("Chance to save: ");
    let save_chance: i32 = input();
    println!("XP per action: ");
    let xp_action: i32 = input();
    println!("Crafting time per action: ");
    let crafting_time: f64 = input_f();
    
    println!("Starting resources: {}\nDouble chance: {}\nSave chance: {}\nXP per action: {}\nCrafting time: {}\n\n", 
        starting_resources, double_chance, save_chance, xp_action, crafting_time);

    let mut total_resources_vec: Vec<i32> = vec![];
    let mut total_xp_vec: Vec<i32> = vec![];
    let mut total_c_c_vec: Vec<f64> = vec![];

    for _i in 0..=10{
        let single_calc = calculate_resources(starting_resources, double_chance, save_chance, xp_action, crafting_time);
        total_resources_vec.push(single_calc.0);
        total_xp_vec.push(single_calc.1);
        total_c_c_vec.push(single_calc.2);
    }
    
    let time_formatted = time_fmt(find_average_f(total_c_c_vec) as i32);


    println!("Total resources made: {}", readable(find_average(total_resources_vec)));
    println!("Total XP gained: {}", readable(find_average(total_xp_vec)));
    println!("Total crafting time: {}:{}:{}", time_formatted.0, time_formatted.1, time_formatted.2);
}

fn calculate_resources(sr: i32, dc: i32, sc: i32, xp: i32, ct:f64) -> (i32, i32, f64) {
    let mut raw_resources = sr;
    let mut crafting_cycles = 0;
    let mut total_xp: i32 = 0;
    let mut processed_resources = 0;
        
    while raw_resources > 0 {
        let double_roll = rand::thread_rng().gen_range(1..=100);
        let save_roll = rand::thread_rng().gen_range(1..=100);
        
        match double_roll.cmp(&dc) {
            Ordering::Less => processed_resources += 2,
            Ordering::Equal => processed_resources += 2,
            Ordering::Greater => processed_resources += 1,
        };
        match save_roll.cmp(&sc) {
            Ordering::Less => raw_resources += 1,
            Ordering::Equal => raw_resources += 1,
            Ordering::Greater => (),
        };

        raw_resources -= 1;
        crafting_cycles += 1;
        total_xp += xp;
    }
    
    return (processed_resources, total_xp, crafting_cycles as f64 * ct);

}

fn readable(s: i32) -> String {
    s.to_formatted_string(&Locale::en)
}

fn find_average(n: Vec<i32>) -> i32{
    let sum: i32 = n.iter().sum();
    sum / (n.len()) as i32
}

fn find_average_f(n: Vec<f64>) -> f64{
    let sum: f64 = n.iter().sum();
    sum / (n.len()) as f64
}

fn input() -> i32 {
    let mut s = String::new();
    io::stdin().read_line(&mut s)
            .expect("Failed to read line.");

    let s: i32 = match s.trim().parse::<i32>() {
        Ok(num) => num,
        Err(_) => panic!("Input not a valid number."),
    };
    s
}

fn input_f() -> f64 {
    let mut s = String::new();
    io::stdin().read_line(&mut s)
            .expect("Failed to read line.");

    let s: f64 = match s.trim().parse::<f64>() {
        Ok(num) => num,
        Err(_) => panic!("'Crafting time' input not a valid number."),
    };
    s
}

fn time_fmt(n: i32) -> (i32, i32, i32) {
    let initial_seconds: i32 = n;
    let seconds: i32 = initial_seconds % 60;
    let minutes: i32 = (initial_seconds / 60) % 60;
    let hours: i32 = initial_seconds / 3600;
    (hours, minutes, seconds)

}