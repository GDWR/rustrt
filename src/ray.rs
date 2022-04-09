use crate::vec3::Vec3;


#[derive(Debug, Default, Clone, Copy)]
pub struct Ray {
    pub pos: Vec3,
    pub dir: Vec3,
}


impl Ray {
    #[inline]
    pub fn new(pos: Vec3, dir: Vec3) -> Ray {
        Ray { pos, dir }
    }

    #[inline]
    pub fn at(self, t: f32) -> Vec3 {
        self.pos + t * self.dir
    }
}