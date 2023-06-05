use crate::common;
use itertools::Itertools;
use nannou::prelude::*;
use rand::Rng;

const FINAL_SQUARE_SIZE_PROPORTION: f32 = 0.1;

struct Square {
    x: f32,
    y: f32,
    size: f32,
    dx: f32,
    dy: f32,
    steps: usize,
}
type Model = Vec<Square>;

pub fn run() {
    nannou::app(generate_model)
        .event(|a, m, e| common::refresh_model_on_space(a, m, e, generate_model))
        .simple_window(view)
        .size(800, 800)
        .run();
}

fn generate_model(app: &App) -> Model {
    let size = app.main_window().inner_size_points().0 * 0.9;
    let n = 10_usize;
    let start = -size / 2.0;
    let step = size / n as f32;
    let mut rng = rand::thread_rng();
    let to_square = |(i, j): (usize, usize)| Square {
        x: start + step * i as f32,
        y: start + step * j as f32,
        size: step,
        dx: 0.4 * rng.gen_range(-1..=1) as f32,
        dy: 0.4 * rng.gen_range(-1..=1) as f32,
        steps: 3 + rng.gen_range(0..=2),
    };
    (0..n).cartesian_product(0..n).map(to_square).collect()
}

// note: contrary to the tutorial i decided to
// implement draw_square non-recursively
// the code worked out much cleaner this way
fn draw_square(square: &Square, draw: &Draw) {
    for step in 0..=square.steps {
        let t = step as f32 / square.steps as f32;
        let size = square.size * (1.0 - t) + square.size * FINAL_SQUARE_SIZE_PROPORTION * t;
        let (x, y) = {
            let even_displacement = (square.size - size) / 2.0;
            (
                square.x + even_displacement + (square.size - size) * square.dx * 0.5,
                square.y + even_displacement + (square.size - size) * square.dy * 0.5,
            )
        };
        draw.quad()
            .rgba(0.0, 0.0, 0.0, 0.0)
            .stroke_weight(2.0)
            .points(
                pt2(x, y),
                pt2(x + size, y),
                pt2(x + size, y + size),
                pt2(x, y + size),
            );
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(WHITE);
    for square in model {
        draw_square(square, &draw);
    }
    draw.to_frame(app, &frame).unwrap();
}
