use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_system(bevy::window::close_on_esc)
        .add_startup_system(init)
        .run()
}

fn init(
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Box::from_corners(
            Vec3::splat(-0.5),
            Vec3::splat(0.5),
        ))),
        material: materials.add(StandardMaterial::from(Color::RED)),
        ..default()
    });

    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(2.0, 3.0, 4.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });

    commands.spawn(PointLightBundle {
        transform: Transform::from_xyz(2.0, 3.0, 4.0),
        ..default()
    });
}
