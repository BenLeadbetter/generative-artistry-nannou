use crate::common;
use nannou::prelude::*;

type Polygon = Vec<Point2>;
#[derive(Debug)]
struct Rectangle(Point2, Point2);
struct Edge {
    position: Point2,
    direction: Point2,
    inside: Point2,
}
type LineSegment = (Point2, Point2);

impl Edge {
    fn contains(&self, pt: Point2) -> bool {
        (pt - self.position).dot(self.inside) > 0.0
    }
    fn intersect(&self, line_segment: LineSegment) -> Option<Point2> {
        let a1 = self.position.x;
        let a2 = self.position.y;
        let b1 = self.direction.x;
        let b2 = self.direction.y;

        let p1 = line_segment.0.x;
        let p2 = line_segment.0.y;
        let q1 = line_segment.1.x;
        let q2 = line_segment.1.y;

        let determinant = b1 * (p2 - q2) - b2 * (p1 - q1);

        if determinant.abs() < std::f32::EPSILON {
            return None;
        }

        let t = (b1 * (a2 - q2) - b2 * (a1 - q1)) / determinant;

        if t < 0.0 || t > 1.0 {
            return None;
        }

        Some(t * line_segment.0 + (1.0 - t) * line_segment.1)
    }
}

impl Rectangle {
    fn edges(&self) -> [Edge; 4] {
        [
            Edge {
                position: self.bottom_left(),
                direction: self.top_left() - self.bottom_left(),
                inside: self.bottom_right() - self.bottom_left(),
            },
            Edge {
                position: self.top_left(),
                direction: self.top_right() - self.top_left(),
                inside: self.bottom_left() - self.top_left(),
            },
            Edge {
                position: self.top_right(),
                direction: self.bottom_right() - self.top_right(),
                inside: self.top_left() - self.top_right(),
            },
            Edge {
                position: self.bottom_right(),
                direction: self.bottom_left() - self.bottom_right(),
                inside: self.top_right() - self.bottom_right(),
            },
        ]
    }
    fn bottom_left(&self) -> Point2 {
        self.0
    }
    fn top_left(&self) -> Point2 {
        pt2(self.0.x, self.1.y)
    }
    fn top_right(&self) -> Point2 {
        self.1
    }
    fn bottom_right(&self) -> Point2 {
        pt2(self.1.x, self.0.y)
    }
    fn width(&self) -> f32 {
        self.span().x
    }
    fn height(&self) -> f32 {
        self.span().y
    }
    fn span(&self) -> Point2 {
        self.top_right() - self.bottom_left()
    }
}

// the Sutherland–Hodgman algorithm
fn clip(mut poly: Polygon, mask: &Rectangle) -> Polygon {
    if poly.is_empty() {
        return poly;
    }
    let mut temp = Polygon::new();
    for edge in mask.edges() {
        let mut previous = poly.last().copied();
        for &vertex in &poly {
            if let Some(prev) = previous {
                if let Some(intersect) = edge.intersect((prev, vertex)) {
                    temp.push(intersect);
                }
            }
            if edge.contains(vertex) {
                temp.push(vertex);
            }
            previous = Some(vertex);
        }
        std::mem::swap(&mut poly, &mut temp);
        temp.clear();
    }
    poly
}

pub fn run() {
    nannou::app(model)
        .simple_window(view)
        .size(800, 800)
        .run();
}

fn model(app: &App) -> Vec<Polygon> {
    let n = 365.0.sqrt().ceil() as usize;
    let size = app.main_window().inner_size_points().0 * 0.9;
    let start = -size / 2.0;
    let step = size / n as f32;
    let mut count = 0;
    let mut row = 0;
    let mut ret = Vec::new();
    loop {
        if count == 365 {
            break;
        }
        if count % n == 0 {
            row += 1;
        }
        let col = count % n;
        let bottom_left = pt2(start + row as f32 * step, -start - col as f32 * step) - pt2(step, step) * 0.5;
        let phi = count as f32 / 365.0 * PI;
        let angle = phi.sin() * PI * 0.45 + 2.42;
        let thickness = phi.cos().abs() * 2.0 + 1.0;
        let line = gen_line(&Rectangle(bottom_left, bottom_left + pt2(step, step)), angle, thickness);
        ret.push(line);
        count += 1;
    }
    ret
}

fn gen_line(rect: &Rectangle, angle: f32, thickness: f32) -> Polygon {
    let mid = rect.bottom_left() + rect.span() / 2.0;
    let v = (0.05 * rect.height() * pt2(0.0, 1.0)) * thickness;
    let w = rect.width() * pt2(1.0, 0.0);
    let mut ret = vec![
        mid + v + w,
        mid + v - w,
        mid - v - w,
        mid - v + w,
    ];
    for p in &mut ret {
        common::rotate_about_point(p, &mid, angle);
    }
    clip(ret, rect)
}

fn view(app: &App, model: &Vec<Polygon>, frame: Frame) {
    let draw = app.draw();
    draw.background().color(WHITE);
    for poly in model {
        draw.polygon()
            .points(poly.clone())
            .color(BLACK);
    }
    draw.to_frame(app, &frame).unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    fn polygons_equivalent(lhs: &Polygon, rhs: &Polygon) -> bool {
        if lhs.is_empty() || rhs.is_empty() {
            return lhs == rhs;
        }
        let Some(index) = rhs.iter().position(|&p| p == lhs[0]) else {
            return false;
        };
        let rhs_iter = {
            let mut ret = rhs.iter().cycle();
            for _ in 0..index {
                ret.next();
            }
            ret.take(lhs.len())
        };

        lhs.iter().eq(rhs_iter.clone()) || lhs.iter().rev().eq(rhs_iter)
    }

    fn assert(poly: Polygon, mask: &Rectangle, expected: &Polygon) {
        let clipped = clip(poly.clone(), &mask);
        if !polygons_equivalent(&clipped, &expected) {
            eprintln!("original = {:?}", poly);
            eprintln!("clipped against {:?} = {:?}", mask, clipped);
            panic!();
        }
    }

    #[test]
    fn clip_totally_intersecting_rectangles() {
        let poly = vec![pt2(0.0, 0.0), pt2(1.0, 0.0), pt2(1.0, 1.0), pt2(0.0, 1.0)];
        let mask = Rectangle(pt2(-1.0, -1.0), pt2(2.0, 2.0));

        assert(poly.clone(), &mask, &poly);
    }

    #[test]
    fn clip_mutually_exclusive_rectangles() {
        let poly = vec![pt2(0.0, 0.0), pt2(1.0, 0.0), pt2(1.0, 1.0), pt2(0.0, 1.0)];
        let mask = Rectangle(pt2(-1.0, -1.0), pt2(-0.5, -0.5));

        assert(poly, &mask, &Polygon::new());
    }

    #[test]
    fn intersecting_rectangles() {
        let poly = vec![pt2(0.0, 0.0), pt2(1.0, 0.0), pt2(1.0, 1.0), pt2(0.0, 1.0)];
        let mask = Rectangle(pt2(0.5, 0.5), pt2(2.0, 2.0));
        let intersection = vec![pt2(1.0, 0.5), pt2(1.0, 1.0), pt2(0.5, 1.0), pt2(0.5, 0.5)];

        assert(poly, &mask, &intersection);
    }
}
