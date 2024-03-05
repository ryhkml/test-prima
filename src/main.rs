use std::{env, ops::Add};

fn main() {

    let args: Vec<String> = env::args().collect();

    let start = &args[1].parse::<u32>().unwrap();
    let start = match start {
        0 | 1   => 2,
        n       => *n
    };

    let limit = &args[2].parse::<u32>().unwrap();

    let mut results = String::from("");

    for i in start..=*limit {

        let mut is_prime = true;
        let mut first = 2;

        while first < i {

            if i % first == 0 {
                is_prime = false;
                break;
            }

            first += 1;
        }

        if is_prime {
            results.push_str(&first.to_string().add(", "));
        }
    }

    results.pop();
    results.pop();

    if results.is_empty() {
        println!();
        println!("Kosong");
        println!();
    } else {
        println!();
        println!("Bilangan prima dari {start} sampai {limit} adalah");
        println!("{results}");
        println!();
    }
}