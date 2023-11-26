use gltf::{
    accessor::{DataType, Dimensions},
    buffer::Data,
    mesh::util::ReadTexCoords,
};

pub mod buffer_accessor;
pub mod normalization;

use crate::core::errors::*;
use crate::mesh::attributes::attribute_data::AttributeData;

use buffer_accessor::BufferAccessor;
use normalization::Normalization;

/// An enum of the iterators user by different vertex attribute formats
pub enum VertexAttributeIter<'a> {
    // For reading native WGPU formats
    F32(gltf::accessor::Iter<'a, f32>),
    U32(gltf::accessor::Iter<'a, u32>),
    F32x2(gltf::accessor::Iter<'a, [f32; 2]>),
    U32x2(gltf::accessor::Iter<'a, [u32; 2]>),
    F32x3(gltf::accessor::Iter<'a, [f32; 3]>),
    U32x3(gltf::accessor::Iter<'a, [u32; 3]>),
    F32x4(gltf::accessor::Iter<'a, [f32; 4]>),
    U32x4(gltf::accessor::Iter<'a, [u32; 4]>),
    S16x2(gltf::accessor::Iter<'a, [i16; 2]>, Normalization),
    U16x2(gltf::accessor::Iter<'a, [u16; 2]>, Normalization),
    S16x4(gltf::accessor::Iter<'a, [i16; 4]>, Normalization),
    U16x4(gltf::accessor::Iter<'a, [u16; 4]>, Normalization),
    S8x2(gltf::accessor::Iter<'a, [i8; 2]>, Normalization),
    U8x2(gltf::accessor::Iter<'a, [u8; 2]>, Normalization),
    S8x4(gltf::accessor::Iter<'a, [i8; 4]>, Normalization),
    U8x4(gltf::accessor::Iter<'a, [u8; 4]>, Normalization),
    // Additional on-disk formats used for RGB colors
    U16x3(gltf::accessor::Iter<'a, [u16; 3]>, Normalization),
    U8x3(gltf::accessor::Iter<'a, [u8; 3]>, Normalization),
}

impl<'a> VertexAttributeIter<'a> {
    /// Creates an iterator over the elements in a vertex attribute accessor
    pub fn from_accessor(
        accessor: gltf::Accessor<'a>,
        buffer_data: &'a Vec<Data>,
    ) -> Result<VertexAttributeIter<'a>, AccessFailed> {
        let normalization = Normalization(accessor.normalized());
        let format = (accessor.data_type(), accessor.dimensions());
        let acc = BufferAccessor {
            accessor,
            buffer_data,
            normalization,
        };
        match format {
            (DataType::F32, Dimensions::Scalar) => acc.with_no_norm(VertexAttributeIter::F32),
            (DataType::U32, Dimensions::Scalar) => acc.with_no_norm(VertexAttributeIter::U32),
            (DataType::F32, Dimensions::Vec2) => acc.with_no_norm(VertexAttributeIter::F32x2),
            (DataType::U32, Dimensions::Vec2) => acc.with_no_norm(VertexAttributeIter::U32x2),
            (DataType::F32, Dimensions::Vec3) => acc.with_no_norm(VertexAttributeIter::F32x3),
            (DataType::U32, Dimensions::Vec3) => acc.with_no_norm(VertexAttributeIter::U32x3),
            (DataType::F32, Dimensions::Vec4) => acc.with_no_norm(VertexAttributeIter::F32x4),
            (DataType::U32, Dimensions::Vec4) => acc.with_no_norm(VertexAttributeIter::U32x4),
            (DataType::I16, Dimensions::Vec2) => acc.with_norm(VertexAttributeIter::S16x2),
            (DataType::U16, Dimensions::Vec2) => acc.with_norm(VertexAttributeIter::U16x2),
            (DataType::I16, Dimensions::Vec4) => acc.with_norm(VertexAttributeIter::S16x4),
            (DataType::U16, Dimensions::Vec4) => acc.with_norm(VertexAttributeIter::U16x4),
            (DataType::I8, Dimensions::Vec2) => acc.with_norm(VertexAttributeIter::S8x2),
            (DataType::U8, Dimensions::Vec2) => acc.with_norm(VertexAttributeIter::U8x2),
            (DataType::I8, Dimensions::Vec4) => acc.with_norm(VertexAttributeIter::S8x4),
            (DataType::U8, Dimensions::Vec4) => acc.with_norm(VertexAttributeIter::U8x4),
            (DataType::U16, Dimensions::Vec3) => acc.with_norm(VertexAttributeIter::U16x3),
            (DataType::U8, Dimensions::Vec3) => acc.with_norm(VertexAttributeIter::U8x3),
            _ => Err(AccessFailed::UnsupportedFormat),
        }
    }

    /// Materializes values for any supported format of vertex attribute
    pub fn into_any_values(self) -> Result<AttributeData, AccessFailed> {
        match self {
            VertexAttributeIter::F32(it) => Ok(AttributeData::Float32(it.collect())),
            VertexAttributeIter::U32(it) => Ok(AttributeData::Uint32(it.collect())),
            VertexAttributeIter::F32x2(it) => Ok(AttributeData::Float32x2(it.collect())),
            VertexAttributeIter::U32x2(it) => Ok(AttributeData::Uint32x2(it.collect())),
            VertexAttributeIter::F32x3(it) => Ok(AttributeData::Float32x3(it.collect())),
            VertexAttributeIter::U32x3(it) => Ok(AttributeData::Uint32x3(it.collect())),
            VertexAttributeIter::F32x4(it) => Ok(AttributeData::Float32x4(it.collect())),
            VertexAttributeIter::U32x4(it) => Ok(AttributeData::Uint32x4(it.collect())),
            VertexAttributeIter::S16x2(it, n) => Ok(n.apply_either(
                it.collect(),
                AttributeData::Snorm16x2,
                AttributeData::Sint16x2,
            )),
            VertexAttributeIter::U16x2(it, n) => Ok(n.apply_either(
                it.collect(),
                AttributeData::Unorm16x2,
                AttributeData::Uint16x2,
            )),
            VertexAttributeIter::S16x4(it, n) => Ok(n.apply_either(
                it.collect(),
                AttributeData::Snorm16x4,
                AttributeData::Sint16x4,
            )),
            VertexAttributeIter::U16x4(it, n) => Ok(n.apply_either(
                it.collect(),
                AttributeData::Unorm16x4,
                AttributeData::Uint16x4,
            )),
            VertexAttributeIter::S8x2(it, n) => Ok(n.apply_either(
                it.collect(),
                AttributeData::Snorm8x2,
                AttributeData::Sint8x2,
            )),
            VertexAttributeIter::U8x2(it, n) => Ok(n.apply_either(
                it.collect(),
                AttributeData::Unorm8x2,
                AttributeData::Uint8x2,
            )),
            VertexAttributeIter::S8x4(it, n) => Ok(n.apply_either(
                it.collect(),
                AttributeData::Snorm8x4,
                AttributeData::Sint8x4,
            )),
            VertexAttributeIter::U8x4(it, n) => Ok(n.apply_either(
                it.collect(),
                AttributeData::Unorm8x4,
                AttributeData::Uint8x4,
            )),
            _ => Err(AccessFailed::UnsupportedFormat),
        }
    }

    /// Materializes texture coordinate values, converting compatible formats to Float32x2
    pub fn into_tex_coord_values(self) -> Result<AttributeData, AccessFailed> {
        match self {
            VertexAttributeIter::U8x2(it, Normalization(true)) => Ok(AttributeData::Float32x2(
                ReadTexCoords::U8(it).into_f32().collect(),
            )),
            VertexAttributeIter::U16x2(it, Normalization(true)) => Ok(AttributeData::Float32x2(
                ReadTexCoords::U16(it).into_f32().collect(),
            )),
            s => s.into_any_values(),
        }
    }
}
