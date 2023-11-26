use super::attribute_data_format::AttributeDataFormat;
use crate::core::enums::AttributeType;

// MeshVertexAttribute
#[derive(Debug, Clone)]
pub struct AttributeMetadata {
    pub attribute_type: AttributeType,
    pub id: u64,
    pub format: AttributeDataFormat,
}

impl AttributeMetadata {
    pub const ATTRIBUTE_POSITION: AttributeMetadata =
        AttributeMetadata::new(AttributeType::Position, 0, AttributeDataFormat::Float32x3);

    pub const ATTRIBUTE_NORMAL: AttributeMetadata =
        AttributeMetadata::new(AttributeType::Normals, 1, AttributeDataFormat::Float32x3);

    pub const ATTRIBUTE_UV_0: AttributeMetadata =
        AttributeMetadata::new(AttributeType::Uv0, 2, AttributeDataFormat::Float32x2);

    pub const ATTRIBUTE_UV_1: AttributeMetadata =
        AttributeMetadata::new(AttributeType::Uv1, 3, AttributeDataFormat::Float32x2);

    pub const ATTRIBUTE_TANGENT: AttributeMetadata =
        AttributeMetadata::new(AttributeType::Tangent, 4, AttributeDataFormat::Float32x4);

    pub const fn new(attribute_type: AttributeType, id: u64, format: AttributeDataFormat) -> Self {
        Self {
            attribute_type,
            id,
            format,
        }
    }
}
