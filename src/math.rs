/*
Library for train game functions.

Written by Stephen.
Last updated 3 August 2024.
*/




// Divides num1 by num2, returning Err if not divisible.
pub fn train_game_div(num1: i128, num2: i128) -> Result<i128, ()> {
    if num2 == 0 {
        Err(())
    } else if num1 % num2 != 0 {
        Err(())
    } else {
        num1.checked_div(num2).ok_or(())
    }
}


// Computes num1 raised to the power of num2.
pub fn train_game_pow(num1: i128, num2: i128) -> Result<i128, ()> {
    if num1 == 0 && num2 == 0 {
        Err(())
    } else {
        num1.checked_pow(num2.try_into().unwrap()).ok_or(())
    }
}


// Computes num1 modulo num2.
pub fn train_game_mod(num1: i128, num2: i128) -> Result<i128, ()> {
    if num2 == 0 {
        Err(())
    } else {
        Ok(num1 % num2)
    }
}


// Factorial function.
pub fn factorial(num1: i128, cache: &Vec<i128>) -> Result<i128, ()> {
    if num1 > 33 || num1 < 0 {
        Err(())
    } else {
        Ok(cache[num1 as usize])
    }
}


// Safer bit shift on 128-bit integers.
pub fn safe_shift(num1: i128, num2: i128, op: &str) -> Result<i128, ()> {
    if num2 > 128 {
        Err(())
    } else if op == ">>" {
        Ok(num1 >> num2)
    } else {
        Ok(num1 << num2)
    }
}
