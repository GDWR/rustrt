use crate::hit_record::HitRecord;
use crate::sphere::Sphere;
use crate::Ray;

#[derive(Default)]
pub struct Scene {
    pub objs: Vec<Sphere>,
}

impl Scene {
    pub fn add(&mut self, sphere: Sphere) {
        self.objs.push(sphere)
    }

    pub fn hit(&self, ray: Ray) -> Option<HitRecord> {
        self.objs
            .iter()
            .filter_map(|o| o.hit(ray))
            .reduce(|closest, hit| if closest.t < hit.t { closest } else { hit })
    }
}
