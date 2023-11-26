use bevy::prelude::*;
use bevy_editor_pls::prelude::*;
use bevy_rapier3d::prelude::*;
use scrape_gltf_loader::loader::load_gltf_file;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugins(RapierDebugRenderPlugin::default())
        .add_plugins(EditorPlugin::default())
        .add_systems(Startup, setup_physics)
        .run();
}

fn setup_physics(mut commands: Commands) {
    /* Create the ground. */
    let mesh = &load_gltf_file("./data/environment.gltf".to_string())[0];
    let tri_meshes = mesh.mesh_collection();
    for tri_mesh in tri_meshes {
        let vertices = tri_mesh
            .vertices
            .iter()
            .map(|vec| Vec3::new(vec[0], vec[1], vec[2]))
            .collect();
        let indices = tri_mesh.indices;
        commands
            .spawn(Collider::trimesh(vertices, indices))
            .insert(TransformBundle::from(Transform::from_xyz(0.0, -2.0, 0.0)));
    }
}
