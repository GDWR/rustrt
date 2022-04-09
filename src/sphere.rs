use crate::hit_record::HitRecord;
use crate::Ray;
use crate::vec3::Vec3;

#[derive(Clone, Copy, Debug, Default)]
pub struct Sphere {
    pub pos: Vec3,
    pub colour: Vec3,
    pub radius: f32,
}

impl Sphere {
    pub fn hit(self, ray: Ray) -> Option<HitRecord> {
        let oc = ray.pos - self.pos;

        let a = ray.dir.dot(ray.dir);
        let b = 2. * (oc.dot(ray.dir));
        let c = oc.dot(oc) - self.radius * self.radius;

        let discriminant = b * b - 4. * a * c;
        if discriminant <= 0. {
            return None;
        }

        let mut temp = (-b - discriminant.sqrt()) / (2. * a);

        if temp < 1e-3 {
            temp = (-b + discriminant.sqrt()) / (2. * a);
        }
        if temp < 1e-3 {
            return None;
        }

        let pos = ray.at(temp);

        Some(HitRecord {
            t: temp,
            pos,
            normal: (pos - self.pos).normalize(),
            obj: Box::new(self),
        })
    }
}