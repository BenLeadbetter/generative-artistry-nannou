use crate::common;
use itertools::Itertools;
use nannou::prelude::*;
use rand::Rng;

type Line = (Point2, Point2);
type Model = Vec<Line>;

pub fn run() {
    nannou::app(generate_model)
        .event(|a, m, e| common::refresh_model_on_space(a, m, e, generate_model))
        .simple_window(view)
        .size(800, 800)
        .run();
}

fn generate_model(app: &App) -> Model {
    let n = 15;
    let width = app.main_window().inner_size_points().0 * 0.9;
    let step = width / (n as f32);
    let start = -width / 2.0;
    let mut _rng = rand::thread_rng();
    let mut rng = rand::thread_rng();
    let mut ret = Vec::new();
    for (i, j) in (0..n).cartesian_product(0..n) {
        let nlines = 3 - j / 5;
        let (line_start, line_step) = match nlines {
            1 => (0.5, 0.5),
            2 => (0.2, 0.6),
            3 => (0.1, 0.4),
            _ => unreachable!(),
        };
        let i = i as f32;
        let j = j as f32;
        let mid_point = pt2(start + step * (i + 0.5), start + step * (j + 0.5));
        let angle = rng.gen_range(0.0..std::f32::consts::PI);
        for k in 0..nlines {
            let k = k as f32;
            let mut p1 = pt2(
                start + step * (i + line_step * k + line_start),
                start + step * j,
            );
            let mut p2 = pt2(
                start + step * (i + line_step * k + line_start),
                start + step * (j + 1.0),
            );
            common::rotate_about_point(&mut p1, &mid_point, angle);
            common::rotate_about_point(&mut p2, &mid_point, angle);
            ret.push((p1, p2));
        }
    }
    ret
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(WHITE);
    for line in model {
        draw.line()
            .start(line.0)
            .end(line.1)
            .caps_round()
            .stroke_weight(5.0);
    }
    draw.to_frame(app, &frame).unwrap();
}
