use bevy::{asset::Assets, ecs::{component::Component, event::EventReader, query::{With, Without}, system::{Commands, Query, ResMut}}, input::mouse::MouseMotion, math::{primitives::Rectangle, Vec3}, render::mesh::Mesh, transform::components::Transform, utils::default};
use bevy_mod_billboard::{BillboardMeshHandle, BillboardTextureBundle, BillboardTextureHandle};

use crate::startup::GlobalAssets;

use super::{note::PlayNote, play_state::InPlay};

#[derive(Component)]
pub struct Cursor;

#[derive(Component)]
pub struct CursorTransformParallax {
    pub parallax_amount: f32
}

pub fn init_cursor(
    mut meshes: ResMut<Assets<Mesh>>,
    globals: ResMut<GlobalAssets>,
    mut commands: Commands
) { 
    // Spawn cursor
    commands.spawn((BillboardTextureBundle {
        transform: Transform::from_translation(Vec3::ZERO).with_scale(Vec3::splat(1.)),
        texture: BillboardTextureHandle(globals.cursor.clone()),
        mesh: BillboardMeshHandle(meshes.add(Rectangle::new(0.35, 0.35))),
        ..default()
    }, InPlay, Cursor));
}

pub fn on_update(
    mut motion_reader: EventReader<MouseMotion>, 
    mut q_cursor: Query<&mut Transform, (With<Cursor>, Without<PlayNote>)>,
    mut q_parallax: Query<(&mut Transform, &CursorTransformParallax), (With<CursorTransformParallax>, Without<Cursor>)>
) {
    // Update cursor position
    let mut cursor_pos = q_cursor.get_single_mut().unwrap();
    for ev in motion_reader.read() {
        cursor_pos.translation.x -= ev.delta.x / 150.0;
        cursor_pos.translation.y -= ev.delta.y / 150.0;
        cursor_pos.translation.x = cursor_pos.translation.x.clamp(-1.5, 1.5);
        cursor_pos.translation.y = cursor_pos.translation.y.clamp(-1.5, 1.5);
    }

    for (mut transform, parallax) in q_parallax.iter_mut() {
        transform.translation.x = -cursor_pos.translation.x / parallax.parallax_amount;
        transform.translation.y = -cursor_pos.translation.y / parallax.parallax_amount;
    }
}
