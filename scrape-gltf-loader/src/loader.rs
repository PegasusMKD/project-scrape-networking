use gltf::{mesh::util::ReadIndices, Semantic};

use crate::conversion;
use crate::core::{enums, utility};
use crate::indices::IndexVec;
use crate::mesh::{attributes::*, mesh_primitive::MeshPrimitive, Mesh};

// "./blend-files/Envoirment.gltf"
pub fn load_gltf_file(file_path: String) -> Vec<Mesh> {
    let (document, buffers, _images) =
        gltf::import(file_path).expect("Couldn't read provided file_path");
    let mut meshes = Vec::new();
    for mesh in document.meshes() {
        let mut mesh_primitives = Vec::new();

        for primitive in mesh.primitives() {
            let primitive_topology = utility::get_primitive_topology(primitive.mode()).unwrap();

            let mut mesh_primitive = MeshPrimitive::new(primitive_topology);
            for (semantic, accessor) in primitive.attributes() {
                if [Semantic::Joints(0), Semantic::Weights(0)].contains(&semantic) {
                    continue;
                }

                match conversion::convert_attribute(semantic, accessor, &buffers, None) {
                    Ok((attribute, values)) => mesh_primitive.insert_attribute(attribute, values),
                    Err(_err) => eprintln!("Something went wrong with adding the attribute..."),
                }
            }

            let reader = primitive.reader(|buffer| Some(buffers[buffer.index()].0.as_slice()));

            if let Some(indices) = reader.read_indices() {
                mesh_primitive.set_indices(Some(match indices {
                    ReadIndices::U8(is) => IndexVec::U16(is.map(|x| x as u16).collect()),
                    ReadIndices::U16(is) => IndexVec::U16(is.collect()),
                    ReadIndices::U32(is) => IndexVec::U32(is.collect()),
                }));
            };

            if mesh_primitive
                .attribute(attribute_metadata::AttributeMetadata::ATTRIBUTE_NORMAL)
                .is_none()
                && matches!(
                    mesh_primitive.topology.clone(),
                    enums::PrimitiveTopology::TriangleList
                )
            {
                let vertex_count_before = mesh_primitive.count_vertices();
                mesh_primitive.duplicate_vertices();
                mesh_primitive.compute_flat_normals();
                let vertex_count_after = mesh_primitive.count_vertices();

                if vertex_count_before != vertex_count_after {
                    println!("Missing vertex normals in indexed geometry, computing them as flat. Vertex count increased from {} to {}", vertex_count_before, vertex_count_after);
                } else {
                    println!("Missing vertex normals in indexed geometry, computing them as flat.");
                }
            }

            if let Some(vertex_attribute) = reader
                .read_tangents()
                .map(|v| attribute_data::AttributeData::Float32x4(v.collect()))
            {
                mesh_primitive.insert_attribute(
                    attribute_metadata::AttributeMetadata::ATTRIBUTE_TANGENT,
                    vertex_attribute,
                );
            }

            mesh_primitives.push(mesh_primitive);
        }

        meshes.push(Mesh { mesh_primitives });
    }

    return meshes;
}

#[cfg(test)]
mod tests {
    use crate::loader::load_gltf_file;

    #[test]
    fn test_load() {
        let meshes = load_gltf_file("./blend-files/Envoirment.gltf".to_string());
        assert_eq!(meshes.len(), 1);

        let mut vertices = 0;
        let mut indice_count = 0;
        for mesh in meshes.iter() {
            for item in &mesh.mesh_primitives {
                vertices += item.count_vertices();
                if let Some(index) = &item.indices {
                    indice_count += index.len();
                }
            }
        }

        assert_eq!(meshes.len(), 1);

        assert_eq!(vertices, 1992);
        println!("Total vertex count: {}", vertices);

        assert_eq!(indice_count / 3, 1256);
        println!("Total triangles (indices/3): {}", indice_count / 3);
    }
}
