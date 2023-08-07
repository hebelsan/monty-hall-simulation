use clap::Parser;

#[derive(Parser)]
struct Cli {
    #[arg(short = 'n', long = "number")]
    number: i32,
}

fn main() {
    let args = Cli::parse();

    let wins = montyrs::simulate(args.number);
    
    let win_rate = (wins as f32 /args.number as f32) * 100.0;
    println!("wins: {} games: {} win-rate: {}%", wins, args.number, win_rate);
}