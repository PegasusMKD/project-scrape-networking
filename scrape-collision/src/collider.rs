use crate::rapier::IntoRapier;
pub use rapier3d::{
    control::CharacterCollision,
    control::KinematicCharacterController,
    geometry::ColliderHandle,
    parry::query::TOIStatus,
    na::{ArrayStorage, Const, Matrix, Point3},
    prelude::{
        Collider, ColliderBuilder, ColliderSet, QueryFilter, QueryPipeline, RigidBody,
        RigidBodyBuilder, RigidBodyHandle, RigidBodySet,
    },
};

use rapier3d::{
    na::{Quaternion, UnitQuaternion}, prelude::{
        BroadPhase, CCDSolver, ImpulseJointSet, IntegrationParameters, IslandManager,
        MultibodyJointSet, NarrowPhase, PhysicsPipeline,
    }
};

use scrape_gltf_loader::loader::load_gltf_file;

pub struct Movement {
    pub next_position: Matrix<f32, Const<3>, Const<1>, ArrayStorage<f32, 3, 1>>,
    pub collisions: Vec<CharacterCollision>,
}

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
        entity_handle: RigidBodyHandle,
        exclude_colliders: Vec<ColliderHandle>,
        desired: Vec<f32>,
    ) -> Movement {
        self.query_pipeline.update(&self.bodies, &self.colliders);
        let desired_translation = desired.into_rapier();
        let entity = self.get_entity(entity_handle);
        let starting_translation = entity.position();
        let entity_collider = ColliderBuilder::capsule_y(0.5, 0.2).build();

        let (sender, mut receiver) = tokio::sync::mpsc::unbounded_channel();
        let mut events = Vec::new();

        let exclude_fn = |handle, _collider: &_| !exclude_colliders.contains(&handle);
        let query_filters = QueryFilter::new().predicate(&exclude_fn).exclude_rigid_body(entity_handle);

        let calculated_movement = self.controller.move_shape(
            self.integration_parameters.dt,
            &self.bodies,
            &self.colliders,
            &self.query_pipeline,
            entity_collider.shape(),
            &starting_translation,
            desired_translation,
            query_filters,
            move |event| {
                sender.send(event).unwrap();
            }, // if we need to fetch any events, use this fn
        );

        while let Ok(event) = receiver.try_recv() {
            events.push(event);
        }

        Movement {
            next_position: calculated_movement.translation,
            collisions: events,
        }
    }

    pub fn update_entity_rotation(
        &mut self,
        entity_handle: RigidBodyHandle,
        i: f32,
        j: f32,
        k: f32,
        w: f32,
    ) {
        let new_rotation = Quaternion::new(w, i, j, k);
        let entity = self.get_mut_entity(entity_handle);
        let quaternion = UnitQuaternion::new_normalize(new_rotation);
        entity.set_next_kinematic_rotation(quaternion);
    }

    pub fn get_entity(&self, entity_handle: RigidBodyHandle) -> &RigidBody {
        self.bodies
            .get(entity_handle)
            .expect("entity did not exist in the RigidBodySet")
    }

    pub fn get_mut_entity(&mut self, entity_handle: RigidBodyHandle) -> &mut RigidBody {
        self.bodies
            .get_mut(entity_handle)
            .expect("entity did not exist in the RigidBodySet")
    }

    pub fn load_entity(&mut self, spawn: Vec<f32>) -> (RigidBodyHandle, ColliderHandle) {
        let entity_collider = ColliderBuilder::capsule_y(0.5, 0.2).build();
        let handle = self.bodies.insert(
            RigidBodyBuilder::kinematic_position_based()
                .translation(spawn.into_rapier()) // Maybe add rotation in the future
                .enabled(true)
                // .ccd_enabled(true)
                .build(),
        );
        let collider = self
            .colliders
            .insert_with_parent(entity_collider, handle, &mut self.bodies);

        (handle, collider)
    }

    pub fn unload_entity(&mut self, handle: RigidBodyHandle) {
        self.bodies.remove(
            handle,
            &mut self.island_manager,
            &mut self.colliders,
            &mut self.impulse_joint_set,
            &mut self.multibody_joint_set,
            true,
        );
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
        let collider = ColliderBuilder::trimesh(vertices, indices)
            .translation(center.into_rapier())
            .build();
        self.colliders
            .insert_with_parent(collider, body_handle, &mut self.bodies);
    }

    pub fn new(map_path: String) -> Self {
        let mut collider = Self::default();
        collider.load_collider(map_path);
        collider
    }

    pub fn run_step(&mut self) {
        let hooks = ();
        let events = (); // TODO: Create event-handler
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
        let (body_handle, collider_handle) = collider.load_entity(vec![11.0, 2.0, 1.0]);

        let movement = collider.calculate_movement(body_handle, vec![body_handle], vec![1.0, 2.0, 1.0]);
        let entity = collider.get_mut_entity(body_handle);
        entity.set_next_kinematic_translation(entity.translation() + movement.next_position);
        collider.run_step();

        let entity = collider.get_entity(body_handle);
        println!("Position: {:?}", entity.translation());
    }
}
