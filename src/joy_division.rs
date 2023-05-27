use nannou::prelude::*;
use rand::Rng;

type Line = Vec<cgmath::Vector2<f32>>;

pub fn run() {
    nannou::app(generate_lines)
        .simple_window(view)
        .size(800, 800)
        .run();
}

fn draw_line(line: &Line, draw: &Draw) {
    // todo: spline interpolation should be down on model creation
    // not during the draw call
    let spline = {
        use splines::*;
        let step = 1.0_f32 / line.len() as f32;
        Spline::from_vec(
            line.iter()
                .enumerate()
                .map(|(i, v)| Key::new(i as f32 * step, *v, Interpolation::CatmullRom))
                .collect(),
        )
    };
    let points = {
        let resolution = 150_usize;
        let step = 1.0_f32 / resolution as f32;
        let mut ret = Vec::new();
        for i in 0..=resolution {
            if let Some(pt) = spline.sample(i as f32 * step) {
                ret.push(pt2(pt.x, pt.y));
            }
        }
        ret
    };
    draw.polyline().weight(2_f32).points(points);
}

fn generate_lines(app: &App) -> Vec<Line> {
    let n = 30;
    let line_resolution = 20;
    let height = app.main_window().inner_size_points().0;
    let vstep = height * 0.85 / (n as f32);
    let hstep = height / (line_resolution as f32);
    let vstart = -height / 2_f32 + 50.0;
    let hstart = -height / 2_f32;
    let mut rng = rand::thread_rng();
    let mut ret = Vec::new();
    for i in 0..n {
        let mut line = Vec::new();
        for j in 0..=line_resolution {
            let variance = (line_resolution / 2 - (j - line_resolution / 2).abs()) as f32;
            let deviation = rng.gen_range(0.0_f32..1.0_f32) * vstep * variance * variance * 0.05_f32;
            line.push(cgmath::Vector2 {
                x: hstart + (j as f32) * hstep,
                y: vstart + vstep * (i as f32) + deviation,
            })
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
