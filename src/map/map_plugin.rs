use bevy::prelude::*;
use noise::{NoiseFn, Perlin};
use rand::Rng;
use test_game::{TILE_LAYER_1_Z, TILE_LAYER_2_Z, TILE_LAYER_3_Z};

const MAP_SIZE: (usize, usize) = (128, 128);
const TILE_SIZE: Vec2 = Vec2::new(32.0, 32.0);
const GRID_COL: usize = 56;
const GRID_ROW: usize = 24;
const START_X: f32 = -(MAP_SIZE.0 as f32 * TILE_SIZE.x) / 2.0;
const START_Y: f32 = -(MAP_SIZE.1 as f32 * TILE_SIZE.y) / 2.0;
const PERLIN_SCALE_FACTOR: f64 = 15.0;

#[repr(usize)]
#[derive(PartialEq, Eq)]
/// A Unit-only enum represinting the different indexes (usize) used in tilesheet.
/// An enum is used to more easily clearify and understand what indexes are represent what in the tilesheet so that we dont have to remember each index manually.
/// Consider changing this into e.g consts in lib.rs or something complety different.
///
/// example:
/// ```rust
/// match value {
///     v if v < 0.5 => TileSheetIndex::MiddleGrassTile as usize,
///     v if v > 0.5 => TileSheetIndex::MiddleSandTile as usize,
///     _ => unreachable!("Unreachable!")
/// }
/// ```
pub enum TileSheetIndex {
    MiddleGrassTile = 113,
    MiddleSandTile = 117,
    Bush = 1078,
    DownGrassTile = 1182,
    LeftGrassTile = 1183,
    RightGrassTile = 1238,
    TopGrassTile = 1239,
}

#[derive(Resource)]
/// Generation seed used strictly for the Perlin grid for map generation.
pub struct GenerationSeed(u32);

#[derive(Component)]
pub struct LayerOne;

#[derive(Component)]
pub struct LayerTwo;

#[derive(Component)]
pub struct LayerThree;

#[derive(Component)]
pub struct TileType(pub TileSheetIndex);
pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        let mut rng = rand::thread_rng();
        app.insert_resource(GenerationSeed(rng.gen()))
            .add_systems(Startup, setup_map);
    }
}
pub fn setup_map(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    seed: Res<GenerationSeed>,
) {
    let map = generate_noise_map(seed.0);
    spawn_ground_layer(
        &mut commands,
        &asset_server,
        &mut texture_atlas_layouts,
        &map,
    );
    spawn_decoration_layer(
        &mut commands,
        &asset_server,
        &mut texture_atlas_layouts,
        &map,
    );
    spawn_rounded_edges_layer(
        &mut commands,
        &asset_server,
        &mut texture_atlas_layouts,
        &map,
    );
}

fn spawn_decoration_layer(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    texture_atlas_layouts: &mut ResMut<Assets<TextureAtlasLayout>>,
    map: &Perlin,
) {
    let texture_handle: Handle<Image> = asset_server.load("environment/map_tilesheet.png");
    let layout = TextureAtlasLayout::from_grid(TILE_SIZE, GRID_COL, GRID_ROW, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);
    commands
        .spawn(())
        .insert(Name::new("Map Layer Three"))
        .insert(LayerThree)
        .insert(Transform::default())
        .insert(GlobalTransform::default())
        .insert(InheritedVisibility::default())
        .with_children(|child| {
            for x in 0..MAP_SIZE.0 {
                for y in 0..MAP_SIZE.1 {
                    if should_spawn_tree(map.get([
                        x as f64 / PERLIN_SCALE_FACTOR,
                        y as f64 / PERLIN_SCALE_FACTOR,
                    ])) {
                        child.spawn(SpriteSheetBundle {
                            texture: texture_handle.clone(),
                            atlas: TextureAtlas {
                                layout: texture_atlas_layout.clone(),
                                index: TileSheetIndex::Bush as usize,
                            },
                            sprite: Sprite {
                                custom_size: Some(TILE_SIZE),
                                ..Default::default()
                            },
                            transform: Transform::from_translation(Vec3::new(
                                START_X + (x as f32 * TILE_SIZE.x),
                                START_Y + (y as f32 * TILE_SIZE.y),
                                TILE_LAYER_3_Z,
                            )),
                            ..default()
                        });
                    }
                }
            }
        });
}

fn spawn_ground_layer(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    texture_atlas_layouts: &mut ResMut<Assets<TextureAtlasLayout>>,
    map: &Perlin,
) {
    let texture_handle: Handle<Image> = asset_server.load("environment/map_tilesheet.png");
    let layout = TextureAtlasLayout::from_grid(TILE_SIZE, GRID_COL, GRID_ROW, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);
    commands
        .spawn(())
        .insert(Name::new("Map Layer One"))
        .insert(LayerOne)
        .insert(Transform::default())
        .insert(GlobalTransform::default())
        .insert(InheritedVisibility::default())
        .with_children(|child| {
            for x in 0..MAP_SIZE.0 {
                for y in 0..MAP_SIZE.1 {
                    let tile_sheet_index = get_ground_texture_index(map.get([
                        x as f64 / PERLIN_SCALE_FACTOR,
                        y as f64 / PERLIN_SCALE_FACTOR,
                    ]));
                    child.spawn(SpriteSheetBundle {
                        texture: texture_handle.clone(),
                        atlas: TextureAtlas {
                            layout: texture_atlas_layout.clone(),
                            index: tile_sheet_index as usize,
                        },
                        sprite: Sprite {
                            custom_size: Some(TILE_SIZE),
                            ..Default::default()
                        },
                        transform: Transform::from_translation(Vec3::new(
                            START_X + (x as f32 * TILE_SIZE.x),
                            START_Y + (y as f32 * TILE_SIZE.y),
                            TILE_LAYER_1_Z,
                        )),
                        ..default()
                    });
                }
            }
        });
}

fn spawn_rounded_edges_layer(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    texture_atlas_layouts: &mut ResMut<Assets<TextureAtlasLayout>>,
    map: &Perlin,
) {
    let texture_handle: Handle<Image> = asset_server.load("environment/map_tilesheet.png");
    let layout = TextureAtlasLayout::from_grid(TILE_SIZE, GRID_COL, GRID_ROW, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);

    commands
        .spawn(())
        .insert(Name::new("Map Layer Two"))
        .insert(LayerTwo)
        .insert(Transform::default())
        .insert(GlobalTransform::default())
        .insert(InheritedVisibility::default())
        .with_children(|child| {
            for x in 0..MAP_SIZE.0 {
                for y in 0..MAP_SIZE.1 {
                    let tile_sheet_index = get_ground_texture_index(map.get([
                        x as f64 / PERLIN_SCALE_FACTOR,
                        y as f64 / PERLIN_SCALE_FACTOR,
                    ]));
                    match tile_sheet_index {
                        TileSheetIndex::MiddleSandTile => {
                            // Check for bottom
                            if y < MAP_SIZE.1
                                && get_ground_texture_index(map.get([
                                    x as f64 / PERLIN_SCALE_FACTOR,
                                    (y + 1) as f64 / PERLIN_SCALE_FACTOR,
                                ])) == TileSheetIndex::MiddleGrassTile
                            {
                                child.spawn(SpriteSheetBundle {
                                    texture: texture_handle.clone(),
                                    atlas: TextureAtlas {
                                        layout: texture_atlas_layout.clone(),
                                        index: TileSheetIndex::DownGrassTile as usize,
                                    },
                                    sprite: Sprite {
                                        custom_size: Some(TILE_SIZE),
                                        ..Default::default()
                                    },
                                    transform: Transform::from_translation(Vec3::new(
                                        START_X + (x as f32 * TILE_SIZE.x),
                                        START_Y + (y as f32 * TILE_SIZE.y),
                                        TILE_LAYER_2_Z,
                                    )),
                                    ..default()
                                });
                            }
                            // Check for Top
                            if y > 0
                                && get_ground_texture_index(map.get([
                                    x as f64 / PERLIN_SCALE_FACTOR,
                                    (y - 1) as f64 / PERLIN_SCALE_FACTOR,
                                ])) == TileSheetIndex::MiddleGrassTile
                            {
                                child.spawn(SpriteSheetBundle {
                                    texture: texture_handle.clone(),
                                    atlas: TextureAtlas {
                                        layout: texture_atlas_layout.clone(),
                                        index: TileSheetIndex::TopGrassTile as usize,
                                    },
                                    sprite: Sprite {
                                        custom_size: Some(TILE_SIZE),
                                        ..Default::default()
                                    },
                                    transform: Transform::from_translation(Vec3::new(
                                        START_X + (x as f32 * TILE_SIZE.x),
                                        START_Y + (y as f32 * TILE_SIZE.y),
                                        TILE_LAYER_2_Z,
                                    )),
                                    ..default()
                                });
                            }
                            // Check for Left
                            if x < MAP_SIZE.0
                                && get_ground_texture_index(map.get([
                                    (x + 1) as f64 / PERLIN_SCALE_FACTOR,
                                    y as f64 / PERLIN_SCALE_FACTOR,
                                ])) == TileSheetIndex::MiddleGrassTile
                            {
                                child.spawn(SpriteSheetBundle {
                                    texture: texture_handle.clone(),
                                    atlas: TextureAtlas {
                                        layout: texture_atlas_layout.clone(),
                                        index: TileSheetIndex::LeftGrassTile as usize,
                                    },
                                    sprite: Sprite {
                                        custom_size: Some(TILE_SIZE),
                                        ..Default::default()
                                    },
                                    transform: Transform::from_translation(Vec3::new(
                                        START_X + (x as f32 * TILE_SIZE.x),
                                        START_Y + (y as f32 * TILE_SIZE.y),
                                        TILE_LAYER_2_Z,
                                    )),
                                    ..default()
                                });
                            }
                            // Check for Right.
                            if x > 0
                                && get_ground_texture_index(map.get([
                                    (x - 1) as f64 / PERLIN_SCALE_FACTOR,
                                    y as f64 / PERLIN_SCALE_FACTOR,
                                ])) == TileSheetIndex::MiddleGrassTile
                            {
                                child.spawn(SpriteSheetBundle {
                                    texture: texture_handle.clone(),
                                    atlas: TextureAtlas {
                                        layout: texture_atlas_layout.clone(),
                                        index: TileSheetIndex::RightGrassTile as usize,
                                    },
                                    sprite: Sprite {
                                        custom_size: Some(TILE_SIZE),
                                        ..Default::default()
                                    },
                                    transform: Transform::from_translation(Vec3::new(
                                        START_X + (x as f32 * TILE_SIZE.x),
                                        START_Y + (y as f32 * TILE_SIZE.y),
                                        TILE_LAYER_2_Z,
                                    )),
                                    ..default()
                                });
                            }
                        }
                        _ => continue,
                    }
                }
            }
        });
}
fn generate_noise_map(seed: u32) -> Perlin {
    Perlin::new(seed)
}

fn should_spawn_tree(val: f64) -> bool {
    match val.abs() {
        v if v > 0.9 => true,
        _ => false,
    }
}

fn get_ground_texture_index(val: f64) -> TileSheetIndex {
    match val.abs() {
        v if v < 0.5 => TileSheetIndex::MiddleGrassTile,
        v if v > 0.5 => TileSheetIndex::MiddleSandTile,
        _ => unreachable!("no index found"),
    }
}
