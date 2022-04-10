use rand::random;
use std::f32::consts::PI;

use crate::vec3::Vec3;
use crate::Ray;

#[derive(Copy, Clone)]
pub struct Camera {
    pos: Vec3,
    dir: Vec3,
    fov: f32,
    width: usize,
    height: usize,
    corner: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
    u: Vec3,
    v: Vec3,
    w: Vec3,
}

impl Camera {
    pub fn create(pos: Vec3, dir: Vec3, up: Vec3, fov: f32, width: usize, height: usize) -> Camera {
        let w = -dir;
        let u = up.cross(w);
        let v = w.cross(u);

        let aspect = width as f32 / height as f32;
        let theta = fov * PI / 180.;
        let y_size = 2. * (theta.tan() / 2.);
        let x_size = y_size * aspect;

        let horizontal = u * -x_size;
        let vertical = v * y_size;
        let corner = pos - horizontal / 2. - vertical / 2. - w;

        Camera {
            pos,
            dir,
            fov,
            w,
            u,
            v,
            width,
            height,
            horizontal,
            vertical,
            corner,
        }
    }

    pub fn get_ray(self, x: usize, y: usize) -> Ray {
        let dx: f32 = (random::<f32>() + x as f32) / self.width as f32;
        let dy: f32 = (random::<f32>() + y as f32) / self.height as f32;

        Ray::new(
            self.pos,
            (self.corner + (dx * self.horizontal) + (dy * self.vertical) - self.pos).normalize(),
        )
    }
}
