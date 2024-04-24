use bevy::{app::{Plugin, Update}, diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin}, ecs::{query::With, system::{Query, Res}}, window::{PrimaryWindow, Window}};

pub struct GameDebugPlugin;

impl Plugin for GameDebugPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        //env::set_var("RUST_BACKTRACE", "full");
        app.add_systems(Update, |diag: Res<DiagnosticsStore>, mut window_query: Query<&mut Window, With<PrimaryWindow>>| {
            let fps = diag.get(&FrameTimeDiagnosticsPlugin::FPS).and_then(|fps| fps.smoothed());
            let window_res = window_query.get_single_mut();
            if fps.is_some() && window_res.is_ok() {
                window_res.unwrap().title = "soundaim | fps: ".to_owned() + &fps.unwrap().to_string();
            }
        });
    }
}