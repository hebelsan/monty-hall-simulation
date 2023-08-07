use rand::Rng;
use clap::Parser;

#[derive(Parser)]
struct Cli {
    #[arg(short = 'n', long = "number")]
    number: i32,
}

fn main() {
    let args = Cli::parse();

    simulate(args.number)
}

fn simulate(times:i32) {
    let mut rng = rand::thread_rng();
    let mut wins = 0;
    for _ in 0..times {
        let player_door = rng.gen_range(0..=2);
        let price_door = rng.gen_range(0..=2);
        // game master opens door
        let game_master_door  = (0..3).map(|n| n).filter(|&n| n != player_door && n != price_door).next().expect("game master could not choose door");
        let resulting_door  = (0..3).map(|n| n).filter(|&n| n != player_door && n != game_master_door).next().expect("player could not switch door");
        if resulting_door == price_door {
            wins += 1;
        }
    }
    println!("wins: {} games: {} rate: {}%", wins, times, (wins as f32 /times as f32) * 100.0);
}