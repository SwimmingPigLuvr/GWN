use owo_colors::OwoColorize;
use rand::{thread_rng, Rng};
use std::io::{self, Write};
use std::time::Instant;
use prettytable::{Table, Row, Cell};

//todo
// separate sections with comment titles

fn main() {
    let base: u32 = 19;
    let limit: u32 = 20;
    print_table(base, limit);
    println!("Select game mode:");
    println!("1. Random");
    println!("2. Practice");
    println!("3. Print Table");
    // take input
    let mut mode_input = String::new();
    io::stdin()
        .read_line(&mut mode_input)
        .expect("Failed to read input");
    // set game mode
    let game_mode: u32 = match mode_input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid input. Please enter 1 or 2.");
            return;
        }
    };
    // set operator
    let (operation, user_number, ascending_order) = if game_mode == 2 {
        println!("Select operation:");
        println!("1. Addition");
        println!("2. Subtraction");
        println!("3. Multiplication");
        println!("4. Division");

        let mut operation_input = String::new();
        io::stdin()
            .read_line(&mut operation_input)
            .expect("Failed to read input");

        let operation: u32 = match operation_input.trim().parse() {
            Ok(num) => {
                if num >= 1 && num <= 4 {
                    num
                } else {
                    println!("Invalid input. Please enter a number between 1 and 4.");
                    return;
                }
            }
            Err(_) => {
                println!("Invalid input. Please enter a number between 1 and 4.");
                return;
            }
        };
        // set number
        println!("Enter the number you want to work on (0 for random):");
        let mut number_input = String::new();
        io::stdin()
            .read_line(&mut number_input)
            .expect("Failed to read input");

        let user_number: u32 = match number_input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input. Please enter a non-negative number.");
                return;
            }
        };
        // practice in order?
        println!("Do you want to go through problems in ascending order? (yes/no)");
        let mut order_input = String::new();
        io::stdin()
            .read_line(&mut order_input)
            .expect("Failed to read input");

        let ascending_order = match order_input.trim().to_lowercase().as_str() {
            "yes" | "y" => true,
            "no" | "n" => false,
            _ => {
                println!("Invalid input. Please enter 'yes' or 'no'.");
                return;
            }
        };

        (operation, user_number, ascending_order)
    } else {
        (0, 0, false)
    };

    println!("{}", ("set duration: 60s 30s 10s").bright_cyan());
    let mut duration_input = String::new();
    io::stdin().read_line(&mut duration_input).expect("incorrect fmt");
    let user_duration: u32 = match duration_input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("{}", ("please enter a number").red());
            return;
        }
    };

    println!("Get ready to start!");
    let start_time = Instant::now();
    let game_duration = std::time::Duration::from_secs(user_duration as u64);
    let mut correct_answers = 0;
    let mut total_questions = 0;
    let mut current_problem_count = 0;

    while start_time.elapsed() < game_duration {
        let (problem, answer) = generate_math_problem(
            operation,
            user_number,
            ascending_order,
            current_problem_count,
        );

        if ascending_order {
            current_problem_count += 1;
        }

        println!("{}", problem);
        print!("Your answer: ");
        io::stdout().flush().unwrap();

        let mut user_input = String::new();
        io::stdin()
            .read_line(&mut user_input)
            .expect("Failed to read input");

        let user_input: i32 = match user_input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input. Skipping question.");
                total_questions += 1;
                continue;
            }
        };

        if user_input == answer {
            println!("Correct!");
            correct_answers += 1;
        } else {
            println!("Incorrect. The correct answer was {}.", answer);
        }
        total_questions += 1;
    }

    let accuracy = correct_answers as f64 / total_questions as f64 * 100.0;
    let solutions_per_minute = correct_answers as f64 / (game_duration.as_secs() as f64 / 60.0);
    println!(
        "Game over! You answered {} out of {} questions correctly ({:.2}%).",
        correct_answers, total_questions, accuracy
    );
    println!("Solutions per minute: {:.2}", solutions_per_minute);
}

fn print_table(base: u32, limit: u32) {
    let mut table = Table::new();
    // Add a header row
    table.add_row(Row::new(vec![
        Cell::new("Multiplier"),
        Cell::new("Product"),
    ]));

    // Add data rows
    for i in 1..=limit {
        table.add_row(Row::new(vec![
            Cell::new(&format!("{} x {}", base, i)),
            Cell::new(&(base * i).to_string()),
        ]));
    }

    // Print the table
    table.printstd();
}



fn generate_math_problem(
    operation: u32,
    user_number: u32,
    ascending_order: bool,
    current_problem_count: u32,
) -> (String, i32) {
    let mut rng = thread_rng();

    let op = if operation == 0 {
        rng.gen_range(1..=3)
    } else {
        operation
    };

    let (num1, num2) = if user_number == 0 {
        let num1 = rng.gen_range(1..=20);
        let num2 = rng.gen_range(1..=20);

        (num1, num2)
    } else if ascending_order {
        (user_number, current_problem_count + 1)
    } else {
        (user_number, rng.gen_range(1..=20))
    };

    match op {
        1 => (format!("{} + {}", num1, num2), (num1 + num2) as i32),
        2 => {
            if num1 >= num2 {
                (format!("{} - {}", num1, num2), (num1 - num2) as i32)
            } else {
                (format!("{} - {}", num2, num1), (num2 - num1) as i32)
            }
        }
        3 => (format!("{} * {}", num1, num2), (num1 * num2) as i32),
        _ => unreachable!(),
    }
}