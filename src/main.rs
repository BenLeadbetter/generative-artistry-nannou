use clap::{Parser, ValueEnum};

mod cubic_disarray;
mod joy_division;
mod tiled_lines;
mod triangular_mesh;
mod un_deux_trois;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum Tutorial {
    TiledLines,
    JoyDivision,
    CubicDisarray,
    TriangularMesh,
    UnDeuxTrois,
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
        TriangularMesh => triangular_mesh::run(),
        UnDeuxTrois => un_deux_trois::run(),
    };
}
