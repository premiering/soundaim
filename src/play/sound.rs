use bevy::{ecs::system::{Res, ResMut}, time::Time};
use bevy_kira_audio::prelude::*;

use super::play_state::PlayStateData;

pub fn init_sound(mut data: ResMut<PlayStateData>, 
    time: ResMut<Time>, 
    kira_audio: Res<Audio>) {
    data.start_time = time.elapsed();
    let mut command = kira_audio.play(data.map.audio.clone());
    //command.fade_in(AudioTween::new(Duration::from_secs(2), AudioEasing::OutPowi(2)));
    command.with_playback_rate(data.play_speed as f64);
    data.song = command.handle().clone();
}