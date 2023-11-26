use std::collections::BTreeMap;

use crate::core::{enums::PrimitiveTopology, utility::face_normal};
use crate::indices::IndexVec;
use crate::mesh::attributes::{
    attribute_data::AttributeData, attribute_metadata::AttributeMetadata, Attribute,
};

#[derive(Debug, Clone)]
pub struct MeshPrimitive {
    pub topology: PrimitiveTopology,
    pub attributes: BTreeMap<u64, Attribute>,
    pub indices: Option<IndexVec>,
}

impl MeshPrimitive {
    pub fn new(topology: PrimitiveTopology) -> Self {
        Self {
            topology,
            attributes: BTreeMap::new(),
            indices: None,
        }
    }

    #[inline]
    pub fn insert_attribute(
        &mut self,
        metadata: AttributeMetadata,
        data: impl Into<AttributeData>,
    ) {
        self.attributes.insert(
            metadata.id,
            Attribute {
                metadata,
                data: data.into(),
            },
        );
    }

    #[inline]
    pub fn set_indices(&mut self, indices: Option<IndexVec>) {
        self.indices = indices;
    }

    /// Retrieves the data currently set to the vertex attribute with the specified `name`.
    #[inline]
    pub fn attribute(&self, metadata: impl Into<AttributeMetadata>) -> Option<&AttributeData> {
        self.attributes
            .get(&metadata.into().id)
            .map(|data| &data.data)
    }

    /// Counts all vertices of the mesh.
    ///
    /// If the attributes have different vertex counts, the smallest is returned.
    pub fn count_vertices(&self) -> usize {
        let mut vertex_count: Option<usize> = None;
        for (_, attribute) in &self.attributes {
            let attribute_len = attribute.data.len();
            if let Some(previous_vertex_count) = vertex_count {
                if previous_vertex_count != attribute_len {
                    vertex_count = Some(std::cmp::min(previous_vertex_count, attribute_len));
                }
            } else {
                vertex_count = Some(attribute_len);
            }
        }

        vertex_count.unwrap_or(0)
    }

    /// Duplicates the vertex attributes so that no vertices are shared.
    ///
    /// This can dramatically increase the vertex count, so make sure this is what you want.
    /// Does nothing if no [IndexVec] are set.
    #[allow(clippy::match_same_arms)]
    pub fn duplicate_vertices(&mut self) {
        fn duplicate<T: Copy>(values: &[T], indices: impl Iterator<Item = usize>) -> Vec<T> {
            indices.map(|i| values[i]).collect()
        }

        let indices = match self.indices.take() {
            Some(indices) => indices,
            None => return,
        };

        for attributes in self.attributes.values_mut() {
            let indices = indices.iter();
            match &mut attributes.data {
                AttributeData::Float32(vec) => *vec = duplicate(vec, indices),
                // AttributeData::Sint32(vec) => *vec = duplicate(vec, indices),
                AttributeData::Uint32(vec) => *vec = duplicate(vec, indices),
                AttributeData::Float32x2(vec) => *vec = duplicate(vec, indices),
                // AttributeData::Sint32x2(vec) => *vec = duplicate(vec, indices),
                AttributeData::Uint32x2(vec) => *vec = duplicate(vec, indices),
                AttributeData::Float32x3(vec) => *vec = duplicate(vec, indices),
                // AttributeData::Sint32x3(vec) => *vec = duplicate(vec, indices),
                AttributeData::Uint32x3(vec) => *vec = duplicate(vec, indices),
                //AttributeData::Sint32x4(vec) => *vec = duplicate(vec, indices),
                AttributeData::Uint32x4(vec) => *vec = duplicate(vec, indices),
                AttributeData::Float32x4(vec) => *vec = duplicate(vec, indices),
                AttributeData::Sint16x2(vec) => *vec = duplicate(vec, indices),
                AttributeData::Snorm16x2(vec) => *vec = duplicate(vec, indices),
                AttributeData::Uint16x2(vec) => *vec = duplicate(vec, indices),
                AttributeData::Unorm16x2(vec) => *vec = duplicate(vec, indices),
                AttributeData::Sint16x4(vec) => *vec = duplicate(vec, indices),
                AttributeData::Snorm16x4(vec) => *vec = duplicate(vec, indices),
                AttributeData::Uint16x4(vec) => *vec = duplicate(vec, indices),
                AttributeData::Unorm16x4(vec) => *vec = duplicate(vec, indices),
                AttributeData::Sint8x2(vec) => *vec = duplicate(vec, indices),
                AttributeData::Snorm8x2(vec) => *vec = duplicate(vec, indices),
                AttributeData::Uint8x2(vec) => *vec = duplicate(vec, indices),
                AttributeData::Unorm8x2(vec) => *vec = duplicate(vec, indices),
                AttributeData::Sint8x4(vec) => *vec = duplicate(vec, indices),
                AttributeData::Snorm8x4(vec) => *vec = duplicate(vec, indices),
                AttributeData::Uint8x4(vec) => *vec = duplicate(vec, indices),
                AttributeData::Unorm8x4(vec) => *vec = duplicate(vec, indices),
            }
        }
    }

    /// Calculates the [`Mesh::ATTRIBUTE_NORMAL`] of a mesh.
    ///
    /// # Panics
    /// Panics if [`Indices`] are set or [`Mesh::ATTRIBUTE_POSITION`] is not of type `float3` or
    /// if the mesh has any other topology than [`PrimitiveTopology::TriangleList`].
    /// Consider calling [`Mesh::duplicate_vertices`] or export your mesh with normal attributes.
    pub fn compute_flat_normals(&mut self) {
        if self.indices.as_ref().is_none() {
            panic!("`compute_flat_normals` can't work on indexed geometry. Consider calling `Mesh::duplicate_vertices`.");
        }

        if !matches!(self.topology, PrimitiveTopology::TriangleList) {
            panic!("`compute_flat_normals` can only work on `TriangleList`s");
        }

        let positions = self
            .attribute(AttributeMetadata::ATTRIBUTE_POSITION)
            .unwrap()
            .as_float3()
            .expect("`Mesh::ATTRIBUTE_POSITION` vertex attributes should be of type `float3`");

        let normals: Vec<_> = positions
            .chunks_exact(3)
            .map(|p| face_normal(p[0], p[1], p[2]))
            .flat_map(|normal| [normal; 3])
            .collect();

        self.insert_attribute(
            AttributeMetadata::ATTRIBUTE_NORMAL,
            AttributeData::Float32x3(normals),
        );
    }
}
