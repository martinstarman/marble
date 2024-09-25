use avian2d::prelude::*;
use bevy::{prelude::*, window::WindowResolution};

#[derive(Component)]
struct MainCamera;

#[derive(Component)]
struct Ball;

#[derive(Component)]
struct Wall;

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

    // top
    commands.spawn((
        Wall,
        RigidBody::Static,
        Collider::rectangle(800., 1.),
        TransformBundle::from_transform(Transform::from_xyz(0., 300., 0.)),
        Restitution::PERFECTLY_ELASTIC,
    ));

    // bottom
    commands.spawn((
        Wall,
        RigidBody::Static,
        Collider::rectangle(800., 1.),
        TransformBundle::from_transform(Transform::from_xyz(0., -300., 0.)),
        Restitution::PERFECTLY_ELASTIC,
    ));

    // left
    commands.spawn((
        Wall,
        RigidBody::Static,
        Collider::rectangle(1., 600.),
        TransformBundle::from_transform(Transform::from_xyz(-400., 0., 0.)),
        Restitution::PERFECTLY_ELASTIC,
    ));

    // right
    commands.spawn((
        Wall,
        RigidBody::Static,
        Collider::rectangle(1., 600.),
        TransformBundle::from_transform(Transform::from_xyz(400., 0., 0.)),
        Restitution::PERFECTLY_ELASTIC,
    ));
}
