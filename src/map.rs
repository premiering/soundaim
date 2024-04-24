use std::{io::{BufRead, BufReader}, str::from_utf8};

use bevy::{asset::{Asset, AssetLoader, AsyncReadExt, Handle}, reflect::TypePath, utils::{thiserror::Error, BoxedFuture}};
use bevy_kira_audio::AudioSource;

#[derive(Clone, Copy, PartialEq)]
pub struct Note {
    pub hit_ms: i128,
    pub x: f32,
    pub y: f32,
    pub size: f32
}

#[derive(Clone, Default)]
pub struct Map {
    pub title: String,
    pub artist: String,
    pub mapper: String,
    pub notes: Handle<NoteData>,
    pub audio: Handle<AudioSource>
}

#[derive(Asset, TypePath, Default, Clone)]
pub struct NoteData(pub Vec<Note>);

#[derive(Default)]
pub struct V1NoteDataLoader;

#[non_exhaustive]
#[derive(Debug, Error)]
pub enum V1NoteDataLoaderError {
    #[error("Could not load asset, invalid format.")]
    Invalid
}

impl AssetLoader for V1NoteDataLoader {
    type Asset = NoteData;

    type Settings = ();

    type Error = V1NoteDataLoaderError;

    fn load<'a>(
        &'a self,
        reader: &'a mut bevy::asset::io::Reader,
        _settings: &'a Self::Settings,
        _load_context: &'a mut bevy::asset::LoadContext,
    ) -> BoxedFuture<'a, Result<NoteData, Self::Error>> {
        Box::pin(async move {
            let mut bytes = Vec::new();
            reader.read_to_end(&mut bytes).await.unwrap();
            let strn = from_utf8(&bytes).unwrap().to_string();
            let reader = BufReader::new(strn.as_bytes());
            let mut split = reader.split(b',');
            split.next();//Skip the first one since it's just the name/roblox id
            let mut notes: Vec<Note> = vec![];
            loop {
                let next = split.next();
                if next.is_none() {
                    break;
                }
                let next_unwrapped = next.unwrap().unwrap();
                let mut note_data = BufReader::new(next_unwrapped.as_slice()).split(b'|');
                // - 1.0 to convert from 0-2 to -1 - 1
                let x = bufreader_to_f32(note_data.next().unwrap().unwrap()) - 1.;
                let y = bufreader_to_f32(note_data.next().unwrap().unwrap()) - 1.;
                let hit_ms = bufreader_to_i128(note_data.next().unwrap().unwrap());
                notes.push(Note {
                    hit_ms,
                    x,
                    y,
                    size: 1.0,
                });
            }
            Ok(NoteData(notes))
        })
    }

    fn extensions(&self) -> &[&str] {
        &["txt"]
    }
}

fn bufreader_to_f32(buf: Vec<u8>) -> f32 {
    return String::from_utf8(buf).unwrap().parse().unwrap();
}

fn bufreader_to_i128(buf: Vec<u8>) -> i128 {
    return String::from_utf8(buf).unwrap().parse().unwrap();
}