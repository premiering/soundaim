use bevy::{app::App, asset::{AssetApp, AssetMetaCheck}, diagnostic::FrameTimeDiagnosticsPlugin, DefaultPlugins};
use bevy_kira_audio::AudioPlugin;
use bevy_mod_billboard::plugin::BillboardPlugin;
use bevy_obj::ObjPlugin;
use debug::GameDebugPlugin;
use map::{NoteData, V1NoteDataLoader};
use state::StatePlugin;

mod state;
mod menu;
mod startup;
mod map;
mod play;
mod debug;

fn main() {
    App::new()
        .insert_resource(AssetMetaCheck::Never)
        .add_plugins((
            DefaultPlugins,
            StatePlugin,
            ObjPlugin,
            AudioPlugin,
            FrameTimeDiagnosticsPlugin,
            GameDebugPlugin,
            BillboardPlugin,
        ))
        .init_asset::<NoteData>()
        .init_asset_loader::<V1NoteDataLoader>()
        .run();
}