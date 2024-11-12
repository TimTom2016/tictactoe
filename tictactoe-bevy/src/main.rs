use bevy::{
    color::palettes::{
        css::{BLACK, GRAY, WHITE},
        tailwind::GRAY_50,
    },
    prelude::*,
    sprite::MaterialMesh2dBundle,
    utils::{HashMap, HashSet},
};
use menu::MenuPlugin;
use tictactoe_logic::{
    grid::{FieldStates, Grid},
    minimax::MiniMax,
};
mod menu;
#[derive(Event)]
struct Click(pub Vec2, pub f32);

#[derive(Event)]
struct Pressed;

#[derive(Event)]
struct KIMove;

#[derive(Component)]
struct Tile {
    pos: Vec2,
    size: f32,
    pub index: u32,
}
#[derive(States, Default, Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum AppState {
    #[default]
    InMenu,
    InGame,
}
#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum PlayerChoice {
    X,
    O,
}

pub enum MoveResult {
    Won(u32),
    Moved(u32),
}

impl PlayerChoice {
    pub fn opposite(&self) -> Self {
        match self {
            PlayerChoice::X => PlayerChoice::O,
            PlayerChoice::O => PlayerChoice::X,
        }
    }
    pub fn to_field_states(&self) -> FieldStates {
        match self {
            PlayerChoice::X => FieldStates::Player1,
            PlayerChoice::O => FieldStates::Player2,
        }
    }
}
#[derive(Resource)]
pub struct GameData {
    pub player: PlayerChoice,
    pub grid: Grid,
    pub moves: u32,
    pub won: bool,
}
#[derive(Debug, PartialEq, Eq)]
pub enum WinPossibilities {
    XWon,
    OWon,
    Tie,
    None,
}

impl GameData {
    pub fn make_move(&mut self, grid_index: u32) -> Result<MoveResult, ()> {
        let moves = self.who_moves();
        if moves != self.player {
            return Err(());
        }
        let current_state = moves.to_field_states();

        // Try to make the move
        if self
            .grid
            .set_elem(grid_index as usize, current_state)
            .is_some()
        {
            self.moves += 1;

            // Check if the current move resulted in a win
            if self.grid.check_win(current_state) {
                self.won = true;
                Ok(MoveResult::Won(grid_index))
            } else {
                Ok(MoveResult::Moved(grid_index))
            }
        } else {
            Err(())
        }
    }

    pub fn who_moves(&self) -> PlayerChoice {
        // If moves is even, X moves; if odd, O moves
        if self.moves % 2 == 0 {
            PlayerChoice::X
        } else {
            PlayerChoice::O
        }
    }
    pub fn make_ki_move(&mut self) -> Result<MoveResult, ()> {
        let mut minimax = MiniMax::new(&self.grid);
        let new_grid = self.grid.clone();
        self.grid = minimax.calculate();
        self.moves += 1;
        let mut changed_index = 0;
        let mut same = true;
        for (index, (old, new)) in self
            .grid
            .clone()
            .into_iter()
            .zip(new_grid.into_iter())
            .enumerate()
        {
            if old != new {
                // Found the move, now make it through the regular move system
                changed_index = index as u32;
                same = false;
            }
        }
        if same {
            return Err(());
        }
        // Check if KI won
        if self.grid.check_win(FieldStates::Player2) {
            self.won = true;
            Ok(MoveResult::Won(changed_index))
        } else {
            Ok(MoveResult::Moved(changed_index))
        }
    }
    pub fn check_game_state(&self) -> WinPossibilities {
        // Check if X won
        if self.grid.check_win(PlayerChoice::X.to_field_states()) {
            return WinPossibilities::XWon;
        }

        // Check if O won
        if self.grid.check_win(PlayerChoice::O.to_field_states()) {
            return WinPossibilities::OWon;
        }

        // Check for tie (grid is full)
        if self.grid.is_full() {
            return WinPossibilities::Tie;
        }

        // Game is still ongoing
        WinPossibilities::None
    }
}

#[derive(Component)]
struct Symbol {
    is_x: bool,
}

#[derive(Component)]
struct OnGameScreen;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(MenuPlugin)
        .add_systems(Startup, setup)
        .add_systems(OnEnter(AppState::InGame), setup_game)
        .add_systems(Update, handle_click.run_if(in_state(AppState::InGame)))
        .add_event::<Click>()
        .add_event::<Pressed>()
        .add_event::<KIMove>()
        .add_systems(
            Update,
            (handle_ki_move).chain().run_if(in_state(AppState::InGame)),
        )
        .init_resource::<SpatialIndex>()
        .init_state::<AppState>()
        .observe(
            |trigger: Trigger<Click>,
             tiles: Query<&Tile>,
             index: Res<SpatialIndex>,
             mut commands: Commands| {
                // You can access the trigger data via the `Observer`
                let event = trigger.event();
                // Access resources
                for e in index.get_nearby(event.0) {
                    // Run queries
                    let mine = tiles.get(e).unwrap();
                    if mine.pos.distance(event.0) < mine.size + event.1 {
                        // And queue commands, including triggering additional events
                        // Here we trigger the `Explode` event for entity `e`
                        commands.trigger_targets(Pressed, e);
                    }
                }
            },
        )
        .observe(on_add_tile)
        .observe(on_remove_tile)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
fn setup_game(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut game_data: ResMut<GameData>,
) {
    let mut observer = Observer::new(tile_pressed);
    let cols = 3;
    let rows = 3;
    let offset = Vec2::new(
        rows as f32 * 64.0 - 64.0, // Total width / 2
        cols as f32 * 64.0 - 64.0, // Total height / 2
    );
    for y in 0..cols {
        for x in 0..rows {
            commands.spawn((MaterialMesh2dBundle {
                mesh: meshes.add(Rectangle::default()).into(),
                transform: Transform::default()
                    .with_scale(Vec3::splat(128.))
                    .with_translation(Vec3::new(
                        x as f32 * 128.0 - offset.x,
                        y as f32 * 128.0 - offset.y,
                        0.0,
                    )),
                material: materials.add(Color::from(WHITE)),
                ..default()
            },));
            observer.watch_entity(
                commands
                    .spawn((
                        MaterialMesh2dBundle {
                            mesh: meshes.add(Rectangle::default()).into(),
                            transform: Transform::default()
                                .with_scale(Vec3::splat(120.))
                                .with_translation(Vec3::new(
                                    x as f32 * 128.0 - offset.x,
                                    y as f32 * 128.0 - offset.y,
                                    0.1,
                                )),
                            material: materials.add(Color::srgb_u8(43, 44, 47)),
                            ..default()
                        },
                        Tile {
                            pos: Vec2::new(x as f32 * 128. - offset.x, y as f32 * 128. - offset.y),
                            size: 120.,
                            index: x + 6 - (y * 3),
                        },
                    ))
                    .id(),
            );
        }
    }
    commands.spawn(observer);
    if matches!(game_data.player, PlayerChoice::O) {
        commands.add(move |world: &mut World| {
            world.send_event(KIMove);
        });
    }
}

fn handle_click(
    mouse_button_input: Res<ButtonInput<MouseButton>>,
    camera: Query<(&Camera, &GlobalTransform)>,
    windows: Query<&Window>,
    mut commands: Commands,
) {
    let (camera, camera_transform) = camera.single();
    if let Some(pos) = windows
        .single()
        .cursor_position()
        .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
        .map(|ray| ray.origin.truncate())
    {
        if mouse_button_input.just_pressed(MouseButton::Left) {
            commands.trigger(Click(pos, 1.0));
        }
    }
}

fn on_add_tile(
    trigger: Trigger<OnAdd, Tile>,
    query: Query<&Tile>,
    mut index: ResMut<SpatialIndex>,
) {
    let mine = query.get(trigger.entity()).unwrap();
    let tile = (
        (mine.pos.x / CELL_SIZE).floor() as i32,
        (mine.pos.y / CELL_SIZE).floor() as i32,
    );
    index.map.entry(tile).or_default().insert(trigger.entity());
}

// Remove despawned mines from our index
fn on_remove_tile(
    trigger: Trigger<OnRemove, Tile>,
    query: Query<&Tile>,
    mut index: ResMut<SpatialIndex>,
) {
    let mine = query.get(trigger.entity()).unwrap();
    let tile = (
        (mine.pos.x / CELL_SIZE).floor() as i32,
        (mine.pos.y / CELL_SIZE).floor() as i32,
    );
    index.map.entry(tile).and_modify(|set| {
        set.remove(&trigger.entity());
    });
}

fn tile_pressed(
    trigger: Trigger<Pressed>,
    query: Query<&Tile>,
    mut game_data: ResMut<GameData>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let id = trigger.entity();
    let Some(entity) = commands.get_entity(id) else {
        return;
    };

    let tile = query.get(id).unwrap();
    if let Ok(result) = game_data.make_move(tile.index) {
        match result {
            MoveResult::Moved(index) => {
                spawn_symbol(
                    &mut commands,
                    &mut meshes,
                    &mut materials,
                    tile.pos,
                    game_data.player.clone(),
                );
                // Trigger KI move if it's not game over
                commands.add(move |world: &mut World| {
                    world.send_event(KIMove);
                });
            }
            MoveResult::Won(index) => {
                spawn_symbol(
                    &mut commands,
                    &mut meshes,
                    &mut materials,
                    tile.pos,
                    game_data.player.clone(),
                );
                // Handle win condition
            }
        }
    }
}

#[derive(Resource, Default)]
struct SpatialIndex {
    map: HashMap<(i32, i32), HashSet<Entity>>,
}

/// Cell size has to be bigger than any `TriggerMine::radius`
const CELL_SIZE: f32 = 48.0;

impl SpatialIndex {
    // Lookup all entities within adjacent cells of our spatial index
    fn get_nearby(&self, pos: Vec2) -> Vec<Entity> {
        let tile = (
            (pos.x / CELL_SIZE).floor() as i32,
            (pos.y / CELL_SIZE).floor() as i32,
        );
        let mut nearby = Vec::new();
        for x in -1..2 {
            for y in -1..2 {
                if let Some(entities) = self.map.get(&(tile.0 + x, tile.1 + y)) {
                    nearby.extend(entities.iter());
                }
            }
        }
        nearby
    }
}

pub fn despawn_screen<T: Component>(to_despawn: Query<Entity, With<T>>, mut commands: Commands) {
    for entity in &to_despawn {
        commands.entity(entity).despawn_recursive();
    }
}

fn handle_ki_move(
    mut game_data: ResMut<GameData>,
    query: Query<&Tile>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut ki_move_events: EventReader<KIMove>,
) {
    for _ in ki_move_events.read() {
        if let Ok(result) = game_data.make_ki_move() {
            match result {
                MoveResult::Moved(index) => {
                    if let Some(tile) = query.iter().find(|t| t.index == index) {
                        spawn_symbol(
                            &mut commands,
                            &mut meshes,
                            &mut materials,
                            tile.pos,
                            game_data.player.opposite(),
                        );
                    }
                }
                MoveResult::Won(index) => {
                    if let Some(tile) = query.iter().find(|t| t.index == index) {
                        spawn_symbol(
                            &mut commands,
                            &mut meshes,
                            &mut materials,
                            tile.pos,
                            game_data.player.opposite(),
                        );
                    }
                }
            }
        }
    }
}

fn spawn_symbol(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    position: Vec2,
    player_choice: PlayerChoice,
) {
    match player_choice {
        PlayerChoice::X => spawn_x(commands, meshes, materials, position),
        PlayerChoice::O => spawn_o(commands, meshes, materials, position),
    }
}

fn spawn_x(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    position: Vec2,
) {
    let symbol_size = 100.0;

    // Spawn first line of X (diagonal from top-left to bottom-right)
    commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes.add(Rectangle::default()).into(),
            transform: Transform::from_xyz(position.x, position.y, 0.2)
                .with_rotation(Quat::from_rotation_z(45f32.to_radians()))
                .with_scale(Vec3::new(symbol_size, symbol_size / 8.0, 1.0)),
            material: materials.add(Color::from(BLACK)),
            ..default()
        },
        Symbol { is_x: true },
        OnGameScreen,
    ));

    // Spawn second line of X (diagonal from top-right to bottom-left)
    commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes.add(Rectangle::default()).into(),
            transform: Transform::from_xyz(position.x, position.y, 0.2)
                .with_rotation(Quat::from_rotation_z(-45f32.to_radians()))
                .with_scale(Vec3::new(symbol_size, symbol_size / 8.0, 1.0)),
            material: materials.add(Color::from(BLACK)),
            ..default()
        },
        Symbol { is_x: true },
        OnGameScreen,
    ));
}

fn spawn_o(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    position: Vec2,
) {
    let symbol_size = 100.0;

    commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes.add(Circle::new(symbol_size / 2.0)).into(),
            transform: Transform::from_xyz(position.x, position.y, 0.2)
                .with_scale(Vec3::splat(1.0)),
            material: materials.add(Color::from(GRAY_50)),
            ..default()
        },
        Symbol { is_x: false },
        OnGameScreen,
    ));
}
