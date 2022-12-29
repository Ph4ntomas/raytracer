use raytracer::primitives::Point;

use sfml::window::{ Event, Style };
use sfml::graphics::{RenderWindow, RenderTarget, Image, Color, Sprite, Texture};

const XSCREEN: u32 = 256;
const YSCREEN: u32 = 256;

use raytracer::shapes::Sphere;

fn main() {
    let mut win = RenderWindow::new((XSCREEN, YSCREEN), "Raytracer", Style::CLOSE, &Default::default());
    let mut renderer = raytracer::Renderer::new(XSCREEN, YSCREEN, None);

    let mut texture = Texture::new().unwrap();

    win.set_framerate_limit(60);

    renderer.objects.push(Box::new(Sphere{
        position: Point::new(0., 0., 400.),
        radius: 50.
    }));
    renderer.render();

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
