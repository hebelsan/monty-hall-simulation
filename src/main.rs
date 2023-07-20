use std::env;
use rand::Rng;

fn main() {
    let args: Vec<String> = env::args().collect();

    match args.len() {
        // one argument passed
        2 => {
            let num = &args[1];
            // parse the number
            match num.parse::<i32>() {
                Ok(n) => {
                    simulate(n)
                },
                Err(_) => {
                    eprintln!("error: argument not an integer");
                    help();
                    return;
                },
            };
        },
        _ => {
            eprintln!("error: incorrect number of arguments");
            help();
        }
    }
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

fn help() {
    println!("usage: cargo run <number_of_simulation_rounds>");
}