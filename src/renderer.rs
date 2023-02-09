use crate::{
    shapes::{Intersection, Shape},
    lights::{Light, LightRay},
    Image,
    Camera, primitives::{ Ray, Quaternion }
};

pub struct Scene {
    pub objects: Vec<Box<dyn Shape>>,
    pub lights: Vec<Box<dyn Light>>,
    pub background_color: u32
}

impl Scene {
    fn find_closest_shape(&self, ray: &Ray) -> Option<(&dyn Shape, Intersection)> {
        self.objects.iter()
            .map(|s| (s, s.intersect(&ray)))
            .filter_map(|(s, inter)| Some(s.as_ref()).zip(inter))
            .min_by(|(_, inter1), (_, inter2)| inter1.distance.total_cmp(&inter2.distance))
    }

    fn find_light_rays(&self, inter: &Intersection) -> Vec<(LightRay, f32)> {
        let normal = inter.normal.normalize();

        self.lights.iter()
                .map(|l| l.get_ray(inter.position))
                .filter(|ray| is_illuminated(ray, self.objects.as_slice()))
                .map(|r| (r.ray.dir.normalize(), r))
                .map(|(norm, r)| (r, normal.dot(&norm)))
                .filter(|(_, dot)| dot.is_sign_positive())
                .collect()
    }

    fn trace_ray(&self, ray: Ray, max_recursion: u32) -> u32 {
        if let Some((shape, inter)) = self.find_closest_shape(&ray) {
            let rays = self.find_light_rays(&inter);
            let mat = shape.get_material(&inter);

            if max_recursion > 0 {
                let refl_color = if mat.reflection_amount > 0. && mat.refraction_amount < 1. {
                    Some(self.trace_ray(reflect_ray(&ray, &inter), max_recursion - 1))
                } else { None };

            }

            if !rays.is_empty() {
                apply_light(rays.as_slice(), shape, 0xffffff)
            } else {
                self.background_color
            }
        } else {
            self.background_color
        }
    }
}

fn reflect_ray(ray: &Ray, inter: &Intersection) -> Ray {
    Ray {
        orig: inter.position,
        dir: Quaternion::new_rotation(90_f32.to_radians(), ray.dir.cross(&inter.normal)).rotate_vector(ray.dir)
    }
    //l ray.dir.cross(&inter.normal);
}

impl Default for Scene {
    fn default() -> Self {
        Self {
            objects: Vec::new(),
            lights: Vec::new(),
            background_color: 0x000000
        }
    }
}

pub struct Renderer {
    pub camera: Camera,
    pub image: Image,
    pub scene: Scene,
    pub recursion_max: u32,
}

impl Renderer {
    pub fn new(image_x: u32, image_y: u32, camera: Option<Camera>) -> Self {
        Renderer {
            image: Image::new(image_x, image_y, None),
            camera: camera.unwrap_or_default(),
            scene: Default::default(),
            recursion_max: 0,
        }
    }

    fn render_pixel(&mut self, x: u32, y: u32) {
        let ray = self.camera.get_ray(self.image.get_dimension(), x, y);
        let color = self.scene.trace_ray(ray, self.recursion_max);

        self.image.draw_pixel(x, y, color)
    }

    pub fn render(&mut self) {
        for x in 0..self.image.x {
            for y in 0..self.image.y {
                self.render_pixel(x, y);
            }
        }
    }
}

fn apply_light(rays: &[(LightRay, f32)], shape: &dyn Shape, color: u32) -> u32 {
    let intensity = light_intensity(rays);
    let col = color.to_be_bytes()
        .into_iter()
        .zip(intensity)
        .map(|(b, l)| ((b as f32 * l) as u8).clamp(0, 255))
        .collect::<Vec<u8>>();

    u32::from_be_bytes([col[0], col[1], col[2], col[3]])
}

/// Check if an intersection point is illuminated by a LightRay
///
/// This function flip the Ray component of the LightRay,
/// then check whether another shape exists
/// between the intersection and the light origin.
fn is_illuminated(lray: &LightRay, objects: &[Box<dyn Shape>]) -> bool {
    let ray = &lray.ray;

    !objects.iter()
            .filter_map(|s| s.intersect(ray))
            .any(|i| i.distance > 0. && i.distance < 1.)
}

fn light_color(shape: &dyn Shape, rays: &[(LightRay, f32)]) -> [f32; 4] {
    fn compute_color(_shape: &dyn Shape, ray: &LightRay, dot: f32) -> [f32; 4] {
        let mut color: [f32; 4] = [0., 0., 0., 0.];
        let col = ray.color.to_be_bytes();

        color[0] = 255.;
        for idx in 1..4 {
            color[idx] = dot * ray.intensity * col[idx] as f32;
        }

        color
    }

    rays.iter()
        .map(|(r, dot)| compute_color(shape, r, *dot))
        .reduce(|acc, e| [acc[0] + e[0], acc[1] + e[1], acc[2] + e[2], acc[3] + e[3]])
        .unwrap_or([0., 0., 0., 0.])
        .map(|c| c.clamp(0., 255.))
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

