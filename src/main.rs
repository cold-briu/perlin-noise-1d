use nannou::{
    noise::{BasicMulti, NoiseFn, Perlin},
    prelude::*,
};

fn main() {
    // nannou::app(model).update(update).run()
    nannou::sketch(view).run()
}

// fn view(app: &App, model: &Model, frame: Frame) {
fn view(app: &App, frame: Frame) {
    let light_color = rgb(0.8, 0.8, 0.8);
    let mid_color = rgb(0.4, 0.4, 0.4);
    let dark_color = rgb(0.1, 0.1, 0.1);

    let boundary = app.window_rect();
    let full_w = boundary.w() as i32;
    let full_h = boundary.h() as i32;

    let resolution = 6;
    let cols = full_w / resolution;
    let rows = full_h / resolution;

    let draw = app.draw();
    draw.background().color(dark_color);

    // let noise = Perlin::new();
    let noise = BasicMulti::new();
    let mut points: Vec<Vec2> = vec![];

    for x in 0..cols {
        for y in 0..rows {
            let pos_x = map_range(x * resolution, 0, cols, boundary.left(), boundary.right());
            let pos_y = map_range(y * resolution, 0, rows, boundary.top(), boundary.bottom());

            draw.ellipse()
                .color(mid_color)
                .x_y(pos_x, pos_y)
                .w_h(2.0, 2.0);

            let p = noise.get([pos_x as f64, 0.]);
            let noise_y = map_range(p, -1.0, 1.0, boundary.top(), boundary.bottom());
            points.push(pt2(pos_x, noise_y as f32));
        }
    }

    draw.polyline()
        .x(boundary.left())
        .weight(1.0)
        .points(points)
        .color(light_color);

    // render the draw
    draw.to_frame(app, &frame).unwrap();
}
