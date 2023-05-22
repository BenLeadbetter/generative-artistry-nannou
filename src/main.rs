use clap::{Parser, ValueEnum};

mod tiled_lines;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum Tutorial {
    TiledLines,
}

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[arg(value_enum)]
    tutorial: Tutorial,
}

fn main() {
    let _cli = Cli::parse();
    tiled_lines::tiled_lines();
}
