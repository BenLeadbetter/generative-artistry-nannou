use crate::common;
use nannou::prelude::*;
use rand::Rng;

const BLUE: (u8, u8, u8) = (19, 86, 162);
const RED: (u8, u8, u8) = (212, 9, 32);
const WHITE: (u8, u8, u8) = (255, 255, 255);
const YELLOW: (u8, u8, u8) = (247, 216, 68);

#[derive(Clone, Copy)]
struct Quad([Point2; 2], (f32, f32, f32));

impl Quad {
    pub fn contains_x(&self, x: f32) -> bool {
        self.0[0].x < x && x < self.0[1].x
    }
    pub fn contains_y(&self, y: f32) -> bool {
        self.0[0].y < y && y < self.0[1].y
    }
}

type Model = Vec<Quad>;

pub fn run() {
    nannou::app(model)
        .event(|a, m, e| common::refresh_model_on_space(a, m, e, model))
        .simple_window(view)
        .size(800, 800)
        .run();
}

fn color(color: &(u8, u8, u8)) -> (f32, f32, f32) {
    let map = |component: u8| component as f32 / 255.0;
    (map(color.0), map(color.1), map(color.2))
}

fn split<P: Fn(&Quad) -> bool, S: Fn(Quad) -> [Quad; 2]>(
    predicate: P,
    split_fn: S,
    model: &mut Model,
) {
    let mut new_quads = Vec::new();
    let mut rng = rand::thread_rng();
    loop {
        let Some(index) = model.iter().position(&predicate) else {
            break;
        };
        if rng.gen() {
            break;
        }
        let quad = model.remove(index);
        let split_quads = split_fn(quad);
        new_quads.push(split_quads[0]);
        new_quads.push(split_quads[1]);
    }
    model.append(&mut new_quads);
}

fn split_quads_with(pt: &Point2, model: &mut Model) {
    split(|q| q.contains_x(pt.x), |q| split_quad_on_x(pt.x, q), model);
    split(|q| q.contains_y(pt.y), |q| split_quad_on_y(pt.y, q), model);
}

fn split_quad_on_x(x: f32, quad: Quad) -> [Quad; 2] {
    [
        Quad([pt2(quad.0[0].x, quad.0[0].y), pt2(x, quad.0[1].y)], quad.1),
        Quad([pt2(x, quad.0[0].y), pt2(quad.0[1].x, quad.0[1].y)], quad.1),
    ]
}

fn split_quad_on_y(y: f32, quad: Quad) -> [Quad; 2] {
    [
        Quad([pt2(quad.0[0].x, quad.0[0].y), pt2(quad.0[1].x, y)], quad.1),
        Quad([pt2(quad.0[0].x, y), pt2(quad.0[1].x, quad.0[1].y)], quad.1),
    ]
}

fn model(app: &App) -> Model {
    let size = app.main_window().inner_size_points().0 * 0.9;
    let start = -size / 2.0;
    let mut model = vec![Quad(
        [pt2(start, start), pt2(start + size, start + size)],
        color(&WHITE),
    )];
    let n = 6_usize;
    let step = size / n as f32;
    for i in 0..n {
        let pt = {
            let i = i as f32;
            let coord = start + i * step;
            pt2(coord, coord)
        };
        split_quads_with(&pt, &mut model);
    }

    {
        // color three squares
        let mut rng = rand::thread_rng();
        let len = model.len();
        model[rng.gen_range(0..len)].1 = color(&YELLOW);
        model[rng.gen_range(0..len)].1 = color(&RED);
        model[rng.gen_range(0..len)].1 = color(&BLUE);
    }

    model
}

fn draw_quad(quad: &Quad, draw: &Draw) {
    draw.quad()
        .points(
            pt2(quad.0[0].x, quad.0[0].y),
            pt2(quad.0[1].x, quad.0[0].y),
            pt2(quad.0[1].x, quad.0[1].y),
            pt2(quad.0[0].x, quad.0[1].y),
        )
        .rgb(quad.1 .0, quad.1 .1, quad.1 .2)
        .stroke_weight(5.0f32);
}

fn view(app: &App, m: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(nannou::color::WHITE);
    for quad in m {
        draw_quad(&quad, &draw);
    }
    draw.to_frame(app, &frame).unwrap();
}
