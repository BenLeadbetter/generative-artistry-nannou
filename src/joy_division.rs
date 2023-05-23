use nannou::prelude::*;
use rand::Rng;

type Line = Vec<Point2>;

pub fn run() {
    nannou::app(generate_lines)
        .simple_window(view)
        .size(800, 800)
        .run();
}

fn draw_line(line: &Line, draw: &Draw) {
    draw.polyline().weight(2_f32).points(line.clone());
}

fn generate_lines(app: &App) -> Vec<Line> {
    let n = 20;
    let line_resolution = 20;
    let height = app.main_window().inner_size_points().0;
    let vstep = height / (n as f32);
    let hstep = height / (line_resolution as f32);
    let start = -height / 2_f32;
    let mut rng = rand::thread_rng();
    let mut ret = Vec::new();
    for i in 0..n {
        let mut line = Vec::new();
        for j in 0..=line_resolution {
            let variance = (line_resolution / 2 - (j - line_resolution / 2).abs()) as f32;
            let deviation = rng.gen_range(0.0_f32..1.0_f32) * vstep * variance * 0.5_f32;
            line.push(pt2(
                start + (j as f32) * hstep,
                start + vstep * (i as f32) + deviation,
            ))
        }
        ret.push(line);
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
