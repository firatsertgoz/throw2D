use bevy::prelude::*;
use bevy_editor_pls::EditorPlugin;
use bevy_rapier2d::prelude::*;
use character::CharacterPlugin;
mod character;
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(EditorPlugin::default())
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        .add_plugins(RapierDebugRenderPlugin::default())
        .add_plugins(CharacterPlugin)
        .add_systems(Startup, setup_graphics)
        .add_systems(Startup, setup_physics)
        .add_systems(Update, apply_forces)
        .run();
}

#[derive(Component)]
pub struct Player;

fn setup_graphics(mut commands: Commands) {
    // Add a camera so we can see the debug-render.
    commands.spawn(Camera2dBundle::default());
}

fn setup_physics(mut commands: Commands) {
    /* Create the ground. */
    commands
        .spawn(Collider::cuboid(500.0, 50.0))
        .insert(TransformBundle::from(Transform::from_xyz(0.0, -100.0, 0.0)));

    /* Create the bouncing ball. */
    commands
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
        .insert(TransformBundle::from(Transform::from_xyz(0.0, 400.0, 0.0)))
        .insert(Player);
}

/* Apply forces and impulses inside of a system. */
fn apply_forces(
    mut ext_forces: Query<&mut ExternalForce, With<Player>>,
    mut ext_impulses: Query<&mut ExternalImpulse, With<Player>>,
    keyboard_input: ResMut<Input<KeyCode>>,
) {
    if keyboard_input.just_pressed(KeyCode::Down) {
        // Apply a force to the ball.
        for mut ext_force in ext_forces.iter_mut() {
            ext_force.force = Vec2::new(0.0, 10.0);
        }
    }
    // Apply forces.
    if keyboard_input.just_pressed(KeyCode::Up) {
        for mut ext_impulse in ext_impulses.iter_mut() {
            ext_impulse.impulse = Vec2::new(10.0, 0.0);
        }
    }
    // Apply impulses.
}
