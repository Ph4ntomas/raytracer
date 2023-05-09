use nannou::{image::DynamicImage, prelude::*};

mod bin;
use bin::Model;
use raytracer::maths::Point;
use raytracer::scene::objects::Sphere;
use raytracer::scene::*;

fn update(_: &App, _model: &mut Model, _update: Update) {}

fn model(app: &App) -> Model {
    let scene = scene()
        .with_object(Sphere::new(Point::new(0., 0., 100.), 50.))
        .build();

    app.new_window()
        .size(512, 512)
        .resizable(false)
        .view(view)
        .build()
        .unwrap();

    Model {
        scene,
        ..Default::default()
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let (tx, ty) = app.main_window().inner_size_points();
    let tx = tx as u32;
    let ty = ty as u32;
    //let [tx, ty] = frame.texture_size();

    let mut img = DynamicImage::new_rgba8(tx, ty);

    {
        let rgba_img = img.as_mut_rgba8().unwrap();

        for y in 0..ty {
            for x in 0..tx {
                let color = model
                    .renderer
                    .render_pixel(x, y, &model.camera, &model.scene);
                rgba_img.put_pixel(x, y, color.to_array().into());
            }
        }
    }

    let texture = wgpu::Texture::from_image(app, &img);
    let draw = app.draw();
    draw.texture(&texture);
    draw.to_frame(app, &frame).unwrap();
}

fn main() {
    nannou::app(model).update(update).run();
}
