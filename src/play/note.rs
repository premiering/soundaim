use bevy::{asset::Handle, ecs::{component::Component, system::Resource}, pbr::StandardMaterial};

use bevy::{asset::Assets, ecs::{entity::Entity, query::{With, Without}, schedule::NextState, system::{Commands, Query, Res, ResMut}}, log::info, math::Vec3, pbr::{AlphaMode, PbrBundle}, render::{color::Color, view::Visibility}, time::Time, transform::components::Transform, utils::default};
use bevy_kira_audio::prelude::*;

use crate::{map::{Note, NoteData}, startup::GlobalAssets, state::GameState};

use super::{cursor::Cursor, play_state::{InPlay, PlayStateData, PlayStatePlugin}};

const APPROACH_RATE: i128 = 500;
const APPROACH_DIST: f32 = 25.0;
const NOTE_FADE_IN: i128 = 400;
const WAIT_TIME_START_FINISH: i128 = 500;
const GAME_RESYNC_THRESHOLD: i128 = 50;// How many milliseconds the audio has to be off of the game to trigger a resync
const NOTE_EARLY_HIT_WINDOW: i128 = 0;
const CURSOR_HITBOX: f32 = 0.2625/2.;

#[derive(Component)]
pub (crate) struct PlayNote {
    pub x: f32,
    pub y: f32,
    pub hit_ms: i128,
    pub hit_result: Option<HitResult>,
    pub note_material: Handle<StandardMaterial>
}

#[derive(PartialEq)]
pub enum HitResult {
    Hit, Miss
}

#[derive(Default, Resource)]
pub struct NotePaletteCycler {
    pub palette: Vec<Color>,
    pub current_material: usize,
}

impl NotePaletteCycler {
    pub fn new(palette: Vec<Color>) -> NotePaletteCycler {
        NotePaletteCycler {
            palette,
            current_material: 0,
        }
    }

    pub fn get_next(&mut self) -> Color {
        let mat = self.palette.get(self.current_material);
        self.current_material += 1;
        if self.current_material >= self.palette.len() {
            self.current_material = 0;
        }
        return mat.expect("Invalid note palette, no colors?").clone();
    }
}

// Tracks which notes should be added next
#[derive(Default)]
pub struct MapNoteTracker {
    data: NoteData,
    last_update_time: i128
}

impl MapNoteTracker {
    pub fn new(note_data: NoteData, play_speed: f32) -> Self {
        let mut t = Self {
            data: note_data.clone(),
            last_update_time: 0
        };
        for note in &mut t.data.0 {
            note.hit_ms = (note.hit_ms as f32 / play_speed as f32) as i128;
        }

        return t;
    }

    pub fn update_get_next(&mut self, time_ms: i128, approach_time_ms: i128) -> Option<Vec<Note>> {
        if self.data.0.len() == 0 {
            return None;
        }
        let mut notes: Vec<Note> = vec![];
        for note in &mut self.data.0 {
            if note.hit_ms - approach_time_ms < time_ms {
                notes.push(note.clone());
                continue;
            }
        }
        /*if notes.len() == 1 && self.data.0.len() == 1 && time_ms < notes.get(0).unwrap().hit_ms + approach_time_ms{
            return Some(notes);
        }*/
        self.data.0.retain(|x| 
            !notes.contains(x)
        );    
        self.last_update_time = time_ms;
        return Some(notes);
    }

    pub fn has_more_notes(&mut self) -> bool {
        return !self.data.0.len() == 0;
    }
}

pub fn init_note_manager(mut data: ResMut<PlayStateData>,
    note_datas: ResMut<Assets<NoteData>>, 
    globals: ResMut<GlobalAssets>,
    mut commands: Commands) {
    data.note_data = note_datas.get(&data.map.notes).expect("Note data not found!").clone();
    data.note_tracker = MapNoteTracker::new(data.note_data.clone(), data.play_speed);

    let note_materials = NotePaletteCycler::new(globals.note_palette.clone());
    commands.insert_resource(note_materials);
}

pub fn on_update(        
    mut data: ResMut<PlayStateData>, 
    time: ResMut<Time>, 
    mut note_query: Query<(Entity, &mut Transform, &mut PlayNote, &mut Visibility), With<PlayNote>>, 
    globals: ResMut<GlobalAssets>,
    audio: Res<Audio>,
    mut note_palette: ResMut<NotePaletteCycler>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut audio_instances: ResMut<Assets<AudioInstance>>,
    mut q_cursor: Query<&mut Transform, (With<Cursor>, Without<PlayNote>)>,
    mut state: ResMut<NextState<GameState>>,
    mut commands: Commands) {
    let mut current_time = time.elapsed();
    //current_time = PlayStatePlugin::duration_add_signed(current_time, data.resync_offset_ms);
    let mut current_time_ms = current_time.as_millis() as i128 - data.start_time.as_millis() as i128;

    // Sync audio and game if required
    if let Some(instance) = audio_instances.get_mut(&data.song) {
        match instance.state() {
            PlaybackState::Playing { position } => {
                let audio_offset = ((position / data.play_speed as f64) * 1000.0) as i128 - current_time_ms;
                if audio_offset.abs() >= GAME_RESYNC_THRESHOLD {
                    info!("Resyncing audio by {0}ms", audio_offset);
                    instance.seek_to((current_time_ms as f64 / 1000.) * data.play_speed as f64);
                    instance.resume(AudioTween::default());
                }
            }
            _ => {
                instance.resume(AudioTween::default());
            }
        }
    }

    let cursor_pos = q_cursor.get_single_mut().unwrap();

    // Update/remove the current notes
    for (entity, mut transform, mut note, mut visibility) in &mut note_query {
        // Testing...
        if current_time_ms > note.hit_ms + 200 {
            commands.entity(entity).despawn();
            if note.hit_result.is_none() {
                note.hit_result = Some(HitResult::Miss);
                data.current_combo = 0;
                data.misses += 1;
            }
        }

        if current_time_ms > note.hit_ms - NOTE_EARLY_HIT_WINDOW && note.hit_result.is_none() {
            // TODO: get the hit result, update the component, then store the result somewhere else
            let did_hit = did_cursor_hit(&note, &cursor_pos);
            if did_hit {
                note.hit_result = Some(HitResult::Hit);
                data.current_combo += 1;
                data.objects_hit += 1;
                if data.current_combo > data.max_combo {
                    data.max_combo = data.current_combo;
                }
                *visibility = Visibility::Hidden;
                audio.play(globals.hit_sound.clone()).with_volume(1.);
            }
        }
        let z_ratio: f32 = (note.hit_ms - current_time_ms) as f32 / APPROACH_RATE as f32; 
        let z: f32 = (z_ratio * APPROACH_DIST) as f32;
        transform.translation.z = z;
        let alpha: f32 = if current_time_ms > note.hit_ms - APPROACH_RATE + NOTE_FADE_IN {
            1.
        } else {
            (current_time_ms - (note.hit_ms - APPROACH_RATE) as i128) as f32 / NOTE_FADE_IN as f32
        };
        let mat = materials.get_mut(&note.note_material);
        if mat.is_some() {
            mat.unwrap().base_color.set_a(alpha * 1000.);
        }
    }

    // Check if the map has ended
    if !data.note_tracker.has_more_notes() && current_time_ms > (data.note_data.0[data.note_data.0.len() - 1].hit_ms as f32 / data.play_speed) as i128 + WAIT_TIME_START_FINISH {
        state.set(GameState::Menu);
        return;
    }

    // Add the notes that have just come into the approach rate field
    let new_notes = data.note_tracker.update_get_next(current_time_ms, APPROACH_RATE);
    if new_notes.is_none() {
        return;
    }
    for note in new_notes.unwrap() {
        let z_ratio: f32 = (note.hit_ms - current_time_ms) as f32 / APPROACH_RATE as f32; 
        let z: f32 = (z_ratio * APPROACH_DIST) as f32;
        let note_color = note_palette.get_next().into();
        let mat = materials.add(StandardMaterial {
            base_color: note_color,
            alpha_mode: AlphaMode::Blend,
            unlit: false,
            reflectance: 0.,
            emissive: note_color,
            ..default()
        });
        commands.spawn((
            PlayNote {
                x: note.x,
                y: note.y,
                hit_ms: note.hit_ms,
                hit_result: None,
                note_material: mat.clone(),
            },
            PbrBundle {
                mesh: globals.note_mesh.clone(),
                transform: Transform::from_xyz(note.x, note.y, z).with_scale(Vec3::new(0.45, 0.45, 0.45)),
                material: mat.clone(),
                ..default()
            },
            InPlay
        ));
    }
}

fn did_cursor_hit(note: &PlayNote, cursor_pos: &Transform) -> bool {
    let left = note.x - 0.5 - CURSOR_HITBOX;
    let top = note.y - 0.5 - CURSOR_HITBOX;
    let right = note.x + 0.5 + CURSOR_HITBOX;
    let bottom = note.y + 0.5 + CURSOR_HITBOX;
    let x = cursor_pos.translation.x;
    let y = cursor_pos.translation.y;
    return left < x && x < right && top < y && y < bottom;
}