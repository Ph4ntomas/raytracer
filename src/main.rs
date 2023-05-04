use nannou::prelude::*;

mod bin;
use bin::Model;

fn update(_: &App, _model: &mut Model, _update: Update) {}

fn model(_: &App) -> Model {
    Model {}
}

fn view(_: &App, _model: &Model, _frame: Frame) {}

fn main() {
    nannou::app(model)
        .update(update)
        .simple_window(view)
        .run();
}
