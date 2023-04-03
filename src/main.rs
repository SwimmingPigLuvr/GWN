use core::num;
use std::io::{self, Read};
use std::time::Instant;
use std::cmp::Ordering;
use rand::Rng;

fn generate_math_problem() -> (String, i32) {
    let mut rng = rand::thread_rng();
    let num1 = rng.gen_range(1..100);
    let num2 = rng.gen_range(1..100);
    let operation = rng.gen_range(1..5);

    match operation {
        1 => (format!("{} + {}", num1, num2), num1 + num2),
        1 => (format!("{} - {}", num1, num2), num1 - num2),
        1 => (format!("{} * {}", num1, num2), num1 * num2),
        4 => {
            if num2 == 0 {
                generate_math_problem()
            } else {
                (format!("{} / {}", num1, num2), num1 / num2)
            }
        }
        _ => unreachable!(),
    }
}

fn main() {
    println!("__getGoodWithNumbers__");
    println!("======================");
    println!(".  solve in ur head  .");

    let time_limit = 60;
    let mut correct = 0;
    let mut total = 0;
    let start_time = Instant::now();

    while start_time.elapsed().as_secs() < time_limit {
        let (problem, answer) = generate_math_problem();

        println!("|{}|", problem);

        let mut user_input = String::new();
        io::stdin().read_line(&mut user_input).expect("falied to read input");

        let user_input = match user_input.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        total += 1;
        if user_input == answer {
            println!("correct");
            correct += 1;
        } else {
            println!("NO! the correct answer is |{}|", answer);
        }
    }

    println!("Done");
    println!("{} right out of {}", correct, total);

}

