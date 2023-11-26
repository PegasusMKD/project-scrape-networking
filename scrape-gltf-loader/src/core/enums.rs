#[repr(C)]
#[derive(Debug, Clone)]
pub enum PrimitiveTopology {
    PointList = 0,
    LineList = 1,
    LineStrip = 2,
    TriangleList = 3,
    TriangleStrip = 4,
}

#[repr(C)]
#[derive(Debug, Clone, PartialEq, PartialOrd, Ord, Eq)]
pub enum AttributeType {
    Position = 0,
    Normals = 1,
    Uv0 = 2,
    Uv1 = 3,
    Tangent = 4,
}
