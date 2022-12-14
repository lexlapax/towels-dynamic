#![allow(unused)]

use nannou::prelude::*;

pub struct Model {
    window: window::Id,
    pub was_updated: bool,
    state: State,
}

impl Model {
    pub fn for_window(window: window::Id) -> Self {
        Self {
            window,
            state: State::default(),
            was_updated: false,
        }
    }
}

#[derive(Default, Debug)]
pub struct State {}

#[no_mangle]
pub fn event(app: &App, model: &mut Model, event: WindowEvent) {}

#[no_mangle]
pub fn update(app: &App, model: &mut Model, update: Update) {}

#[no_mangle]
pub fn view(app: &App, _model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(WHITE);
    draw.ellipse()
        .color(PURPLE)
        .width(40.0)
        .height(40.0);
    draw.to_frame(app, &frame).unwrap();
}
