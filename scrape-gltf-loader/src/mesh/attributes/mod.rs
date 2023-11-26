pub mod attribute_data;
pub mod attribute_data_format;
pub mod attribute_metadata;

use attribute_data::AttributeData;
use attribute_metadata::AttributeMetadata;

// MeshVertexData
#[derive(Debug, Clone)]
pub struct Attribute {
    pub metadata: AttributeMetadata,
    pub data: AttributeData,
}
