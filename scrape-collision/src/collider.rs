use crate::rapier::IntoRapier;
use rapier3d::prelude::{
    ActiveEvents, BroadPhase, CCDSolver, ColliderHandle, ImpulseJointSet, IntegrationParameters,
    IslandManager, MultibodyJoint, MultibodyJointSet, NarrowPhase, PhysicsPipeline,
};
pub use rapier3d::{
    control::KinematicCharacterController,
    na::{ArrayStorage, Const, Matrix, Point3},
    prelude::{
        Collider, ColliderBuilder, ColliderSet, QueryFilter, QueryPipeline, RigidBody,
        RigidBodyBuilder, RigidBodyHandle, RigidBodySet,
    },
};
use scrape_gltf_loader::loader::load_gltf_file;

pub struct GameCollider {
    bodies: RigidBodySet,
    colliders: ColliderSet,
    controller: KinematicCharacterController,
    query_pipeline: QueryPipeline,
    physics_pipeline: PhysicsPipeline,

    // NOT USED
    integration_parameters: IntegrationParameters,
    island_manager: IslandManager,
    broad_phase: BroadPhase,
    narrow_phase: NarrowPhase,
    impulse_joint_set: ImpulseJointSet,
    multibody_joint_set: MultibodyJointSet,
    ccd_solver: CCDSolver,
    gravity: Vec<f32>,
}

impl GameCollider {
    pub fn calculate_movement(
        &mut self,
        player_handle: RigidBodyHandle,
        desired: Vec<f32>,
        delta: f32,
    ) -> Matrix<f32, Const<3>, Const<1>, ArrayStorage<f32, 3, 1>> {
        self.query_pipeline.update(&self.bodies, &self.colliders);
        let desired_translation = desired.into_rapier();
        let player = self.get_player(player_handle);
        let starting_translation = player.position();
        let player_collider = ColliderBuilder::capsule_y(0.5, 0.2).build();

        let calculated_movement = self.controller.move_shape(
            self.integration_parameters.dt,
            &self.bodies,
            &self.colliders,
            &self.query_pipeline,
            player_collider.shape(),
            &starting_translation,
            desired_translation,
            QueryFilter::new().exclude_rigid_body(player_handle),
            |_event| {}, // if we need to fetch any events, use this fn
        );

        calculated_movement.translation
    }

    pub fn get_player(&self, player_handle: RigidBodyHandle) -> &RigidBody {
        self.bodies
            .get(player_handle)
            .expect("Player did not exist in the RigidBodySet")
    }

    pub fn get_mut_player(&mut self, player_handle: RigidBodyHandle) -> &mut RigidBody {
        self.bodies
            .get_mut(player_handle)
            .expect("Player did not exist in the RigidBodySet")
    }

    pub fn load_player(&mut self, spawn: Vec<f32>) -> RigidBodyHandle {
        let player_collider = ColliderBuilder::capsule_y(0.5, 0.2).build();
        let handle = self.bodies.insert(
            RigidBodyBuilder::kinematic_position_based()
                .translation(spawn.into_rapier()) // Maybe add rotation in the future
                .enabled(true)
                // .ccd_enabled(true)
                .build(),
        );
        self.colliders
            .insert_with_parent(player_collider, handle, &mut self.bodies);
        handle
    }

    pub fn unload_player(&mut self, handle: RigidBodyHandle) {
        self.bodies.remove(handle, &mut self.island_manager, &mut self.colliders, &mut self.impulse_joint_set, &mut self.multibody_joint_set, true);
    }

    // "./data/environment.gltf"
    pub fn load_collider(&mut self, map_path: String) {
        // Hardcode to [0] since it will
        // always be 1 big mesh
        let mesh = &load_gltf_file(map_path.to_string())[0];
        let sub_tri_meshes = mesh.mesh_collection();
        for tri_mesh in sub_tri_meshes {
            self.add_tri_mesh(tri_mesh.vertices, tri_mesh.indices);
        }
    }

    pub fn add_tri_mesh(&mut self, in_vertices: Vec<[f32; 3]>, indices: Vec<[u32; 3]>) {
        let center: Vec<f32> = vec![0.0, 0.0, 0.0];
        
        let vertices = in_vertices
            .iter()
            .map(|vec| Point3::new(vec[0], vec[1], vec[2]))
            .collect();
        let collider_body = RigidBodyBuilder::fixed().build();
        let body_handle = self.bodies.insert(collider_body);
        let collider = ColliderBuilder::trimesh(vertices, indices).translation(center.into_rapier()).build();
        self.colliders
            .insert_with_parent(collider.clone(), body_handle, &mut self.bodies);
    }

    pub fn new(map_path: String) -> Self {
        let mut collider = Self::default();
        collider.load_collider(map_path);
        collider
    }

    pub fn run_step(&mut self) {
        let hooks = ();
        let events = ();
        self.physics_pipeline.step(
            &self.gravity.into_rapier(),
            &self.integration_parameters,
            &mut self.island_manager,
            &mut self.broad_phase,
            &mut self.narrow_phase,
            &mut self.bodies,
            &mut self.colliders,
            &mut self.impulse_joint_set,
            &mut self.multibody_joint_set,
            &mut self.ccd_solver,
            Some(&mut self.query_pipeline),
            &hooks,
            &events,
        );

    }
}

impl Default for GameCollider {
    fn default() -> Self {
        // TODO: Create custom controller for Scrape which will configure all of these by default
        let mut controller = KinematicCharacterController::default();
        controller.slide = true;
        // Don’t allow climbing slopes larger than 45 degrees.
        controller.max_slope_climb_angle = 60.0_f32.to_radians();
        // Automatically slide down on slopes smaller than 30 degrees.
        controller.min_slope_slide_angle = 30.0_f32.to_radians();
        controller.offset = rapier3d::control::CharacterLength::Relative(0.01);

        Self {
            bodies: RigidBodySet::new(),
            colliders: ColliderSet::new(),
            controller,
            query_pipeline: QueryPipeline::new(),
            physics_pipeline: PhysicsPipeline::new(),

            integration_parameters: IntegrationParameters::default(),
            island_manager: IslandManager::new(),
            broad_phase: BroadPhase::new(),
            narrow_phase: NarrowPhase::new(),
            impulse_joint_set: ImpulseJointSet::new(),
            multibody_joint_set: MultibodyJointSet::new(),
            ccd_solver: CCDSolver::new(),
            gravity: vec![0.0, 0.0, 0.0],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::GameCollider;

    #[test]
    pub fn simple_out_of_bounds_test() {
        let mut collider = GameCollider::new("./data/environment.gltf".to_string());
        let handle = collider.load_player(vec![11.0, 2.0, 1.0]);
        
        let data = collider.calculate_movement(handle, vec![1.0, 2.0, 1.0], 10.0);
        let player = collider.get_mut_player(handle);
        player.set_next_kinematic_translation(player.translation() + data);
        collider.run_step(); 

        let player = collider.get_player(handle);
        println!("Position: {:?}", player.translation());
    }
}
