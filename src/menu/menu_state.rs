use bevy::{app::{App, AppExit, Plugin, Update}, core_pipeline::core_2d::Camera2dBundle, ecs::{component::Component, entity::Entity, event::Events, query::{Changed, With, Without}, schedule::{common_conditions::in_state, IntoSystemConfigs, NextState, OnEnter, OnExit}, system::{Commands, EntityCommands, Query, Res, ResMut, Resource}}, hierarchy::{BuildChildren, ChildBuilder, DespawnRecursiveExt}, render::{camera::{Camera, ClearColorConfig}, color::Color}, text::{Text, TextSection, TextStyle}, ui::{node_bundles::{ButtonBundle, NodeBundle, TextBundle}, widget::Button, AlignItems, AlignSelf, BackgroundColor, FlexDirection, FlexWrap, Interaction, JustifyContent, JustifyItems, PositionType, Style, UiRect, Val}, utils::default};

use crate::{play::play_state::{MapLoadPlayResource, PlayStateData}, startup::GlobalAssets, state::GameState};

pub struct MenuStatePlugin;

#[derive(Component)]
pub struct OnMenu;

#[derive(Component)]
pub struct TestPlayButton;

#[derive(Component)]
pub struct QuitGameButton;

impl Plugin for MenuStatePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Menu), build_menu);
        app.add_systems(OnExit(GameState::Menu), cleanup_menu);
        app.add_systems(Update, on_test_play.run_if(in_state(GameState::Menu)));
        app.add_systems(Update, on_quit_game.run_if(in_state(GameState::Menu)));
    }
}

fn build_menu(globals: ResMut<GlobalAssets>, mut commands: Commands) {
    commands.spawn((NodeBundle {
        style: Style {
            width: Val::Percent(100.),
            height: Val::Percent(100.),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            flex_direction: FlexDirection::Column,
            ..default()
        },
        ..default()
    }, OnMenu)).with_children(|builder| {
        //background_color: BackgroundColor(Color::rgb(0.05, 0.05, 0.05)),
        builder.spawn(TextBundle {
            style: Style {
                margin: UiRect::px(10., 10., 10., 10.),
                ..default()
            },
            text: Text::from_sections(
                [
                    TextSection::new("soundaim: ", TextStyle {
                        font_size: 30.,
                        color: Color::rgb(0.8, 0.05, 0.8),
                        ..default()
                    }),
                    TextSection::new("a Sound Space client!", TextStyle {
                        font_size: 30.,
                        color: Color::rgb(1., 1., 1.),
                        ..default()
                    })
                ]),
            ..default()
        });
        builder.spawn((ButtonBundle {
            style: Style {
                width: Val::Px(400.),
                height: Val::Px(70.),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                margin: UiRect::px(10., 10., 10., 10.),
                ..default()
            },
            background_color: BackgroundColor(Color::rgb(0.5, 0.05, 0.7)),
            ..default()
        }, TestPlayButton)).with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                "Play test map (BelowAmateur - birb)",
                TextStyle {
                    font_size: 30.0,
                    color: Color::rgb(0.9, 0.9, 0.9),
                    ..default()
                },
            ));
        });
        builder.spawn((ButtonBundle {
            style: Style {
                width: Val::Px(400.),
                height: Val::Px(70.),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                margin: UiRect::px(10., 10., 10., 10.),
                ..default()
            },
            background_color: BackgroundColor(Color::rgb(0.7, 0.15, 0.0)),
            ..default()
        }, QuitGameButton)).with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                "Quit game",
                TextStyle {
                    font_size: 30.0,
                    color: Color::rgb(0.9, 0.9, 0.9),
                    ..default()
                },
            ));
        });
    });

    commands.spawn((Camera2dBundle {
        camera: Camera {
            clear_color: ClearColorConfig::Custom(Color::rgb(0., 0., 0.)),
            ..default()
        },
        ..default()
    }, OnMenu));
}

fn on_test_play(globals: ResMut<GlobalAssets>, mut commands: Commands, mut interaction_query: Query<&Interaction, (Changed<Interaction>, With<Button>, With<TestPlayButton>)>) {
    for (interaction) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                let load_play = MapLoadPlayResource::create_loaded(globals.test_map.clone());
                commands.insert_resource(load_play);
            }
            Interaction::Hovered => {

            }
            Interaction::None => {

            }
        }
    }
}

fn on_quit_game(mut interaction_query: Query<&Interaction, (Changed<Interaction>, With<Button>, With<QuitGameButton>, Without<TestPlayButton>)>, mut exit: ResMut<Events<AppExit>>) {
    for interaction in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                exit.send(AppExit);
            }
            Interaction::Hovered => {

            }
            Interaction::None => {

            }
        }
    }
}

fn cleanup_menu(mut commands: Commands, query: Query<Entity, With<OnMenu>>) {
    for ent in query.iter() {
        commands.entity(ent).despawn_recursive();
    }
}