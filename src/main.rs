/*
Solves the train game (specified below) by brute force.

Take the four digits of the car number you are on and try to make 10.

Implementation-specific rules:
- 0 to the power of 0 is undefined.
- Operands for division must be exactly divisible.
- d/dx is not permitted.
- base conversion is not permitted.

Note: this program won't find all possible combinations to make 10 with the 
given digits and operators.

Written by Stephen.
Last updated 3 August 2024.
*/


mod math;

use itertools::Itertools;
use itertools::repeat_n;




fn main() -> Result<(), String> {
    // Take 4-digit number from commandline arg.
    let input_arg = std::env::args().nth(1).unwrap_or_default();
    if input_arg == "help" {
        print_help_msg();
        return Ok(())
    } else if input_arg.len() != 4 || input_arg.parse::<u128>().is_err() {
        return Err(String::from(
            "First argument must be a 4-digit number. Use the argument 'help' for more info."
        ));
    }

    let mut in_order: bool = false;
    let mut target: i128 = 10;

    // Process the options.
    let all_args = std::env::args().collect::<Vec<String>>();
    let mut position = 2;
    while position < all_args.len() {
        match all_args[position].as_str() {
            "help" => {
                print_help_msg();
                return Ok(());
            }
            "-i" => {
                in_order = true;
            },
            "-t" => {
                let target_str = all_args.get(position + 1);
                if target_str.is_none() {
                    return Err(String::from("No target provided with `-t`."));
                }
                let new_target = target_str.unwrap().parse::<i128>();
                if new_target.is_err() {
                    return Err(String::from("Target must be an integer."));
                }
                target = new_target.unwrap();
                position += 1;
            },
            _ => {
                return Err(String::from(
                    "Invalid options provided. Use the argument 'help' for more info."
                ));
            }
        }
        position += 1;
    }

    // Operators.
    let operators = vec![
        "+",
        "-",
        "*",
        "/",
        "mod",
        "<<",
        ">>",
        "bitwiseOR",
        "bitwiseAND",
        "bitwiseXOR",
        "^",
    ];

    // Factorial cache.
    let mut factorial_cache: Vec<i128> = vec![1];
    for i in 1usize..=33 {
        factorial_cache.push((i as i128) * factorial_cache[i - 1]);
    }

    // Get an iterator over every arrangement of digits and every arrangement of 
    // operators.
    let digits_operators: Vec<((Vec<i128>, Vec<&str>), Vec<bool>)> = if in_order { 
        vec![input_arg.chars().map(|x| x.to_digit(10).unwrap() as i128).collect_vec()]
        .into_iter()
        .cartesian_product(repeat_n(operators.into_iter(), 3).multi_cartesian_product().into_iter())
        .cartesian_product(repeat_n([false, true].into_iter(), 6).multi_cartesian_product().into_iter())
        .collect()
    } else {
        input_arg
        .chars()
        .map(|x| x.to_digit(10).unwrap() as i128)
        .permutations(4)
        .unique()
        .into_iter()
        .cartesian_product(repeat_n(operators.into_iter(), 3).multi_cartesian_product().into_iter())
        .cartesian_product(repeat_n([false, true].into_iter(), 6).multi_cartesian_product().into_iter())
        .collect()
    };

    // Calculate all combinations of digits and operators.
    let length = &digits_operators.len();
    let mut valid_count = 0;
    for ((digits, operators), f_positions) in digits_operators {
        if let Ok(result) = calculate(&digits, &operators, &f_positions, &factorial_cache, target) {
            if result == target {
                valid_count += 1;
            }
        }
    }

    let order_expr = if in_order {"with digits in order"} else {""};
    eprintln!(
        "{} of {} combinations {} for car {} make {}.",
        valid_count, length, order_expr, input_arg, target
    );
    Ok(())
}




// Given digits, operators, and factorial positions, calculate the result.
fn calculate(
    digits: &Vec<i128>,
    ops: &Vec<&str>,
    pos: &Vec<bool>,
    cache: &Vec<i128>,
    target: i128
) -> Result<i128, ()> {
    let num1 = digits[0];
    let num2 = digits[1];
    let num3 = digits[2];
    let num4 = digits[3];

    let operand1 = if pos[0] {math::factorial(num1, cache)?} else {num1};
    let operand2 = if pos[1] {math::factorial(num2, cache)?} else {num2};
    let operand3 = if pos[3] {math::factorial(num3, cache)?} else {num3};
    let operand4 = if pos[5] {math::factorial(num4, cache)?} else {num4};

    let result = operate(operand1, operand2, ops[0])?;
    let result = if pos[2] {math::factorial(result, cache)?} else {result};
    let result = operate(result, operand3, ops[1])?;
    let result = if pos[4] {math::factorial(result, cache)?} else {result};
    let result = operate(result, operand4, ops[2])?;

    if result == target {
        let expr1 = if pos[0] {format!("{num1}!")} else {num1.to_string()};
        let expr2 = if pos[1] {format!("{num2}!")} else {num2.to_string()};
        let expr3 = if pos[2] {"!"} else {""};
        let expr4 = if pos[3] {format!("{num3}!")} else {num3.to_string()};
        let expr5 = if pos[4] {"!"} else {""};
        let expr6 = if pos[5] {format!("{num4}!")} else {num4.to_string()};

        println!(
            "(({} {} {}){} {} {}){} {} {} = {}",
            expr1, ops[0], expr2, expr3, ops[1], expr4, expr5, ops[2], expr6, target
        );
    }

    Ok(result)
}


// Applies an operation.
fn operate(num1: i128, num2: i128, op: &str) -> Result<i128, ()> {
    match op {
        "+" => num1.checked_add(num2).ok_or(()),
        "-" => num1.checked_sub(num2).ok_or(()),
        "*" => num1.checked_mul(num2).ok_or(()),
        "/" => math::train_game_div(num1, num2),
        "mod" => math::train_game_mod(num1, num2),
        "<<" => math::safe_shift(num1, num2, op),
        ">>" => math::safe_shift(num1, num2, op),
        "bitwiseAND" => Ok(num1 & num2),
        "bitwiseOR" => Ok(num1 | num2),
        "bitwiseXOR" => Ok(num1 ^ num2),
        "^" => math::train_game_pow(num1, num2),
        _ => Err(())
    }
}


// Prints a help message.
fn print_help_msg() {
    eprintln!("Arguments: <car number> [-i] [-t <integer>]");
    eprintln!("\t-h: Displays the help that you are seeing here.");
    eprintln!("\t-i: Only consider solutions with digits in order.");
    eprintln!("\t-t: Set a custom target.");
}
