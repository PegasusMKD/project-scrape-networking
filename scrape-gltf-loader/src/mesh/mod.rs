pub mod attributes;
pub mod mesh_primitive;

use mesh_primitive::MeshPrimitive;

use self::attributes::attribute_metadata::AttributeMetadata;

// Add 'Copy' trait as derived
#[derive(Debug, Clone)]
pub struct Mesh {
    pub mesh_primitives: Vec<MeshPrimitive>,
}

pub struct SubTriMesh {
    pub vertices: Vec<[f32; 3]>,
    pub indices: Vec<[u32; 3]>,
}

impl Mesh {
    pub fn mesh_collection(&self) -> Vec<SubTriMesh> {
        let mut sub_tri_meshes = Vec::new();
        for (idx, _primitive) in self.mesh_primitives.iter().enumerate() {
            sub_tri_meshes.push(SubTriMesh { vertices: self.get_vertices(idx), indices: self.get_indices(idx) })
        }

        sub_tri_meshes
    }

    pub fn get_vertices(&self, index: usize) -> Vec<[f32; 3]> {
        let primitives = &self.mesh_primitives;
        if index >= primitives.len() {
            eprintln!("Primitive Index out of bounds!");
            return Vec::new();
        }

        let mut data = Vec::new();
        let primitive = &primitives[index];
        let o_points = primitive
            .attribute(AttributeMetadata::ATTRIBUTE_POSITION)
            .unwrap()
            .as_float3()
            .unwrap();
        data.extend(o_points);
        return data;
    }

    pub fn get_indices(&self, index: usize) -> Vec<[u32; 3]> {
        let primitives = &self.mesh_primitives;
        let mut indices = Vec::new();
        if index >= primitives.len() {
            eprintln!("Primitive Index out of bounds!");
            return Vec::new();
        }
        let primitive = &primitives[index];
        if let Some(index_data) = &primitive.indices {
            let mut grouped = index_data.iter();
            loop {
                match grouped.next_chunk::<3>() {
                    Ok(data) => indices.push(data.map(|coord| coord as u32)),
                    Err(_) => break,
                };
            }
        }

        return indices;
    }
}
