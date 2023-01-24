use nannou::{
    noise::{NoiseFn, Perlin},
    prelude::*,
};

static NOISE_STEP: f32 = 300.;
// static CANVAS_SIZE: [f32; 2] = [400., 400.];

// struct Model {
//     noise: BasicMulti,
//     points: Vec<Vec2>,
//     frame_start: u64,
//     xoff: i32,
// }

// fn model(app: &App) -> Model {
//     let _window = app
//         .new_window()
//         .size(CANVAS_SIZE[0], CANVAS_SIZE[1])
//         .view(view)
//         .build()
//         .unwrap();

//     Model {
//         noise: BasicMulti::new(),
//         points: vec![],
//         frame_start: 0,
//     }
// }

fn main() {
    // nannou::app(model).update(update).run()
    nannou::sketch(view).run()
}

// fn view(app: &App, model: &Model, frame: Frame) {
fn view(app: &App, frame: Frame) {
    let light_color = rgb(0.2, 0.2, 0.2);
    let boundary = app.window_rect();

    let draw = app.draw();
    draw.background().color(rgb(1.0, 1.0, 1.0));

    // let current_step = (app.elapsed_frames()) as f32 / NOISE_STEP;
    // let _x = Perlin::new().get([current_step.into(), 0.]);
    // let x = map_range(_x, -1.0, 1.0, boundary.left(), boundary.right());

    // draw.ellipse().w_h(10., 10.).color(light_color).x_y(x, 10.);

    // draw.line()
    //     .start(Vec2::new(boundary.left(), 0.))
    //     .end(Vec2::new(boundary.right(), 0.))
    //     .color(light_color);

    let points = (0..boundary.w() as i32).map(|i| {
        let x = i as f32 - 25.0; //subtract 25 to center the sine wave
        let point = pt2(x, x.sin()) * 20.0; //scale sine wave by 20.0
        point
    });

    draw.polyline()
        .x(0 as f32)
        .weight(1.0)
        .points(points)
        .color(light_color);

    // render the draw
    draw.to_frame(app, &frame).unwrap();
}

// fn update(app: &App, model: &mut Model, _update: Update) {}
