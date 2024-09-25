use std::f32::consts::FRAC_PI_4;

use avian2d::prelude::*;
use bevy::{prelude::*, window::WindowResolution};

#[derive(Component)]
struct MainCamera;

#[derive(Component)]
struct Ball;

#[derive(Component)]
struct Floor;

fn main() -> AppExit {
    App::new()
        .add_plugins((
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Marble".into(),
                        resolution: WindowResolution::new(800., 600.),
                        ..Default::default()
                    }),
                    ..Default::default()
                }),
            PhysicsPlugins::default(),
            PhysicsDebugPlugin::default(),
        ))
        .insert_resource(Gravity(Vec2::NEG_Y * 1000.))
        .add_systems(Startup, setup)
        .run()
}

fn setup(mut commands: Commands) {
    commands.spawn((MainCamera, Camera2dBundle::default()));

    commands.spawn((
        Ball,
        RigidBody::Dynamic,
        Collider::circle(10.),
        TransformBundle::from_transform(Transform::from_xyz(0., 100., 0.)),
        Restitution::PERFECTLY_ELASTIC,
    ));

    commands.spawn((
        Floor,
        RigidBody::Static,
        Collider::rectangle(200., 10.),
        TransformBundle::from_transform(
            Transform::from_xyz(0., 0., 0.).with_rotation(Quat::from_rotation_z(FRAC_PI_4)),
        ),
    ));
}
