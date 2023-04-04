use core::num;
use std::io;
use std::time::{Instant, Duration};
use rand::Rng;

// todo
// solutions per/minute
// accuracy in percentage

fn generate_math_problem() -> (String, i32) {
    let mut rng = rand::thread_rng();
    let mut num1 = rng.gen_range(1..100);
    let mut num2 = rng.gen_range(1..100);
    let mult1 = rng.gen_range(1..=20);
    let mult2 = rng.gen_range(1..=20);
    let operation = rng.gen_range(1..=3);

    match operation {
        1 => {
            if num1 < num2 {
                std::mem::swap(&mut num1, &mut num2);
            }
            (format!("{} + {}", num1, num2), num1 + num2)
        },
        2 => {
            if num1 < num2 {
                std::mem::swap(&mut num1, &mut num2);
            }
            (format!("{} - {}", num1, num2), num1 - num2)
        }
        3 => (format!("{} * {}", mult1, mult2), mult1 * mult2),
        _ => unreachable!(),
    }
}

fn main() {
    let mut correct = 0;
    let mut total = 0;

    println!("choose test length: 60s 30s 10s");
    let mut duration_input = String::new();
    io::stdin().read_line(&mut duration_input).expect("failed to read input");

    let game_duration_secs: u64 = match duration_input.trim().parse(){
        Ok(num) => num,
        Err(_) => {
            println!("please enter a positive number");
            return;
        }
    };
    let game_duration = Duration::from_secs(game_duration_secs);

    let start_time = Instant::now();

    println!("You have {} seconds to solve as many math problems as you can!", game_duration_secs);

    while start_time.elapsed() < game_duration {
        let (problem, answer) = generate_math_problem();
        println!("{}", problem);

        let mut user_input = String::new();
        io::stdin().read_line(&mut user_input).expect("Failed to read input");

        let user_input: i32 = match user_input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input. Please enter a number.");
                continue;
            }
        };

        if user_input == answer {
            correct += 1;
            println!("Correct!");
        } else {
            println!("Incorrect. The correct answer was {}.", answer);
        }

        total += 1;

        let time_left = game_duration.checked_sub(start_time.elapsed()).unwrap_or_else(|| Duration::new(0, 0));
        println!("Time left: {} seconds", time_left.as_secs());


    }

    println!(
        "Time's up! You answered {}/{} questions correctly.",
        correct, total
    );
    let solutions_per_minute = (correct as f64 * 60.0) / game_duration.as_secs() as f64;
    println!("spm: {:.2}", solutions_per_minute);
}


