use nannou::{
    glam::Vec2,
    noise::{BasicMulti, NoiseFn, Perlin, Seedable},
    prelude::*,
};

static NOISE_STEP: f32 = 50.;
static CANVAS_SIZE: [f32; 2] = [400., 400.];

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
    let draw = app.draw();
    let boundary = app.window_rect();

    // advance by 1/500 per frame
    let current_step = (app.elapsed_frames()) as f32 / NOISE_STEP;
    let _x = Perlin::new().get([current_step.into(), 0.]);
    let x = map_range(_x, -1.0, 1.0, boundary.left(), boundary.right());

    let _y = Perlin::new().get([(current_step * 0.4).into(), 0.]);
    let y = map_range(_y, -1.0, 1.0, boundary.top(), boundary.bottom());

    draw.background().color(rgb(1.0, 1.0, 1.0));
    draw.ellipse()
        .w_h(10., 10.)
        .color(rgb(0.2, 0.2, 0.2))
        .x_y(x, y);

    // render the draw
    draw.to_frame(app, &frame).unwrap();
}

// fn update(app: &App, model: &mut Model, _update: Update) {}
