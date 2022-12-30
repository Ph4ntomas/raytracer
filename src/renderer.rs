use crate::{
    shapes::{Intersection, Shape},
    lights::{Light, LightRay},
    Image,
    Camera, primitives::Ray
};

pub struct Renderer {
    pub camera: Camera,
    pub image: Image,
    pub objects: Vec<Box<dyn Shape>>,
    pub lights: Vec<Box<dyn Light>>
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

    fn find_closest_shape(&self, ray: Ray) -> Option<(&dyn Shape, Intersection)> {
        self.objects.iter()
            .map(|s| (s, s.intersect(&ray)))
            .filter_map(|(s, inter)| Some(s.as_ref()).zip(inter))
            .min_by(|(_, inter1), (_, inter2)| inter1.distance.total_cmp(&inter2.distance))
    }

    fn find_light_rays(&self, inter: Intersection) -> Vec<(LightRay, f32)> {
        let normal = inter.normal.normalize();

        self.lights.iter()
                .map(|l| l.get_ray(inter.position))
                .filter(|ray| is_illuminated(&inter, ray, self.objects.as_slice()))
                .map(|r| ((r.ray.orig - inter.position).normalize(), r))
                .map(|(norm, r)| (r, normal.dot(&norm)))
                .filter(|(_, dot)| dot.is_sign_positive())
                .collect()
    }

    fn render_pixel(&mut self, x: u32, y: u32) {
        let ray = self.camera.get_ray(self.image.get_dimension(), x, y);
        let closest = self.find_closest_shape(ray);

        if let Some((shape, inter)) = closest {
            let rays : Vec<(LightRay, f32)> = self.find_light_rays(inter);
            if !rays.is_empty() {
                let color = apply_light(rays.as_slice(), shape);
                self.image.draw_pixel(x, y, color);
            } else {
                self.image.draw_pixel(x, y, 0x00);
            }
        } else {
            self.image.draw_pixel(x, y, 0x000000)
        }
    }

    pub fn render(&mut self) {
        for x in 0..self.image.x {
            for y in 0..self.image.y {
                self.render_pixel(x, y);
            }
        }
    }
}

fn apply_light(rays: &[(LightRay, f32)], shape: &dyn Shape) -> u32 {
    let lum = light_intensity(rays);
    let col = light_color(shape, rays);

    let col = col.into_iter()
        .zip(lum)
        .map(|(b, l)| ((b * l) as u8).clamp(0, 255))
        .collect::<Vec<u8>>();

    u32::from_be_bytes([col[0], col[1], col[2], col[3]])
}

/// Check if an intersection point is illuminated by a LightRay
///
/// This function flip the Ray component of the LightRay,
/// then check whether another shape exists
/// between the intersection and the light origin.
fn is_illuminated(inter: &Intersection, lray: &LightRay, objects: &[Box<dyn Shape>]) -> bool {
    let ray = Ray {
        orig: inter.position,
        dir: lray.ray.dir * -1.
    };
    !objects.iter()
            .filter_map(|s| s.intersect(&ray))
            .any(|i| i.distance > 0. && i.distance < 1.)
}

fn light_color(shape: &dyn Shape, rays: &[(LightRay, f32)]) -> [f32; 4] {
    fn compute_color(_shape: &dyn Shape, ray: &LightRay, dot: f32) -> [f32; 4] {
        let mut color: [f32; 4] = [0., 0., 0., 0.];
        let col = ray.color.to_be_bytes();

        color[0] = 256.;
        for idx in 1..4 {
            color[idx] = dot * ray.intensity * col[idx] as f32;
        }

        color
    }

    rays.iter()
        .map(|(r, dot)| compute_color(shape, r, *dot))
        .reduce(|acc, e| [acc[0] + e[0], acc[1] + e[1], acc[2] + e[2], acc[3] + e[3]])
        .unwrap_or([0., 0., 0., 0.])
}

fn light_intensity(rays: &[(LightRay, f32)]) -> [f32; 4] {
    fn compute_luminosity(ray: &LightRay, dot: f32) -> [f32; 4] {
        let mut lum : [f32; 4] = [0., 0., 0., 0.];
        let col = ray.color.to_be_bytes();

        lum[0] = 1.;
        for idx in 1..4 {
            lum[idx] = dot * ray.intensity * col[idx] as f32/255.;
        }

        lum
    }

    rays.iter()
        .map(|(r, dot)| compute_luminosity(r, *dot))
        .reduce(|acc, e| [acc[0] + e[0], acc[1] + e[1], acc[2] + e[2], acc[3] + e[3]])
        .unwrap_or([0., 0., 0., 0.])
        .map(|l| l.clamp(0., 1.))
}

