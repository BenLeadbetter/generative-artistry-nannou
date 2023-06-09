use clap::{Parser, ValueEnum};

mod circle_packing;
mod common;
mod cubic_disarray;
mod hours_of_dark;
mod hypnotic_squares;
mod joy_division;
mod piet_mondrian;
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
    CirclePacking,
    HypnoticSquares,
    PietMondrian,
    HoursOfDark,
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
        CirclePacking => circle_packing::run(),
        HypnoticSquares => hypnotic_squares::run(),
        PietMondrian => piet_mondrian::run(),
        HoursOfDark => hours_of_dark::run(),
    };
}
