use rapier3d::na::Quaternion;

pub struct RotationalDirection {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

pub trait IntoDirection {
    fn into_direction(&self) -> RotationalDirection;
}

impl IntoDirection for Quaternion<f32> {
    fn into_direction(&self) -> RotationalDirection {
        let coords = self.coords.xyz();
        RotationalDirection {
            x: coords[0],
            y: coords[1],
            z: coords[2],
        } // todo: re-check
    }
}
