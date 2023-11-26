use gltf::buffer::Data;
use std::collections::HashMap;

use super::core::errors::*;
use super::mesh::attributes::{
    attribute_data::AttributeData, attribute_data_format::AttributeDataFormat,
    attribute_metadata::AttributeMetadata,
};
use super::vertex_iterator::VertexAttributeIter;

enum ConversionMode {
    Any,
    TexCoord,
}

pub fn convert_attribute(
    semantic: gltf::Semantic,
    accessor: gltf::Accessor,
    buffer_data: &Vec<Data>,
    // Might need to add this in later
    _custom_vertex_attributes: Option<&HashMap<String, AttributeMetadata>>,
) -> Result<(AttributeMetadata, AttributeData), ConvertAttributeError> {
    let semantic_result = match &semantic {
        gltf::Semantic::Positions => {
            Some((AttributeMetadata::ATTRIBUTE_POSITION, ConversionMode::Any))
        }
        gltf::Semantic::Normals => Some((AttributeMetadata::ATTRIBUTE_NORMAL, ConversionMode::Any)),
        gltf::Semantic::Tangents => {
            Some((AttributeMetadata::ATTRIBUTE_TANGENT, ConversionMode::Any))
        }
        gltf::Semantic::TexCoords(0) => {
            Some((AttributeMetadata::ATTRIBUTE_UV_0, ConversionMode::TexCoord))
        }
        gltf::Semantic::TexCoords(1) => {
            Some((AttributeMetadata::ATTRIBUTE_UV_1, ConversionMode::TexCoord))
        }
        // gltf::Semantic::Extras(name) => custom_vertex_attributes
        //    .get(name)
        //    .map(|attr| (attr.clone(), ConversionMode::Any)),
        _ => None,
    };

    if let Some((metadata, conversion)) = semantic_result {
        let raw_iter = VertexAttributeIter::from_accessor(accessor.clone(), buffer_data);
        let converted_values = raw_iter.and_then(|iter| match conversion {
            ConversionMode::Any => iter.into_any_values(),
            ConversionMode::TexCoord => iter.into_tex_coord_values(),
        });

        match converted_values {
            Ok(values) => {
                let loaded_format = AttributeDataFormat::from(&values);
                if metadata.format == loaded_format {
                    Ok((metadata, values))
                } else {
                    Err(ConvertAttributeError::WrongFormat(
                        semantic.to_string(),
                        loaded_format,
                        metadata.attribute_type,
                        metadata.format,
                    ))
                }
            }
            Err(err) => Err(ConvertAttributeError::AccessFailed(err, accessor.index())),
        }
    } else {
        return Err(ConvertAttributeError::UnknownName(semantic.to_string())); // TODO: Change
    }
}
