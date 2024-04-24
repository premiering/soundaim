use std::{ops::{Add, Sub}, time::Duration};

use bevy::{app::{Plugin, Update}, asset::{AssetServer, Assets, Handle, LoadState}, core_pipeline::core_3d::Camera3dBundle, ecs::{change_detection::DetectChanges, component::Component, entity::Entity, query::With, schedule::{common_conditions::in_state, IntoSystemConfigs, NextState, OnEnter, OnExit}, system::{Commands, Query, Res, ResMut, Resource}}, hierarchy::DespawnRecursiveExt, math::{primitives::Cuboid, Vec3}, pbr::{AmbientLight, PbrBundle, StandardMaterial}, render::{camera::{PerspectiveProjection, Projection}, color::Color, mesh::Mesh}, transform::components::Transform, utils::default, window::{CursorGrabMode, PrimaryWindow, Window}};
use bevy_kira_audio::prelude::*;

use crate::{map::{Map, NoteData}, startup::GlobalAssets, state::GameState};

use super::{cursor::{self, CursorTransformParallax}, hud, note::{self, MapNoteTracker}, sound};

#[derive(Resource, Default)]
pub struct PlayStateData {
    pub map: Map,
    pub song: Handle<AudioInstance>,
    pub note_data: NoteData,
    pub note_tracker: MapNoteTracker,
    pub start_time: Duration,
    //pub last_update_time: Duration,
    pub current_combo: i128,
    pub objects_hit: i128,
    pub misses: i128,
    pub max_combo: i128,
    pub play_speed: f32,
}

impl PlayStateData {
    // Normalized to 0 - 100
    pub fn get_accuracy(&mut self) -> f32 {
        if self.misses == 0 {
            return 100.;
        }
        return (self.objects_hit as f32 / (self.objects_hit + self.misses) as f32) * 100.;
    }
}

pub struct PlayStatePlugin;

#[derive(Component)]
pub struct InPlay;

#[derive(Component)]
pub struct PlayCamera;

// Resource to keep track of the progress to load and play a map.
// This should be inserted whenever a map wants to be played, it will be detected and the map will play when loaded.
#[derive(Resource)]
pub struct MapLoadPlayResource {
    map: Map,
}

impl MapLoadPlayResource {
    pub fn create_loaded(map: Map) -> MapLoadPlayResource {
        MapLoadPlayResource {
            map
        }
    }
}

fn poll_map_load_play(
    mut commands: Commands, 
    opt_map_load_play: Option<ResMut<MapLoadPlayResource>>,
    mut state: ResMut<NextState<GameState>>
    ) {
    if opt_map_load_play.is_none() {
        return;
    }
    let map_load_play = opt_map_load_play.unwrap();
    let added = map_load_play.is_added();

    if added {
        let mut play_state_data = PlayStateData::default();
        play_state_data.play_speed = 1.;
        play_state_data.map = map_load_play.map.clone();
        commands.insert_resource(play_state_data);
        state.set(GameState::Play);
        commands.remove_resource::<MapLoadPlayResource>();
    }
}

impl PlayStatePlugin {
    fn on_enter(
        mut data: ResMut<PlayStateData>,
        mut meshes: ResMut<Assets<Mesh>>,
        mut materials: ResMut<Assets<StandardMaterial>>,
        mut commands: Commands
    ) { 
        // Spawn camera
        commands.spawn((Camera3dBundle {
            transform: Transform::from_xyz(0.0, 0.0, -4.0).looking_at(Vec3::ZERO, Vec3::Y),
            projection: Projection::Perspective(PerspectiveProjection {
                fov: 70.0_f32.to_radians(),
                aspect_ratio: 16./9.,
                ..default()
            }),
            ..default()
        }, InPlay, PlayCamera,
        CursorTransformParallax {
            parallax_amount: 200.
        }));

        // Spawn the sky
        commands.spawn((
            PbrBundle {
                mesh: meshes.add(Mesh::from(Cuboid::new(100.0, 100.0, 100.0))),
                material: materials.add(StandardMaterial {
                    base_color: Color::rgb(0.0, 0.0, 0.0),
                    unlit: true,
                    cull_mode: None,
                    ..default()
                }),
                transform: Transform::from_scale(Vec3::splat(20.0)),
                ..default()
            }, InPlay
        ));

        // Ambient lighting
        commands.insert_resource(AmbientLight {
            color: Color::WHITE,
            brightness: 100.0,
        });
    }

    fn update_window_cursor_state(mut q_windows: Query<&mut Window, With<PrimaryWindow>>) {
        let mut window = q_windows.single_mut();
        window.cursor.grab_mode = CursorGrabMode::Locked;
        window.cursor.visible = false;
    }

    fn on_exit(mut commands: Commands, mut q_windows: Query<&mut Window, With<PrimaryWindow>>, q_entities: Query<Entity, With<InPlay>>) {
        let mut window = q_windows.single_mut();
        window.cursor.grab_mode = CursorGrabMode::None;
        window.cursor.visible = true;
        for ent in q_entities.iter() {
            commands.entity(ent).despawn_recursive();
        } 
        commands.remove_resource::<AmbientLight>();
    }

    pub fn duration_add_signed(a: Duration, add: i128) -> Duration {
        if add >= 0 {
            return a.add(Duration::from_millis(add as u64));
        } else {
            return a.sub(Duration::from_millis(-add as u64));
        }
    }
}

impl Plugin for PlayStatePlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(OnEnter(GameState::Play), (
            PlayStatePlugin::on_enter,
            hud::init_hud,
            cursor::init_cursor,
            note::init_note_manager,
            sound::init_sound
        ));
        app.add_systems(OnExit(GameState::Play), PlayStatePlugin::on_exit);

        // Bevy's system is not the best (or i'm misusing?)
        // This is required so that the systems are executed in the correct order.
        let update_cursor = cursor::on_update.run_if(in_state(GameState::Play));
        let update_notes = note::on_update.run_if(in_state(GameState::Play));
        let update_hud = hud::on_update.run_if(in_state(GameState::Play));
        let update_win_cursor = PlayStatePlugin::update_window_cursor_state.run_if(in_state(GameState::Play));
        app.add_systems(Update, (
            update_cursor.before(note::on_update),
            update_notes.before(hud::on_update),
            update_hud.after(note::on_update),
            update_win_cursor,
            poll_map_load_play
        ));
    }
}