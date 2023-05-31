use itertools::Itertools;
use nannou::prelude::*;
use rand::Rng;

type Square = [Point2; 4];

pub fn run() {
    nannou::app(generate_squares)
        .simple_window(view)
        .size(800, 800)
        .run();
}

fn draw_square(square: &Square, draw: &Draw) {
    draw
        .quad()
        .points(square[0], square[1], square[2], square[3])
        .stroke_weight(2.0f32)
        .rgba(0.0, 0.0, 0.0, 0.0);
}

fn generate_squares(app: &App) -> Vec<Square> {
    let n = 9_usize;
    let (start, step) = {
        let width = app.main_window().inner_size_points().0;
        let draw_area = 0.9;
        (- draw_area * width / 2.0, width * draw_area / n as f32)
    };

    let mut squares = Vec::new();
    let mut rng = rand::thread_rng();
    for (i, j) in (0..n).cartesian_product(0..n) {
        let i = i as f32;
        let j = j as f32;
        let n = n as f32;
        let mut square = [
            vec2(- step / 2.0f32, - step / 2.0f32),
            vec2(step / 2.0f32, - step / 2.0f32),
            vec2(step / 2.0f32, step / 2.0f32),
            vec2(- step / 2.0f32, step / 2.0f32),
        ];
        let variance = 1.0 - j / (n - 1.0);
        let rotate = rng.gen_range(-1.0f32..1.0f32) * variance * std::f32::consts::PI * 0.06;
        let displacement = 
            vec2(start + (i + 0.5) * step, start + (j + 0.5) * step)
            + vec2(rng.gen_range(-1.0f32..1.0f32) * variance * step / 2.0f32, 0.0);
        for point in &mut square {
            *point = point.rotate(rotate);
            *point += displacement;
        }
        squares.push(square);
    }

    squares
}

fn view(app: &App, model: &Vec<Square>, frame: Frame) {
    let draw = app.draw();
    draw.background().color(WHITE);
    for square in model {
        draw_square(square, &draw);
    }
    draw.to_frame(app, &frame).unwrap();
}
