use bevy::prelude::*;

use super::world_cmps::MyWorld;

pub fn ground(
    mut cmds: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    cmds.spawn((
        PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Plane {
                size: 5.0,
                ..default()
            })),
            material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
            ..default()
        },
        MyWorld,
        Name::new("Ground"),
    ));
}

pub fn light(mut cmds: Commands) {
    cmds.spawn((
        DirectionalLightBundle {
            transform: Transform::from_rotation(Quat::from_rotation_x(-0.5)),
            ..default()
        },
        MyWorld,
        Name::new("Light"),
    ));
}

pub fn despawn(mut cmds: Commands, world_q: Query<Entity, With<MyWorld>>) {
    for entity in world_q.iter() {
        cmds.entity(entity).despawn_recursive();
    }
}
