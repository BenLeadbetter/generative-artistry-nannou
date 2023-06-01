use nannou::{color::Component, prelude::*};
use rand::Rng;

type Coord = (usize, usize);
type Grid = Vec<Vec<(Point2, Coord)>>;
type ColorMap = std::collections::HashMap<(Coord, Coord, Coord), f32>;
type Model = (Grid, ColorMap);

pub fn run() {
    nannou::app(generate_model)
        .simple_window(view)
        .size(800, 800)
        .run();
}

fn generate_model(app: &App) -> Model {
    let grid = generate_grid(&app);
    let color_map = generate_color_map(&grid);
    (grid, color_map)
}

fn key(triangle: &[(Point2, Coord); 3]) -> (Coord, Coord, Coord) {
    (triangle[0].1, triangle[1].1, triangle[2].1)
}

fn generate_color_map(grid: &Grid) -> ColorMap {
    let mut map = ColorMap::new();
    let mut rng = rand::thread_rng();
    for_each_triangle(&grid, |triangle| {
        map.insert(key(triangle), rng.gen_range(0.0..f32::max_intensity()));
    });
    map
}

fn generate_grid(app: &App) -> Grid {
    let n = 7_usize;
    let size = app.main_window().inner_size_points().0 * 0.9;
    let xstart = -size / 2.0;
    let xstep = size / n as f32;
    let ystep = xstep * 3.0.sqrt() * 0.5;
    let m = (size / ystep).floor() as usize + 1;
    let ystart = -((m - 1) as f32 * ystep) * 0.5;
    let mut grid = Vec::new();
    let mut rng = rand::thread_rng();
    for j in 0..m {
        let mut row = Vec::new();
        for i in 0..n {
            let xoffset = if j % 2 == 0 { xstep * 0.5 } else { 0.0 };
            row.push((pt2(
                xstart + i as f32 * xstep + xoffset + rng.gen_range(-0.3..0.3) * xstep,
                ystart + j as f32 * ystep + rng.gen_range(-0.3..0.3) * ystep,
            ), (i, j)));
        }
        grid.push(row);
    }
    grid
}

fn for_each_triangle<F: FnMut(&[(Point2, (usize, usize)); 3])>(grid: &Grid, mut func: F) {
    for (i, strip) in grid.windows(2).enumerate() {
        {
            //    o
            //
            // o     o
            let mut top_points = strip[1].iter();
            if i % 2 == 0 {
                top_points.next();
            }
            for points in strip[0]
                .windows(2)
                .zip(top_points)
                .map(|(sl, p)| [*p, sl[0], sl[1]])
            {
                func(&points);
            }
        }

        {
            // o     o
            //
            //    o
            let mut top_points = strip[0].iter();
            if i % 2 == 1 {
                top_points.next();
            }
            for points in strip[1]
                .windows(2)
                .zip(top_points)
                .map(|(sl, p)| [*p, sl[0], sl[1]])
            {
                func(&points);
            }
        }
    }
}

fn draw_triangle(triangle: &[(Point2, (usize, usize)); 3], draw: &Draw, color_map: &ColorMap) {
    draw.polygon()
        .rgba(0.0, 0.0, 0.0, 0.0)
        .stroke_weight(2.0)
        .gray(color_map[&key(&triangle)])
        .points(triangle.iter().map(|p| p.0));
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(WHITE);
    for_each_triangle(&model.0, |points| { draw_triangle(points, &draw, &model.1); });
    draw.to_frame(app, &frame).unwrap();
}
