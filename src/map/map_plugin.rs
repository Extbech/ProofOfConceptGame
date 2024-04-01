use bevy::prelude::*;
use noise::{NoiseFn, Perlin};
use rand::Rng;
use test_game::{TILE_LAYER_1_Z, TILE_LAYER_2_Z};
const MAP_SIZE: (usize, usize) = (128, 128);
const TILE_SIZE: Vec2 = Vec2::new(32.0, 32.0);
const GRID_COL: usize = 56;
const GRID_ROW: usize = 22;
const START_X: f32 = -(MAP_SIZE.0 as f32 * TILE_SIZE.x) / 2.0;
const START_Y: f32 = -(MAP_SIZE.1 as f32 * TILE_SIZE.y) / 2.0;
const PERLIN_SCALE_FACTOR: f64 = 15.0;

#[derive(Resource)]
pub struct GenerationSeed(u32);
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
    spawn_ground_layer(
        &mut commands,
        &asset_server,
        &mut texture_atlas_layouts,
        &seed,
    );
    spawn_decoration_layer(
        &mut commands,
        &asset_server,
        &mut texture_atlas_layouts,
        seed,
    );
}

fn spawn_decoration_layer(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    texture_atlas_layouts: &mut ResMut<Assets<TextureAtlasLayout>>,
    seed: Res<GenerationSeed>,
) {
    let map = generate_noise_map(seed.0);

    let texture_handle: Handle<Image> = asset_server.load("environment/map_tilesheet.png");
    let layout = TextureAtlasLayout::from_grid(TILE_SIZE, GRID_COL, GRID_ROW, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);

    for x in 0..MAP_SIZE.0 {
        for y in 0..MAP_SIZE.1 {
            if should_spawn_tree(map.get([
                x as f64 / PERLIN_SCALE_FACTOR,
                y as f64 / PERLIN_SCALE_FACTOR,
            ])) {
                commands.spawn(SpriteSheetBundle {
                    texture: texture_handle.clone(),
                    atlas: TextureAtlas {
                        layout: texture_atlas_layout.clone(),
                        index: 1078,
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
    }
}

fn spawn_ground_layer(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    texture_atlas_layouts: &mut ResMut<Assets<TextureAtlasLayout>>,
    seed: &Res<GenerationSeed>,
) {
    let map = generate_noise_map(seed.0);

    let texture_handle: Handle<Image> = asset_server.load("environment/map_tilesheet.png");
    let layout = TextureAtlasLayout::from_grid(TILE_SIZE, GRID_COL, GRID_ROW, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);

    for x in 0..MAP_SIZE.0 {
        for y in 0..MAP_SIZE.1 {
            commands.spawn(SpriteSheetBundle {
                texture: texture_handle.clone(),
                atlas: TextureAtlas {
                    layout: texture_atlas_layout.clone(),
                    index: get_ground_texture_index(map.get([
                        x as f64 / PERLIN_SCALE_FACTOR,
                        y as f64 / PERLIN_SCALE_FACTOR,
                    ])),
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
}
fn generate_noise_map(seed: u32) -> Perlin {
    //let mut rng = thread_rng();
    //let seed: u32 = rng.gen();

    Perlin::new(seed)
}

fn should_spawn_tree(val: f64) -> bool {
    match val.abs() {
        v if v > 0.9 => true,
        _ => false,
    }
}

fn get_ground_texture_index(val: f64) -> usize {
    match val.abs() {
        v if v < 0.5 => 113,
        v if v > 0.5 => 117,
        _ => unreachable!("no index found"),
    }
}
