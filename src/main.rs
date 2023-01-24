use nannou::prelude::*;

fn main() {
    nannou::sketch(view).run()
}

fn view(app: &App, frame: Frame) {
    // Prepare to draw.
    let draw = app.draw();
    let boundary = app.window_rect();
    let sine = app.time.sin();
    let x = map_range(sine, -1.0, 1.0, boundary.left(), boundary.right());

    draw.background().color(rgb(1.0, 1.0, 1.0));
    draw.ellipse().color(rgb(0.2, 0.2, 0.2)).x_y(x, 0.0);

    // render the draw
    draw.to_frame(app, &frame).unwrap();
}
