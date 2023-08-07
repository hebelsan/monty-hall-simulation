use clap::Parser;

#[derive(Parser)]
struct Cli {
    #[arg(short = 'r', long = "rounds")]
    rounds: i32,
}

fn main() {
    let args = Cli::parse();

    let wins = montyrs::simulate(args.rounds);

    let win_rate = (wins as f32 /args.rounds as f32) * 100.0;
    println!("wins: {} games: {} win-rate: {}%", wins, args.rounds, win_rate);
}