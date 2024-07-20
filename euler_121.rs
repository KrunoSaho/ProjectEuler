use core::panic;

use rand::prelude::*;
use rand_xoshiro::Xoshiro256Plus;
use rayon::prelude::*;

fn run_sim(payout: i64, max_turns: u32, iterations: u32) -> (f64, f64) {
    let mut rng = Xoshiro256Plus::from_entropy();

    let mut times_won = 0;
    let mut avg_banker_cash = 0;

    for _ in 0..iterations {
        let mut score = 0;
        let mut has_player_won = false;

        for i in 0..max_turns as usize {
            let player_wins = rng.gen_ratio(1, 2 + i as u32);

            if player_wins {
                score += 1;
            }

            // players has blue > red at this time
            if score > max_turns / 2 {
                has_player_won = true;
                break;
            }
        }

        if has_player_won {
            times_won += 1;
            avg_banker_cash -= payout - 1;
        } else {
            avg_banker_cash += 1;
        }
    }

    (
        times_won as f64 / iterations as f64,
        avg_banker_cash as f64 / iterations as f64,
    )
}

// 4 turn game = 11/120 = 0.09166666666666666
fn main() {
    let mut rng = Xoshiro256Plus::from_entropy();

    let lower_end = 2260;
    let upper_end = 2270;

    let mut payouts = (0..300)
        .map(|_| rng.gen_range(lower_end..upper_end))
        .collect::<Vec<i64>>();
    payouts.sort();
    payouts.dedup();

    let turns = 15;
    let n = 10_000_000;


    let mut data = payouts
        .par_iter()
        .filter_map(|&payout| {
            let (_, aba) = run_sim(payout, turns, n);

            if aba < 0.0 {
                return Some(payout);
            }

            None
        })
        .collect::<Vec<i64>>();

    data.sort();

    for payout in data {
        let mut avg_banker_cash = 0.0;

        for _ in 0..100 {
            let (_, aba) = run_sim(payout, turns, n);

            avg_banker_cash += aba;
        }

        avg_banker_cash /= 100.0;

        if avg_banker_cash < 0.0 {
            println!("Payout: {}", payout);
            println!();
            break;
        }
    }
}
