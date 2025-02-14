use rapier3d::prelude::*;

pub fn setup_physics() -> (PhysicsPipeline, RigidBodySet, ColliderSet) {
    let gravity = Vector3::new(0.0, -9.81, 0.0);
    let mut physics_pipeline = PhysicsPipeline::new();
    let mut rigid_body_set = RigidBodySet::new();
    let mut collider_set = ColliderSet::new();

    // Add ground plane
    let ground_collider = ColliderBuilder::cuboid(10.0, 0.1, 10.0).build();
    collider_set.insert(ground_collider);

    (physics_pipeline, rigid_body_set, collider_set)
}

pub fn step_physics(
    physics_pipeline: &mut PhysicsPipeline,
    rigid_body_set: &mut RigidBodySet,
    collider_set: &mut ColliderSet,
) {
    let physics_dt = 1.0 / 60.0;
    physics_pipeline.step(
        &Vector3::new(0.0, -9.81, 0.0),
        &IntegrationParameters::default(),
        rigid_body_set,
        collider_set,
        &mut (),
        &mut (),
        &(),
        &(),
    );
}

