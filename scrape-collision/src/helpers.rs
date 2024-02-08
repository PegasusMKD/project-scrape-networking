use rapier3d::na::Quaternion;

pub struct RotationalDirection {
    x: f32,
    y: f32,
    z: f32,
}

pub trait IntoDirection {
    fn into_direction(&self) -> RotationalDirection;
}


impl IntoDirection for Quaternion<f32> {

    fn into_direction(&self) -> RotationalDirection {
        RotationalDirection { x: self.coords.x, y: self.coords.y, z: self.coords.z } // todo: re-check
    }
}
