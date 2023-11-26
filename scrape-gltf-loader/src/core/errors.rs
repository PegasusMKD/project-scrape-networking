use thiserror::Error;

use crate::core::enums::AttributeType;
use crate::mesh::attributes::attribute_data_format::AttributeDataFormat;

#[derive(Error, Debug)]
pub enum ConvertAttributeError {
    #[error("Vertex attribute {0} has format {1:?} but expected {3:?} for target attribute {2:?}")]
    WrongFormat(
        String,
        AttributeDataFormat,
        AttributeType,
        AttributeDataFormat,
    ),
    #[error("{0} in accessor {1}")]
    AccessFailed(AccessFailed, usize),
    #[error("Unknown vertex attribute {0}")]
    UnknownName(String),
}

/// An error that occurs when accessing buffer data
#[derive(Error, Debug)]
pub enum AccessFailed {
    #[error("Malformed vertex attribute data")]
    MalformedData,
    #[error("Unsupported vertex attribute format")]
    UnsupportedFormat,
}
