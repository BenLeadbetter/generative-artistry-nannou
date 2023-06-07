use crate::common;
use itertools::Itertools;
use nannou::prelude::*;
use rand::Rng;

#[derive(PartialEq)]
enum Direction {
    Forwards,
    Backwards,
}

struct Line {
    x: f32,
    y: f32,
    step: f32,
    direction: Direction,
}

pub fn run() {
    nannou::app(generate_lines)
        .event(|a, m, e| common::refresh_model_on_space(a, m, e, generate_lines))
        .simple_window(view)
        .size(800, 800)
        .run();
}

fn draw_line(line: &Line, draw: &Draw) {
    let (start, end) = match line.direction {
        Direction::Forwards => (
            pt2(line.x, line.y),
            pt2(line.x + line.step, line.y + line.step),
        ),
        Direction::Backwards => (
            pt2(line.x + line.step, line.y),
            pt2(line.x, line.y + line.step),
        ),
    };
    draw.line()
        .start(start)
        .end(end)
        .weight(2_f32)
        .caps_square();
}

fn generate_lines(app: &App) -> Vec<Line> {
    let n = 30;
    let width = app.main_window().inner_size_points().0;
    let step = width / (n as f32);
    let start = -width / 2_f32;
    let mut rng = rand::thread_rng();
    let mut ret = Vec::new();
    for (i, j) in (0..n).cartesian_product(0..n) {
        use Direction::*;
        ret.push(Line {
            x: start + (i as f32) * step,
            y: start + (j as f32) * step,
            step,
            direction: if rng.gen() { Forwards } else { Backwards },
        })
    }
    ret
}

fn view(app: &App, model: &Vec<Line>, frame: Frame) {
    let draw = app.draw();
    draw.background().color(WHITE);
    for line in model {
        draw_line(line, &draw);
    }
    draw.to_frame(app, &frame).unwrap();
}
