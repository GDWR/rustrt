use crate::Ray;
use crate::hit_record::HitRecord;
use crate::sphere::Sphere;

#[derive(Default)]
pub struct Scene {
    pub objs: Vec<Sphere>,
}

impl Scene {
    pub fn add(self: &mut Self, sphere: Sphere) {
        self.objs.push(sphere)
    }

    pub fn hit(&self, ray: Ray) -> Option<HitRecord> {
        self.objs.iter()
            .map(|o| o.hit(ray))
            .flatten()
            .reduce(|closest, hit| if closest.t < hit.t { closest } else { hit })
    }
}