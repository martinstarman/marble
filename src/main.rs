use avian2d::prelude::*;
use bevy::{
    prelude::*,
    window::{PrimaryWindow, WindowResolution},
};

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
        .insert_resource(Gravity(Vec2::NEG_Y * 9.81 * 100.0))
        .add_systems(Startup, setup)
        .add_systems(Update, click)
        .run()
}

fn setup(mut commands: Commands) {
    commands.spawn((MainCamera, Camera2dBundle::default()));

    commands.spawn((
        Ball,
        // RigidBody::Dynamic,
        RigidBody::Static,
        Collider::circle(10.),
        TransformBundle::from_transform(Transform::from_xyz(0., 290., 0.)),
        Restitution::PERFECTLY_ELASTIC,
        LinearVelocity::ZERO,
    ));

    //
    commands.spawn((
        //
        RigidBody::Static,
        Collider::circle(10.),
        TransformBundle::from_transform(Transform::from_xyz(-100., 0., 0.)),
        Restitution::PERFECTLY_ELASTIC,
    ));

    commands.spawn((
        //
        RigidBody::Static,
        Collider::circle(10.),
        TransformBundle::from_transform(Transform::from_xyz(100., 0., 0.)),
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

fn click(
    mut query: Query<(&mut RigidBody, &mut LinearVelocity), With<Ball>>,
    mouse: Res<ButtonInput<MouseButton>>,
    windows: Query<&Window, With<PrimaryWindow>>,
    camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
) {
    if !mouse.just_pressed(MouseButton::Left) {
        return;
    }

    let window = windows.single();

    if let Some(cursor_position) = window.cursor_position() {
        let (camera, global_transform) = camera.single();

        if let Some(position) = camera.viewport_to_world_2d(global_transform, cursor_position) {
            for (mut rigid_body, mut linear_velocity) in &mut query {
                *rigid_body = RigidBody::Dynamic;
                *linear_velocity = LinearVelocity(position);
            }
        }
    }
}
