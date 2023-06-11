use bevy::{
    prelude::*,
    render::{
        camera::RenderTarget,
        render_resource::{
            Extent3d, TextureDescriptor, TextureDimension, TextureFormat, TextureUsages,
        },
    },
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_system(bevy::window::close_on_esc)
        .add_startup_system(init)
        .add_system(update_image_size)
        .add_system(animate_cube)
        .run()
}

#[derive(Resource)]
struct RenderTargetImage(Handle<Image>);

#[derive(Component)]
struct MyCube;

fn init(
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut images: ResMut<Assets<Image>>,
) {
    commands.spawn((
        MyCube,
        PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Box::from_corners(
                Vec3::splat(-0.5),
                Vec3::splat(0.5),
            ))),
            material: materials.add(StandardMaterial::from(Color::RED)),
            ..default()
        },
    ));

    // This is the texture that will be rendered to.
    let mut image = Image {
        texture_descriptor: TextureDescriptor {
            label: None,
            size: Extent3d::default(),
            dimension: TextureDimension::D2,
            format: TextureFormat::Bgra8UnormSrgb,
            mip_level_count: 1,
            sample_count: 1,
            usage: TextureUsages::TEXTURE_BINDING
                | TextureUsages::COPY_DST
                | TextureUsages::RENDER_ATTACHMENT,
            view_formats: &[],
        },
        ..default()
    };
    image.resize(image.texture_descriptor.size);
    let image = images.add(image);

    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(2.0, 3.0, 4.0).looking_at(Vec3::ZERO, Vec3::Y),
        camera: Camera {
            target: RenderTarget::Image(image.clone()),
            order: -1,
            ..default()
        },
        ..default()
    });

    commands.spawn(PointLightBundle {
        transform: Transform::from_xyz(2.0, 3.0, 4.0),
        ..default()
    });

    commands.spawn(Camera2dBundle::default());

    commands
        .spawn(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                flex_direction: FlexDirection::Row,
                ..default()
            },

            ..default()
        })
        .with_children(|commands| {
            commands.spawn(ImageBundle {
                style: Style {
                    size: Size::new(Val::Percent(50.0), Val::Percent(100.0)),
                    flex_direction: FlexDirection::Row,
                    ..default()
                },
                image: UiImage::from(image.clone()),
                ..default()
            });
            commands.spawn(ImageBundle {
                style: Style {
                    size: Size::new(Val::Percent(50.0), Val::Percent(100.0)),
                    flex_direction: FlexDirection::Row,
                    ..default()
                },
                image: UiImage::from(image.clone()),
                ..default()
            });
        });

    commands.insert_resource(RenderTargetImage(image));
}

fn update_image_size(query: Query<(&Node, &UiImage)>, mut images: ResMut<Assets<Image>>) {
    for (node, ui_image) in query.iter() {
        let target_size = node.size().max(Vec2::ONE);
        let image = images.get_mut(&ui_image.texture).unwrap();
        image.resize(Extent3d {
            width: target_size.x as u32,
            height: target_size.y as u32,
            depth_or_array_layers: 1,
        });
    }
}

fn animate_cube(mut query: Query<&mut Transform, With<MyCube>>, time: Res<Time>) {
    for mut transform in query.iter_mut() {
        transform.rotate_y(time.delta_seconds());
    }
}
