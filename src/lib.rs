use rand::Rng;
use rayon::prelude::*;

pub fn simulate_sequential(rounds: i32) -> i32 {
    let wins = (0..rounds)
        .map(|_| simulate_single_round())
        .sum();

    wins
}

pub fn simulate_parallel(rounds: i32) -> i32 {
    let wins = (0..rounds)
        .into_par_iter() // Parallel iterator
        .map(|_| simulate_single_round())
        .sum();

    wins
}

fn simulate_single_round() -> i32 {
    let mut rng = rand::thread_rng();
    let player_door = rng.gen_range(0..3);
    let prize_door = rng.gen_range(0..3);

    // Game master opens door
    let game_master_door = (0..3).find(|&n| n != player_door && n != prize_door).unwrap();

    // Player switches door
    let resulting_door = (0..3).find(|&n| n != player_door && n != game_master_door).unwrap();

    if resulting_door == prize_door {
        1
    } else {
        0
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn simulate_10000_sequential() {
        let rounds = 10000;
        let wins = simulate_sequential(rounds);
        let win_rate = (wins as f32 / rounds as f32) * 100.0;
        assert!(win_rate > 60.);
    }
    
    #[test]
    fn simulate_10000_parralel() {
        let rounds = 10000;
        let wins = simulate_parallel(rounds);
        let win_rate = (wins as f32 / rounds as f32) * 100.0;
        assert!(win_rate > 60.);
    }
}