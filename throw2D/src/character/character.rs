use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

#[derive(Component)]
pub struct Head;
#[derive(Component)]
pub struct Torso;
#[derive(Component)]
pub struct LeftArm;
#[derive(Component)]
pub struct RightArm;
#[derive(Component)]
pub struct LeftLeg;
#[derive(Component)]

pub struct RightLeg;
pub(crate) fn setup_character(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let joint = RevoluteJointBuilder::new()
        .local_anchor1(Vec2::new(0.0, 1.0))
        .local_anchor2(Vec2::new(0.0, -3.0)); // Create the torso.
    let body_id = commands
        .spawn(RigidBody::Dynamic)
        .insert(Collider::ball(50.0))
        .insert(Restitution::coefficient(0.7))
        .insert(ExternalForce {
            force: Vec2::new(0.0, 0.0),
            torque: 0.0,
        })
        .insert(ExternalImpulse {
            impulse: Vec2::new(0.0, 0.0),
            torque_impulse: 0.0,
        })
        .insert(TransformBundle::from(Transform::from_xyz(0.0, 0.0, 0.0)))
        .id();
    commands
        .spawn(RigidBody::Dynamic)
        .insert(Collider::capsule(
            Vec2::new(0.0, -50.0),
            Vec2::new(0.0, -60.0),
            1.0,
        ))
        .insert(Restitution::coefficient(0.7))
        .insert(ExternalForce {
            force: Vec2::new(0.0, 0.0),
            torque: 0.0,
        })
        .insert(ExternalImpulse {
            impulse: Vec2::new(0.0, 0.0),
            torque_impulse: 0.0,
        })
        .insert(TransformBundle::from(Transform::from_xyz(0.0, -25.0, 0.0)))
        .insert(ImpulseJoint::new(body_id, joint));
}
