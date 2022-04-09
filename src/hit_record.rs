use crate::{Sphere, Vec3};

#[derive(Debug, Default)]
pub struct HitRecord {
    pub t: f32,
    pub pos: Vec3,
    pub normal: Vec3,
    pub obj: Box<Sphere>,
}
