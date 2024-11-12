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
mod menu;
#[derive(Event)]
struct Click(pub Vec2, pub f32);

#[derive(Event)]
struct Pressed;

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

#[derive(Component)]
struct OnGameScreen;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(MenuPlugin)
        .add_systems(Startup, setup)
        .add_systems(OnEnter(AppState::InGame), setup_game)
        .add_systems(Update, handle_click.run_if(in_state(AppState::InGame)))
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
) {
    let mut observer = Observer::new(tile_pressed);
    for y in 0..3 {
        for x in 0..3 {
            commands.spawn((MaterialMesh2dBundle {
                mesh: meshes.add(Rectangle::default()).into(),
                transform: Transform::default()
                    .with_scale(Vec3::splat(128.))
                    .with_translation(Vec3::new(x as f32 * 128.0, y as f32 * 128.0, 0.0)),
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
                                    x as f32 * 128.0,
                                    y as f32 * 128.0,
                                    0.1,
                                )),
                            material: materials.add(Color::srgb_u8(43, 44, 47)),
                            ..default()
                        },
                        Tile {
                            pos: Vec2::new(x as f32 * 128., y as f32 * 128.),
                            size: 120.,
                            index: x + 6 - (y * 3),
                        },
                    ))
                    .id(),
            );
        }
    }
    commands.spawn(observer);
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

fn tile_pressed(trigger: Trigger<Pressed>, query: Query<&Tile>, mut commands: Commands) {
    // If a triggered event is targeting a specific entity you can access it with `.entity()`
    let id = trigger.entity();
    let Some(mut entity) = commands.get_entity(id) else {
        return;
    };
    info!("Boom! {:?} exploded.", query.get(id).unwrap().index);
}

#[derive(Resource, Default)]
struct SpatialIndex {
    map: HashMap<(i32, i32), HashSet<Entity>>,
}

/// Cell size has to be bigger than any `TriggerMine::radius`
const CELL_SIZE: f32 = 64.0;

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
