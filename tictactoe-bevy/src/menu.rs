use bevy::{
    color::palettes::{
        css::{BLACK, RED},
        tailwind::{BLUE_400, BLUE_50, BLUE_700, BLUE_800, BLUE_900},
    },
    prelude::*,
};
use tictactoe_logic::grid::Grid;

use crate::{despawn_screen, AppState, GameData, PlayerChoice};
pub struct MenuPlugin;

#[derive(States, Default, Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
enum MenuState {
    #[default]
    InTransition,
    InMenu,
}

#[derive(Component)]
struct OnMenuScreen;

#[derive(Component)]
enum MenuButtonAction {
    PlayX,
    PlayO,
    Exit,
}

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<MenuState>()
            .add_systems(OnEnter(AppState::InMenu), setup_menu)
            .add_systems(Update, (menu_action).run_if(in_state(MenuState::InMenu)))
            .add_systems(OnExit(AppState::InMenu), despawn_screen::<OnMenuScreen>);
    }
}

fn setup_menu(mut menu_state: ResMut<NextState<MenuState>>, mut commands: Commands) {
    menu_state.set(MenuState::InMenu);
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..Default::default()
                },
                ..Default::default()
            },
            OnMenuScreen,
        ))
        .with_children(|parent| {
            parent
                .spawn(NodeBundle {
                    border_radius: BorderRadius::all(Val::Percent(5.0)),
                    background_color: BackgroundColor(BLUE_700.into()),
                    border_color: BorderColor(BLUE_800.into()),
                    style: Style {
                        border: UiRect::all(Val::Px(1.0)),
                        padding: UiRect::all(Val::Px(5.0)),
                        min_width: Val::Vw(20.0),
                        min_height: Val::Vh(20.0),
                        flex_direction: FlexDirection::Column,
                        justify_content: JustifyContent::SpaceBetween,
                        align_items: AlignItems::Center,
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .with_children(|parent| {
                    parent
                        .spawn(NodeBundle {
                            style: Style {
                                justify_content: JustifyContent::SpaceEvenly,
                                width: Val::Percent(100.0),
                                ..Default::default()
                            },
                            ..Default::default()
                        })
                        .with_children(|parent| {
                            parent
                                .spawn((
                                    ButtonBundle {
                                        background_color: BackgroundColor(BLUE_400.into()),
                                        border_radius: BorderRadius::all(Val::Percent(20.0)),
                                        border_color: BorderColor(BLACK.into()),
                                        style: Style {
                                            border: UiRect::all(Val::Px(1.0)),
                                            padding: UiRect::all(Val::Px(20.0))
                                                .with_top(Val::Px(5.0))
                                                .with_bottom(Val::Px(5.0)),
                                            justify_self: JustifySelf::Start,
                                            align_self: AlignSelf::Start,
                                            ..Default::default()
                                        },
                                        ..Default::default()
                                    },
                                    MenuButtonAction::PlayX,
                                ))
                                .with_children(|parent| {
                                    parent.spawn(TextBundle::from_section(
                                        "Play X",
                                        TextStyle {
                                            ..Default::default()
                                        },
                                    ));
                                });
                            parent
                                .spawn((
                                    ButtonBundle {
                                        background_color: BackgroundColor(BLUE_400.into()),
                                        border_radius: BorderRadius::all(Val::Percent(20.0)),
                                        border_color: BorderColor(BLACK.into()),
                                        style: Style {
                                            border: UiRect::all(Val::Px(1.0)),
                                            padding: UiRect::all(Val::Px(20.0))
                                                .with_top(Val::Px(5.0))
                                                .with_bottom(Val::Px(5.0)),
                                            justify_self: JustifySelf::Start,
                                            align_self: AlignSelf::Start,
                                            ..Default::default()
                                        },
                                        ..Default::default()
                                    },
                                    MenuButtonAction::PlayO,
                                ))
                                .with_children(|parent| {
                                    parent.spawn(TextBundle::from_section(
                                        "Play O",
                                        TextStyle {
                                            ..Default::default()
                                        },
                                    ));
                                });
                        });
                })
                .with_children(|parent| {
                    parent
                        .spawn((
                            ButtonBundle {
                                background_color: BackgroundColor(RED.into()),
                                border_radius: BorderRadius::all(Val::Percent(20.0)),
                                border_color: BorderColor(BLACK.into()),
                                style: Style {
                                    border: UiRect::all(Val::Px(1.0)),
                                    padding: UiRect::all(Val::Px(20.0))
                                        .with_top(Val::Px(5.0))
                                        .with_bottom(Val::Px(5.0)),
                                    ..Default::default()
                                },
                                ..Default::default()
                            },
                            MenuButtonAction::Exit,
                        ))
                        .with_children(|parent| {
                            parent.spawn(TextBundle::from_section(
                                "Exit",
                                TextStyle {
                                    ..Default::default()
                                },
                            ));
                        });
                });
        });
}

fn menu_action(
    interaction_query: Query<
        (&Interaction, &MenuButtonAction),
        (Changed<Interaction>, With<Button>),
    >,
    mut app_state: ResMut<NextState<AppState>>,
    mut menu_state: ResMut<NextState<MenuState>>,
    mut exit: EventWriter<bevy::app::AppExit>,
    mut commands: Commands,
) {
    for (interaction, menu_button_action) in &interaction_query {
        if *interaction == Interaction::Pressed {
            match menu_button_action {
                MenuButtonAction::PlayX => {
                    info!("Playing as X");
                    commands.insert_resource(GameData {
                        player: PlayerChoice::X,
                        grid: Grid::new(3, 3),
                        moves: 0,
                        won: false,
                    });
                    menu_state.set(MenuState::InTransition);
                    app_state.set(AppState::InGame);
                }
                MenuButtonAction::PlayO => {
                    info!("Playing as O");
                    commands.insert_resource(GameData {
                        player: PlayerChoice::O,
                        grid: Grid::new(3, 3),
                        moves: 0,
                        won: false,
                    });
                    menu_state.set(MenuState::InTransition);
                    app_state.set(AppState::InGame);
                }
                MenuButtonAction::Exit => {
                    exit.send(bevy::app::AppExit::Success);
                }
            }
        }
    }
}

// fn button_hover_system(
//     mut buttons: Query<(&Interaction, &mut BackgroundColor), (Changed<Interaction>, With<Button>)>,
// ) {
//     for (interaction, mut color) in &mut buttons {
//         match *interaction {
//             Interaction::Pressed => {
//                 *color = BackgroundColor(BLUE_900.into());
//             }
//             Interaction::Hovered => {
//                 *color = BackgroundColor(BLUE_800.into());
//             }
//             Interaction::None => {
//                 *color = BackgroundColor(BLUE_400.into());
//             }
//         }
//     }
// }
