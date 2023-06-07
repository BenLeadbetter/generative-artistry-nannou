use crate::common;
use nannou::prelude::*;
use rand::Rng;

type Layer = Vec<Point2>;

pub fn run() {
    nannou::app(generate_layers)
        .event(|app, model, event| {
            common::refresh_model_on_space(app, model, event, generate_layers)
        })
        .simple_window(view)
        .size(800, 800)
        .run();
}

fn draw_layer(layer: &Layer, draw: &Draw) {
    for top in layer.windows(2) {
        draw.quad().color(WHITE).points(
            pt2(top[0].x, -400.0),
            top[0],
            top[1],
            pt2(top[1].x, -400.0),
        );
    }
    draw.polyline().weight(2_f32).points(layer.clone());
}

fn generate_layers(app: &App) -> Vec<Layer> {
    let n = 30;
    let m = 30;
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
                rng.gen_range(0.0_f32..1.0_f32) * vstep * variance * variance * 0.02_f32;
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

fn view(app: &App, model: &Vec<Layer>, frame: Frame) {
    let draw = app.draw();
    draw.background().color(WHITE);
    for layer in model.iter().rev() {
        draw_layer(layer, &draw);
    }
    draw.to_frame(app, &frame).unwrap();
}
