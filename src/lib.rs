use rand::Rng;

pub fn simulate(rounds:i32) -> i32 {
    let mut rng = rand::thread_rng();
    let mut wins = 0;
    for _ in 0..rounds {
        let player_door = rng.gen_range(0..=2);
        let price_door = rng.gen_range(0..=2);
        // game master opens door
        let game_master_door  = (0..3).map(|n| n).filter(|&n| n != player_door && n != price_door).next().expect("game master could not choose door");
        let resulting_door  = (0..3).map(|n| n).filter(|&n| n != player_door && n != game_master_door).next().expect("player could not switch door");
        if resulting_door == price_door {
            wins += 1;
        }
    }

    return wins
}

#[test]
fn simulate_1000() {
    let rounds = 1000;
    let wins = simulate(rounds);
    let win_rate = (wins as f32 / rounds as f32) * 100.0;
    assert!(win_rate > 60.);
}