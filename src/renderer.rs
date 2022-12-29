use crate::{
    shapes,
    lights,
    Image,
    Camera
};


pub struct Renderer {
    pub camera: Camera,
    pub image: Image,
    pub objects: Vec<Box<dyn shapes::Shape>>,
    pub lights: Vec<Box<dyn lights::Light>>
}

impl Renderer {
    pub fn new(image_x: u32, image_y: u32, camera: Option<Camera>) -> Self {
        Renderer {
            image: Image::new(image_x, image_y, None),
            camera: camera.unwrap_or_default(),
            objects: Vec::new(),
            lights: Vec::new()
        }
    }

    fn render_pixel(&mut self, x: u32, y: u32) {
        let ray = self.camera.get_ray(self.image.get_dimension(), x, y);

        let closest = self.objects.as_slice().into_iter()
            .map(|s| (s, s.intersect(&ray)))
            .filter(|(_, inter)| inter.is_some())
            .map(|(s, inter)| (s, inter.unwrap()))
            .min_by(|(_, inter1), (_, inter2)| inter1.distance.total_cmp(&inter2.distance));

        if let Some(_shape) = closest {
            self.image.draw_pixel(x, y, 0xffffb7b7);
        } else {
            self.image.draw_pixel(x, y, 0x000000)
        }
    }

    pub fn render(&mut self) {
        for x in 0..self.image.x {
            for y in 0..self.image.y {
                self.render_pixel(x, y);
            }

            println!("Computing line {x} Done");
        }

        println!("Rendered {}x{} pixels.", self.image.x, self.image.y);
    }
}
