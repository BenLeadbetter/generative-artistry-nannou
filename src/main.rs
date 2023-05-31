use clap::{Parser, ValueEnum};

mod joy_division;
mod tiled_lines;
mod cubic_disarray;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum Tutorial {
    TiledLines,
    JoyDivision,
    CubicDisarray,
}

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[arg(value_enum)]
    tutorial: Tutorial,
}

fn main() {
    let cli = Cli::parse();
    use Tutorial::*;
    match cli.tutorial {
        TiledLines => tiled_lines::run(),
        JoyDivision => joy_division::run(),
        CubicDisarray => cubic_disarray::run(),
    };
}
