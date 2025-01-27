use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "peer-review")]
#[command(about = "Review coworker performance by quarter")]

struct Cli {
    /// Quarter to analyze (1-4)
    #[arg(short, long)]
    quarter: u8,

    /// Year to analyze
    #[arg(short, long, default_value_t = 2025)]
    year: i32,
}

fn main() {
    let cli = Cli::parse();

    // 驗證季度輸入
    if !(1..=4).contains(&cli.quarter) {
        eprintln!("Error: Quarter must be between 1 and 4");
        std::process::exit(1);
    }

    println!("Analyzing Q{} {}", cli.quarter, cli.year);
}
