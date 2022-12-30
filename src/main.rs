use raytracer::Renderer;
use raytracer::lights::DotLight;
use raytracer::primitives::Point;

use sfml::window::{ Event, Style };
use sfml::graphics::{RenderWindow, RenderTarget, Image, Color, Sprite, Texture};

const XSCREEN: u32 = 256;
const YSCREEN: u32 = 256;

use raytracer::shapes::Sphere;

fn setup_renderer() -> Renderer {
    let mut renderer = raytracer::Renderer::new(XSCREEN, YSCREEN, None);

    renderer.objects.push(Box::new(Sphere::new (
        Point::new(0., 0., 400.),
        150.
    )));

    renderer.objects.push(Box::new(Sphere::new (
        Point::new(100., 200., 300.),
        20.
    )));

    renderer.lights.push(Box::new(DotLight {
        pos: Point::new(500., 500., 0.),
        color: 0x0000ff,
        ..Default::default()
    }));

    renderer.lights.push(Box::new(DotLight {
        pos: Point::new(200., 0., 0.),
        color: 0xffff00,
        ..Default::default()
    }));

    renderer.render();

    renderer

}

fn main() {
    let mut win = RenderWindow::new((XSCREEN, YSCREEN), "Raytracer", Style::CLOSE, &Default::default());
    let mut texture = Texture::new().unwrap();
    let renderer = setup_renderer();

    win.set_framerate_limit(60);

    while win.is_open() {
        while let Some(event) = win.poll_event() {
            match event {
                Event::Closed => win.close(),
                _ => {}
            }
        }

        win.clear(Color::BLACK);

        unsafe {
            let img = Image::create_from_pixels(XSCREEN, YSCREEN, renderer.image.get_data()).unwrap();
            texture.load_from_image(&img, Default::default()).expect("Couldn't create image");
        }
        let sprite = Sprite::with_texture(&texture);

        win.draw(&sprite);

        win.display();
    }
}
