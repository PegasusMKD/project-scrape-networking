// Values
#[derive(Debug, Clone)]
pub enum AttributeData {
    Float32(Vec<f32>),
    // Sint32(Vec<i32>),
    Uint32(Vec<u32>),
    Float32x2(Vec<[f32; 2]>),
    // Sint32x2(Vec<[i32; 2]>),
    Uint32x2(Vec<[u32; 2]>),
    Float32x3(Vec<[f32; 3]>),
    // Sint32x3(Vec<[i32; 3]>),
    Uint32x3(Vec<[u32; 3]>),
    Float32x4(Vec<[f32; 4]>),
    // Sint32x4(Vec<[i32; 4]>),
    Uint32x4(Vec<[u32; 4]>),
    Sint16x2(Vec<[i16; 2]>),
    Snorm16x2(Vec<[i16; 2]>),
    Uint16x2(Vec<[u16; 2]>),
    Unorm16x2(Vec<[u16; 2]>),
    Sint16x4(Vec<[i16; 4]>),
    Snorm16x4(Vec<[i16; 4]>),
    Uint16x4(Vec<[u16; 4]>),
    Unorm16x4(Vec<[u16; 4]>),
    Sint8x2(Vec<[i8; 2]>),
    Snorm8x2(Vec<[i8; 2]>),
    Uint8x2(Vec<[u8; 2]>),
    Unorm8x2(Vec<[u8; 2]>),
    Sint8x4(Vec<[i8; 4]>),
    Snorm8x4(Vec<[i8; 4]>),
    Uint8x4(Vec<[u8; 4]>),
    Unorm8x4(Vec<[u8; 4]>),
}

impl AttributeData {
    /// Returns the number of vertices in this [`AttributeData`]. For a single
    /// mesh, all of the [`AttributeData`] must have the same length.
    #[allow(clippy::match_same_arms)]
    pub fn len(&self) -> usize {
        match self {
            AttributeData::Float32(values) => values.len(),
            // AttributeData::Sint32(values) => values.len(),
            AttributeData::Uint32(values) => values.len(),
            AttributeData::Float32x2(values) => values.len(),
            // AttributeData::Sint32x2(values) => values.len(),
            AttributeData::Uint32x2(values) => values.len(),
            AttributeData::Float32x3(values) => values.len(),
            // AttributeData::Sint32x3(values) => values.len(),
            AttributeData::Uint32x3(values) => values.len(),
            AttributeData::Float32x4(values) => values.len(),
            // AttributeData::Sint32x4(values) => values.len(),
            AttributeData::Uint32x4(values) => values.len(),
            AttributeData::Sint16x2(values) => values.len(),
            AttributeData::Snorm16x2(values) => values.len(),
            AttributeData::Uint16x2(values) => values.len(),
            AttributeData::Unorm16x2(values) => values.len(),
            AttributeData::Sint16x4(values) => values.len(),
            AttributeData::Snorm16x4(values) => values.len(),
            AttributeData::Uint16x4(values) => values.len(),
            AttributeData::Unorm16x4(values) => values.len(),
            AttributeData::Sint8x2(values) => values.len(),
            AttributeData::Snorm8x2(values) => values.len(),
            AttributeData::Uint8x2(values) => values.len(),
            AttributeData::Unorm8x2(values) => values.len(),
            AttributeData::Sint8x4(values) => values.len(),
            AttributeData::Snorm8x4(values) => values.len(),
            AttributeData::Uint8x4(values) => values.len(),
            AttributeData::Unorm8x4(values) => values.len(),
        }
    }

    /// Returns the values as float triples if possible.
    pub fn as_float3(&self) -> Option<&[[f32; 3]]> {
        match self {
            AttributeData::Float32x3(values) => Some(values),
            _ => None,
        }
    }
}
