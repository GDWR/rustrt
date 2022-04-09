use rand::Rng;

use hit_record::HitRecord;

use crate::camera::Camera;
use crate::image::Image;
use crate::ray::Ray;
use crate::scene::Scene;
use crate::sphere::Sphere;
use crate::vec3::Vec3;

mod image;
mod camera;
mod vec3;
mod scene;
mod sphere;
mod ray;
mod hit_record;

const WIDTH: usize = 1920;
const HEIGHT: usize = 1080;
const FOV: f32 = 40.;
const MAX_DEPTH: usize = 40;
const NUM_SAMPLES: usize = 300;
const SKY_COLOUR: Vec3 = Vec3(173. / 255., 273. / 255., 255. / 255.);

fn raytrace(scene: &Scene, mut ray: Ray) -> Vec3 {
    let mut result = Vec3::ones();

    for _ in 0..MAX_DEPTH {
        match scene.hit(ray) {
            Some(record) => {
                result *= record.obj.colour;
                ray.pos = record.pos;
                ray.dir = Vec3::random();
            }
            None => return SKY_COLOUR * result
        }
    }

    return result;
}

fn main() {
    let mut img = Image::create(WIDTH, HEIGHT);
    let camera = Camera::create(
        Vec3::zeros(),
        Vec3::new(0., 0., 1.),
        Vec3::new(0., 1., 0.),
        FOV,
        WIDTH,
        HEIGHT,
    );

    let scene = &mut Scene::default();
    scene.add(
        Sphere {
            pos: Vec3::new(0., -100.5, 1.),
            radius: 100.,
            colour: Vec3::new(1., 0.6, 0.3),
        }
    );
    scene.add(
        Sphere {
            pos: Vec3::new(-0.6, 0., 1.),
            radius: 0.5,
            colour: Vec3::new(0.2, 0.2, 1.),
        }
    );
    scene.add(
        Sphere {
            pos: Vec3::new(0.6, 0., 1.),
            radius: 0.5,
            colour: Vec3::new(1., 0.2, 0.2),
        }
    );

    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            let mut colour = Vec3::zeros();

            for _ in 0..NUM_SAMPLES {
                let ray = camera.get_ray(x, HEIGHT - y - 1);
                colour += raytrace(scene, ray);
            }

            img.set_pixel(x, y, colour / NUM_SAMPLES as f32);
        }
    }

    img.save("test.ppm");
}
