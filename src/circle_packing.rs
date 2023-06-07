use crate::common;
use nannou::prelude::*;
use rand::Rng;

struct Circle(Point2, f32);
type Model = Vec<Circle>;

impl Circle {
    fn intersects_circle(&self, other: &Circle) -> bool {
        (self.0 - other.0).length() < self.1 + other.1
    }
    fn intersects_square(&self, sq_size: f32) -> bool {
        let mut ret = false;
        ret = ret || self.0.x + self.1 > sq_size / 2.0;
        ret = ret || self.0.y + self.1 > sq_size / 2.0;
        ret = ret || self.0.x - self.1 < -sq_size / 2.0;
        ret = ret || self.0.y - self.1 < -sq_size / 2.0;
        ret
    }
}

pub fn run() {
    nannou::app(model)
        .event(|a, m, e| common::refresh_model_on_space(a, m, e, model))
        .simple_window(view)
        .size(800, 800)
        .run();
}

fn draw_circle(circle: &Circle, draw: &Draw) {
    draw.ellipse()
        .radius(circle.1)
        .xy(circle.0)
        .stroke_weight(2.0f32)
        .rgba(0.0, 0.0, 0.0, 0.0);
}

fn model(app: &App) -> Model {
    let size = app.main_window().inner_size_points().0 * 0.9;
    let n = 500_usize;
    let mut circles_created = 0_usize;
    let mut model = Vec::<Circle>::new();
    let mut rng = rand::thread_rng();
    loop {
        if circles_created == n {
            break;
        }
        let mut new_circle = Circle(
            pt2(
                rng.gen_range(-0.5..0.5) * size,
                rng.gen_range(-0.5..0.5) * size,
            ),
            3.0,
        );
        if model
            .iter()
            .any(|circle| circle.intersects_circle(&new_circle))
        {
            continue;
        }
        loop {
            let max_radius = new_circle.1 > size * 0.25;
            let intersects_edge = new_circle.intersects_square(size);
            let intersects_another_circle = model
                .iter()
                .any(|circle| circle.intersects_circle(&new_circle));
            if max_radius || intersects_edge || intersects_another_circle {
                break;
            }
            new_circle.1 += 0.2;
        }
        model.push(new_circle);
        circles_created += 1;
    }
    model
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(WHITE);
    for circle in model {
        draw_circle(circle, &draw);
    }
    draw.to_frame(app, &frame).unwrap();
}
