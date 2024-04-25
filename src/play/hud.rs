use bevy::{asset::{Assets, Handle}, ecs::{component::Component, query::{With, Without}, system::{Commands, Query, ResMut}}, math::{primitives::Rectangle, Vec3}, render::{color::Color, mesh::Mesh, render_asset::RenderAssetUsages, render_resource::{Extent3d, TextureDimension, TextureFormat}, texture::Image}, text::{Font, Text, TextSection, TextStyle}, transform::components::Transform, utils::default};
use bevy_mod_billboard::{BillboardLockAxis, BillboardMeshHandle, BillboardTextBundle, BillboardTextureBundle, BillboardTextureHandle};
use num_format::{Locale, ToFormattedString};

use crate::startup::GlobalAssets;

use super::{cursor::CursorTransformParallax, play_state::{InPlay, PlayStateData}};

pub enum PlayGrade {
    SS,
    A,
    B,
    C,
    D
}

#[derive(Component)]
pub struct LeftPlayGradeText;

#[derive(Component)]
pub struct LeftPanelText;

#[derive(Component)]
pub struct RightPanelText;

#[derive(Component)]
pub struct PlayGrid;

pub fn init_hud(
    mut data: ResMut<PlayStateData>, 
    mut meshes: ResMut<Assets<Mesh>>,
    mut images: ResMut<Assets<Image>>,
    globals: ResMut<GlobalAssets>,
    mut commands: Commands
) {
    // Spawn play grid
    commands.spawn((BillboardTextureBundle {
        transform: Transform::from_translation(Vec3::ZERO).with_scale(Vec3::splat(1.)),
        texture: BillboardTextureHandle(globals.play_grid.clone()),
        mesh: BillboardMeshHandle(meshes.add(Rectangle::new(3., 3.))),
        ..default()
    }, InPlay, PlayGrid,
    CursorTransformParallax {
        parallax_amount: 50.
    }));

    // Song title
    commands.spawn((BillboardTextBundle {
        transform: Transform::from_translation(Vec3::new(0., 1.9, 0.))
            .with_scale(Vec3::splat(0.0065)),
        text: Text::from_sections([
            TextSection {
                value: data.map.title.clone(),
                style: TextStyle {
                    font_size: 24.0,
                    color: Color::rgb(0.851, 0.247, 0.269),
                    ..default()
                }
            },
        ]).with_justify(bevy::text::JustifyText::Center),
        ..default()
    }, InPlay));

    // Left panel text
    commands.spawn((BillboardTextBundle {
        transform: Transform::from_translation(Vec3::new(2.5, 0., -0.3))
            .with_scale(Vec3::splat(0.0025)).looking_at(Vec3::new(0., 0., -10.), Vec3::Y),
        text: build_left_panel_text(
            data.current_combo, 
            data.get_accuracy(), 
            globals.main_font.clone()
        ),
        ..default()
    },
    BillboardLockAxis {
        rotation: true,
        ..default()
    }, InPlay, LeftPanelText));

    // Left play grade text
    commands.spawn((BillboardTextBundle {
        transform: Transform::from_translation(Vec3::new(2.5, 1., -0.3))
            .with_scale(Vec3::splat(0.0025)).looking_at(Vec3::new(0., 1., -10.), Vec3::Y),
        text: build_play_grade_text(PlayGrade::SS, globals.main_font.clone()),
        ..default()
    },
    BillboardLockAxis {
        rotation: true,
        ..default()
    }, InPlay, LeftPlayGradeText));

    // Left play grade box
    commands.spawn((BillboardTextureBundle {
        transform: Transform::from_translation(Vec3::new(2.5, 1., -0.3))
            .looking_at(Vec3::new(0., 1., -10.), Vec3::Y),
        texture: BillboardTextureHandle(globals.play_grade_box.clone()),
        mesh: BillboardMeshHandle(meshes.add(Rectangle::new(1., 1.1))),
        ..default()
    },
    BillboardLockAxis {
        rotation: true,
        ..default()
    }, InPlay));

    // Left panel background
    commands.spawn((BillboardTextureBundle {
        transform: Transform::from_translation(Vec3::new(-2.5, 0., -0.3))
            .looking_at(Vec3::new(0., 0., -10.), Vec3::Y),
        texture: BillboardTextureHandle(images.add(Image::new_fill(Extent3d {
            width: 1_u32,
            height: 1_u32,
            depth_or_array_layers: 1,
        }, TextureDimension::D2, &[1, 1, 1, 50], TextureFormat::Rgba8Unorm, RenderAssetUsages::all()))),
        mesh: BillboardMeshHandle(meshes.add(Rectangle::new(1.3, 3.3))),
        ..default()
    },
    BillboardLockAxis {
        rotation: true,
        ..default()
    }, InPlay));

    // Right panel background
    commands.spawn((BillboardTextureBundle {
        transform: Transform::from_translation(Vec3::new(2.5, 0., -0.3))
            .looking_at(Vec3::new(0., 0., -10.), Vec3::Y),
        texture: BillboardTextureHandle(images.add(Image::new_fill(Extent3d {
            width: 1_u32,
            height: 1_u32,
            depth_or_array_layers: 1,
        }, TextureDimension::D2, &[1, 1, 1, 50], TextureFormat::Rgba8Unorm, RenderAssetUsages::all()))),
        mesh: BillboardMeshHandle(meshes.add(Rectangle::new(1.3, 3.3))),
        ..default()
    },
    BillboardLockAxis {
        rotation: true,
        ..default()
    }, InPlay));

    // Right panel text
    commands.spawn((BillboardTextBundle {
        transform: Transform::from_translation(Vec3::new(-2.5, 0., -0.3))
            .with_scale(Vec3::splat(0.0025)).looking_at(Vec3::new(0., 0., -10.), Vec3::Y),
        text: build_right_panel_text(0, data.misses, data.objects_hit, data.note_data.0.len() as i128, globals.main_font.clone()),
        ..default()
    },
    BillboardLockAxis {
        rotation: true,
        ..default()
    }, InPlay, RightPanelText)); 
}

pub fn on_update(
    mut data: ResMut<PlayStateData>, 
    mut q_left_panel_text: Query<&mut Text, With<LeftPanelText>>,
    mut q_right_panel_text: Query<&mut Text, (Without<LeftPanelText>, With<RightPanelText>)>,
    mut q_play_grade_text: Query<&mut Text, (With<LeftPlayGradeText>, Without<RightPanelText>, Without<LeftPanelText>)>,
    globals: ResMut<GlobalAssets>
) {
    // Update the left panel info
    let mut left_panel_text = q_left_panel_text.get_single_mut().unwrap();
    *left_panel_text = build_left_panel_text(
        data.current_combo, 
        data.get_accuracy(),
        globals.main_font.clone()
    );

    // Update the right panel info
    let mut right_panel_text = q_right_panel_text.get_single_mut().unwrap();
    *right_panel_text = build_right_panel_text(calc_score(data.objects_hit, data.max_combo, data.get_accuracy()), data.misses, data.objects_hit, data.note_data.0.len() as i128, globals.main_font.clone());

    // Update play grade
    let mut play_grade_text = q_play_grade_text.get_single_mut().unwrap();
    *play_grade_text = build_play_grade_text(calc_play_grade(data.get_accuracy()), globals.main_font.clone());
}

fn build_left_panel_text(combo: i128, accuracy: f32, font: Handle<Font>) -> Text {
    Text::from_sections([
        TextSection {
            value: "\nCOMBO".to_string(),
            style: TextStyle {
                font_size: 96.0,
                color: Color::WHITE,
                font: font.clone(),
            },
        },
        TextSection {
            value: ("\n".to_owned() + &combo.to_string()),
            style: TextStyle {
                font_size: 96.0,
                color: Color::WHITE,
                font: font.clone(),
            }
        },
        TextSection {
            value: "\n".to_owned() + &format!("{:.1$}", accuracy, 1) + "%\n",
            style: TextStyle {
                font_size: 96.0,
                color: get_accuracy_color(calc_play_grade(accuracy)),
                font: font.clone(),
            }
        },
    ]).with_justify(bevy::text::JustifyText::Center)
}

fn build_right_panel_text(score: i128, misses: i128, hits: i128, max_hits: i128, font: Handle<Font>) -> Text {
    Text::from_sections([
        TextSection {
            value: score.to_formatted_string(&Locale::en),
            style: TextStyle {
                font_size: 80.0,
                color: Color::YELLOW,
                font: font.clone(),
            },
        },
        TextSection {
            value: "\n\nMISSES".to_string(),
            style: TextStyle {
                font_size: 80.0,
                color: Color::WHITE,
                font: font.clone(),
            },
        },
        TextSection {
            value: ("\n".to_owned() + &misses.to_string()),
            style: TextStyle {
                font_size: 80.0,
                color: Color::WHITE,
                font: font.clone(),
            }
        },
        TextSection {
            value: "\nNOTES".to_string(),
            style: TextStyle {
                font_size: 80.0,
                color: Color::WHITE,
                font: font.clone(),
            }
        },
        TextSection {
            value: ("\n".to_owned() + &hits.to_string() + "/" + &max_hits.to_string()),
            style: TextStyle {
                font_size: 80.0,
                color: Color::WHITE,
                font: font.clone(),
            }
        },
    ]).with_justify(bevy::text::JustifyText::Center)
}

fn calc_play_grade(accuracy: f32) -> PlayGrade {
    if accuracy == 100. {
        return PlayGrade::SS;
    } else if accuracy > 93.5 {
        return PlayGrade::A;
    } else if accuracy > 87.5 {
        return PlayGrade::B;
    } else if accuracy > 78.5 {
        return PlayGrade::C;
    } else {
        return PlayGrade::D;
    }
}

fn calc_score(hits: i128, max_combo: i128, accuracy: f32) -> i128 {
    return ((hits * 5 * max_combo) as f32 * (0.8 + accuracy / 500.)) as i128;
}

fn build_play_grade_text(grade: PlayGrade, font: Handle<Font>) -> Text {
    Text::from_section(
        match grade {
            PlayGrade::SS => {
                "SS".to_string()
            },
            PlayGrade::A => {
                "A".to_string()
            },
            PlayGrade::B => {
                "B".to_string()
            },
            PlayGrade::C => {
                "C".to_string()
            },
            PlayGrade::D => {
                "D".to_string()
            },
        },
        TextStyle {
            font_size: 146.0,
            color: get_play_grade_color(grade),
            font: font.clone(),
        },
    )
}

fn get_play_grade_color(grade: PlayGrade) -> Color {
    match grade {
        PlayGrade::SS => {
            Color::hex("#e329d7").unwrap()
        },
        PlayGrade::A => {
            Color::hex("#03ff2d").unwrap()
        },
        PlayGrade::B => {
            Color::hex("#ffd500").unwrap()
        },
        PlayGrade::C => {
            Color::hex("#d48806").unwrap()
        },
        PlayGrade::D => {
            Color::hex("#c41d31").unwrap()
        },
    }
}

fn get_accuracy_color(grade: PlayGrade) -> Color {
    match grade {
        PlayGrade::SS => {
            Color::hex("#e329d7").unwrap()
        },
        PlayGrade::A => {
            Color::hex("#03ff2d").unwrap()
        },
        PlayGrade::B => {
            Color::hex("#ffd500").unwrap()
        },
        PlayGrade::C => {
            Color::hex("#ffd500").unwrap()
        },
        PlayGrade::D => {
            Color::hex("#c41d31").unwrap()
        },
    }
}
