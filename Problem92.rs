/***
 *
 * Project Euler 92
 *
 */

use std::collections::HashMap;

fn main() {
    let mut history = HashMap::<i64, i64>::new();
    let n = 10_000_000_i64;
    let mut counter = 0;

    for i in 1..n {
        counter += if create_number_chain(&mut history, i) {
            1
        } else {
            0
        };
    }

    println!(
        "There are {} numbers below {} that arrive at 89",
        counter, n
    );
}

fn create_number_chain(main_history: &mut HashMap<i64, i64>, n: i64) -> bool {
    let mut next_number = n;
    let mut history = vec![next_number];

    loop {
        let square_digit_sum = next_square_digit(next_number);
        next_number = square_digit_sum;
        history.push(next_number);

        if main_history.contains_key(&next_number) {
            next_number = 89;
            break;
        }

        if next_number == 1 || next_number == 89 {
            break;
        }
    }

    if next_number == 89 {
        for number in history {
            main_history.insert(number, 89);
        }

        return true;
    }

    false
}

fn next_square_digit(n: i64) -> i64 {
    split_digits(n).iter().fold(0_i64, |acc, x| acc + x * x)
}

fn split_digits(n: i64) -> Vec<i64> {
    let mut digits = Vec::new();
    let mut n = n;
    while n > 0 {
        let next_digit = n % 10;
        if next_digit != 0 {
            digits.push(next_digit);
        }
        n /= 10;
    }
    digits.reverse();
    digits
}

// tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn number_chain_arrives_at_89_test() {
        let mut history = HashMap::<i64, i64>::new();

        assert_eq!(create_number_chain(&mut history, 1), false);
        assert_eq!(create_number_chain(&mut history, 2), true);
        assert_eq!(create_number_chain(&mut history, 4), true);
        assert_eq!(create_number_chain(&mut history, 85), true);
        assert_eq!(create_number_chain(&mut history, 89), true);
        assert_eq!(create_number_chain(&mut history, 3), true);
    }

    #[test]
    fn next_square_digit_test() {
        assert_eq!(next_square_digit(2), 4);

        assert_eq!(next_square_digit(44), 32);
        assert_eq!(next_square_digit(32), 13);
        assert_eq!(next_square_digit(13), 10);
        assert_eq!(next_square_digit(10), 1);
        assert_eq!(next_square_digit(1), 1);

        assert_eq!(next_square_digit(85), 89);
        assert_eq!(next_square_digit(89), 145);
        assert_eq!(next_square_digit(145), 42);
        assert_eq!(next_square_digit(42), 20);
        assert_eq!(next_square_digit(20), 4);
        assert_eq!(next_square_digit(4), 16);
        assert_eq!(next_square_digit(16), 37);
        assert_eq!(next_square_digit(37), 58);
        assert_eq!(next_square_digit(58), 89);
    }
}
