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
    let n = 30;
    let m = 20;
    let line_resolution = 200;
    let step = 1.0_f32 / m as f32;
    let height = app.main_window().inner_size_points().0;
    let vstep = height * 0.85 / (n as f32);
    let hstep = height / (m as f32);
    let vstart = -height / 2_f32 + 50.0;
    let hstart = -height / 2_f32;
    let mut rng = rand::thread_rng();

    let mut make_spline = |index: i32| {
        use splines::*;
        let mut make_point = |j: i32| {
            let variance = (m / 2 - (j - m / 2).abs()) as f32;
            let deviation =
                rng.gen_range(0.0_f32..1.0_f32) * vstep * variance * variance * 0.05_f32;
            pt2(
                hstart + (j as f32) * hstep,
                vstart + vstep * (index as f32) + deviation,
            )
        };
        Spline::from_vec(
            (0..=m)
                .map(|j| Key::new(j as f32 * step, make_point(j), Interpolation::CatmullRom))
                .collect(),
        )
    };

    let mut ret = Vec::new();
    for i in 0..n {
        let spline = make_spline(i);
        ret.push(
            (0..line_resolution)
                .map(|index| spline.sample(index as f32 / line_resolution as f32))
                .filter_map(|x| x)
                .collect(),
        );
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
