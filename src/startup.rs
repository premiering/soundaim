use bevy::{app::{App, Plugin, Update}, asset::{AssetServer, Handle, LoadState}, ecs::{schedule::{common_conditions::in_state, IntoSystemConfigs, NextState, OnEnter}, system::{Commands, ResMut}}, render::{color::Color, mesh::Mesh, texture::Image}, text::Font};
use bevy_common_assets::json::JsonAssetPlugin;
use bevy_kira_audio::{Audio, AudioSource};
use bevy::prelude::*;
use ::serde::Deserialize;
use serde_json::Value;

use crate::{map::{Map, NoteData}, state::GameState};

pub struct StartupPlugin;

impl Plugin for StartupPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Startup), StartupPlugin::on_startup);
        //app.add_systems(Update, StartupPlugin::on_update.run_if(in_state(GameState::Startup)));
    }
}

//TODO: Check loading of all assets before entering menu screen.
#[derive(Resource)]
pub struct GlobalAssets {
    pub note_mesh: Handle<Mesh>,
    pub hit_sound: Handle<AudioSource>,
    pub note_palette: Vec<Color>,
    pub play_grid: Handle<Image>,
    pub cursor: Handle<Image>,
    pub main_font: Handle<Font>,
    pub play_grade_box: Handle<Image>,
    pub maps_path: String,
    pub test_map: Map,
}

impl StartupPlugin {
    fn on_startup(server: ResMut<AssetServer>, mut commands: Commands, mut state: ResMut<NextState<GameState>>) {
        let assets = GlobalAssets {
            note_mesh: server.load::<Mesh>("meshes/circle_note.obj"),
            hit_sound: server.load::<AudioSource>("sounds/hit.ogg"),
            note_palette: vec![// Wii color palette
                Color::hex("#008dfeff").unwrap(), 
                Color::hex("#ed3434ff").unwrap(), 
                Color::hex("#11bd0cff").unwrap(), 
                Color::hex("#feb200ff").unwrap()
            ],
            play_grid: server.load::<Image>("images/grid_outer.png"),
            cursor: server.load::<Image>("images/default_cursor.png"),
            main_font: server.load::<Font>("fonts/Emulogic-zrEw.ttf"),
            play_grade_box: server.load::<Image>("images/play_grade_box.png"),
            maps_path: "/maps/".to_owned(),
            test_map: Map {
                audio: server.load::<AudioSource>("maps/ss_archive_belowamateur_-_birb.mp3"),
                notes: server.load::<NoteData>("maps/ss_archive_belowamateur_-_birb.txt"),
                title: "birb".to_owned(),
                artist: "BelowAmateur".to_owned(),
                mapper: "SS Archive".to_owned()
            },
        };

        commands.insert_resource(assets);

        state.set(GameState::Menu);
    }

    /*fn on_update(mut globals: ResMut<GlobalAssets>, mut state: ResMut<NextState<GameState>>, server: ResMut<AssetServer>) {
        let audio_state = server.load_state(globals.test_map_audio);
        let notes_state = server.load_state(globals.test_map_notes);
        if audio_state == LoadState::Loaded && notes_state == LoadState::Loaded {
            globals.test_map.audio = globals.test_map_audio
            state.set(GameState::Menu);
        }
    }*/
}