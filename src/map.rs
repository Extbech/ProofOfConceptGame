use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;
use bevy_ecs_tilemap::tiles::{TileStorage, TileTextureIndex};

const BACKGROUND: &str = "environment/map.png";
const BACKGROUND_METADATA: TilemapMetadata = TilemapMetadata {
    size: TilemapSize { x: 128, y: 64 },
    tile_size: TilemapTileSize { x: 32.0, y: 32.0 },
    grid_size: TilemapGridSize { x: 32.0, y: 32.0 },
};

struct TilemapMetadata {
    size: TilemapSize,
    tile_size: TilemapTileSize,
    grid_size: TilemapGridSize,
}

pub fn setup_map(commands: Commands, asset_server: Res<AssetServer>) {
    setup_grass(commands, asset_server);
}

fn setup_grass(mut commands: Commands, asset_server: Res<AssetServer>) {
    let texture_handle_grass: Handle<Image> = asset_server.load(BACKGROUND);
    let tilemap_entity = commands.spawn_empty().id();

    let TilemapMetadata {
        size,
        grid_size,
        tile_size,
    } = BACKGROUND_METADATA;

    let mut tile_storage = TileStorage::empty(size);

    for x in 0..size.x {
        for y in 0..size.y {
            let tile_pos = TilePos { x, y };
            let texture_index = if y > 30 && y < 34 {
                6
            } else if y > 25 && y < 29 {
                13
            } else {
                3
            };
            let tile_entity = commands
                .spawn((TileBundle {
                    position: tile_pos,
                    tilemap_id: TilemapId(tilemap_entity),
                    texture_index: TileTextureIndex(texture_index),
                    ..Default::default()
                },))
                .id();

            tile_storage.set(&tile_pos, tile_entity);
        }
    }

    let map_type = TilemapType::default();

    commands.entity(tilemap_entity).insert(TilemapBundle {
        size,
        grid_size,
        map_type,
        tile_size,
        storage: tile_storage,
        texture: TilemapTexture::Single(texture_handle_grass),
        transform: get_tilemap_center_transform(&size, &grid_size, &map_type, 0.0),
        ..Default::default()
    });
}
