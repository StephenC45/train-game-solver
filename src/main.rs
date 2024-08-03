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
    if input_arg.len() != 4 || input_arg.parse::<u128>().is_err() {
        return Err(String::from("Argument must be a 4-digit number."));
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
    let digits_operators: Vec<((Vec<i128>, Vec<&str>), Vec<bool>)> = input_arg
        .chars()
        .map(|x| x.to_digit(10).unwrap() as i128)
        .permutations(4)
        .unique()
        .into_iter()
        .cartesian_product(repeat_n(operators.into_iter(), 3).multi_cartesian_product().into_iter())
        .cartesian_product(repeat_n([false, true].into_iter(), 6).multi_cartesian_product().into_iter())
        .collect();

    // Calculate all combinations of digits and operators.
    let length = &digits_operators.len();
    let mut valid_combinations = 0;
    for ((digits, operators), factorial_places) in digits_operators {
        if let Ok(result) = calculate(&digits, &operators, &factorial_places, &factorial_cache) {
            if result == 10 {
                valid_combinations += 1;
            }
        }
    }

    eprintln!("\n{} of {} combinations are valid.", valid_combinations, length);
    Ok(())
}




// Given digits, operators, and factorial positions, calculate the result.
fn calculate(
    digits: &Vec<i128>,
    ops: &Vec<&str>,
    pos: &Vec<bool>,
    cache: &Vec<i128>,
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

    if result == 10 {
        let expr1 = if pos[0] {format!("{num1}!")} else {num1.to_string()};
        let expr2 = if pos[1] {format!("{num2}!")} else {num2.to_string()};
        let expr3 = if pos[2] {"!"} else {""};
        let expr4 = if pos[3] {format!("{num3}!")} else {num3.to_string()};
        let expr5 = if pos[4] {"!"} else {""};
        let expr6 = if pos[5] {format!("{num4}!")} else {num4.to_string()};

        println!(
            "(({} {} {}){} {} {}){} {} {} = 10",
            expr1, ops[0], expr2, expr3, ops[1], expr4, expr5, ops[2], expr6
        );
    }

    Ok(result)
}


// Applies an operation.
fn operate(num1: i128, num2: i128, op: &str) -> Result<i128, ()> {
    match op {
        "+" => Ok(num1 + num2),
        "-" => Ok(num1 - num2),
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
