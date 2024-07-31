/*
Solves the train game (specified below) by brute force.

Take the four digits of the car number you are on and try to make 10.

Not implemented: factorial.

Implementation-specific rules:
- 0 to the power of 0 is undefined.
- Operands for division must be exactly divisible.
- d/dx is not permitted.
- base conversion is not permitted.

Written by Stephen.
Last updated 31 July 2024.
*/


use itertools::Itertools;
use itertools::repeat_n;




fn main() -> Result<(), String> {
    // Take 4-digit number from commandline arg.
    let input_arg = std::env::args().nth(1).unwrap_or_default();
    if input_arg.len() != 4 || input_arg.parse::<u128>().is_err() {
        return Err(String::from("Argument must be a 4-digit number."));
    }

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

    // Get an iterator over every arrangement of digits and every arrangement of 
    // operators.
    let digits_operators: Vec<(Vec<i128>, Vec<&str>)> = std::env::args()
        .nth(1)
        .unwrap()
        .chars()
        .map(|x| x.to_digit(10).unwrap() as i128)
        .permutations(4)
        .into_iter()
        .cartesian_product(repeat_n(operators.into_iter(), 3).multi_cartesian_product().into_iter())
        .collect();

    // Calculate all combinations of digits and operators.
    let mut valid_combinations = 0;
    for (digits, operators) in &digits_operators {
        if let Ok(result) = calculate(digits.to_vec(), operators.to_vec()) {
            if result == 10 {
                valid_combinations += 1;
            }
        }
    }

    println!("\n{} of {} combinations are valid.", valid_combinations, digits_operators.len());
    Ok(())
}




// Given digits and operators, calculate the result.
fn calculate(digits: Vec<i128>, operators: Vec<&str>) -> Result<i128, ()> {
    let num1 = digits[0];
    let num2 = digits[1];
    let num3 = digits[2];
    let num4 = digits[3];

    let op1 = operators[0];
    let op2 = operators[1];
    let op3 = operators[2];

    let result = operate(num1, num2, op1)?;
    let result = operate(result, num3, op2)?;
    let result = operate(result, num4, op3)?;

    if result == 10 {
        println!(
            "(({} {} {}) {} {}) {} {} = 10",
            num1, op1, num2, op2, num3, op3, num4
        );
    }

    Ok(result)
}


// Divides num1 by num2, returning Err if not divisible.
fn train_game_div(num1: i128, num2: i128) -> Result<i128, ()> {
    if num2 == 0 {
        Err(())
    } else if num1 % num2 != 0 {
        Err(())
    } else {
        num1.checked_div(num2).ok_or(())
    }
}


fn train_game_pow(num1: i128, num2: i128) -> Result<i128, ()> {
    if num1 == 0 && num2 == 0 {
        Err(())
    } else {
        num1.checked_pow(num2.try_into().unwrap()).ok_or(())
    }
}


fn train_game_mod(num1: i128, num2: i128) -> Result<i128, ()> {
    if num2 == 0 {
        Err(())
    } else {
        Ok(num1 % num2)
    }
}


// Applies an operation.
fn operate(num1: i128, num2: i128, op: &str) -> Result<i128, ()> {
    match op {
        "+" => Ok(num1 + num2),
        "-" => Ok(num1 - num2),
        "*" => num1.checked_mul(num2).ok_or(()),
        "/" => train_game_div(num1, num2),
        "mod" => train_game_mod(num1, num2),
        "<<" => Ok(num1 << num2),
        ">>" => Ok(num1 >> num2),
        "bitwiseAND" => Ok(num1 & num2),
        "bitwiseOR" => Ok(num1 | num2),
        "bitwiseXOR" => Ok(num1 ^ num2),
        "^" => train_game_pow(num1, num2),
        _ => Err(())
    }
}
