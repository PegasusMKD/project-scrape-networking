use bevy_math::Vec3;
use gltf::mesh::Mode;

use super::enums::PrimitiveTopology;

pub fn face_normal(a: [f32; 3], b: [f32; 3], c: [f32; 3]) -> [f32; 3] {
    let (a, b, c) = (Vec3::from(a), Vec3::from(b), Vec3::from(c));
    (b - a).cross(c - a).normalize().into()
}

/// Maps the `primitive_topology` form glTF to `wgpu`.
pub fn get_primitive_topology(mode: Mode) -> Result<PrimitiveTopology, ()> {
    match mode {
        Mode::Points => Ok(PrimitiveTopology::PointList),
        Mode::Lines => Ok(PrimitiveTopology::LineList),
        Mode::LineStrip => Ok(PrimitiveTopology::LineStrip),
        Mode::Triangles => Ok(PrimitiveTopology::TriangleList),
        Mode::TriangleStrip => Ok(PrimitiveTopology::TriangleStrip),
        _mode => Err(()),
    }
}
