use bevy::{app::Plugin, ecs::schedule::States};

use crate::{menu::menu_state::MenuStatePlugin, play::play_state::PlayStatePlugin, startup::StartupPlugin};

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub (crate) enum GameState {
    #[default] Startup,
    Menu,
    Play
}

pub struct StatePlugin;

impl StatePlugin {

}

impl Plugin for StatePlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.init_state::<GameState>().add_plugins((
            StartupPlugin,
            MenuStatePlugin,
            PlayStatePlugin
        ));
    }
}