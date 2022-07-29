use macroquad::prelude::*;
use uuid::Uuid;

use crate::{GameContext, TextureLibrary, Tile};
use crate::engine::constants::WORLD_UNIT;
use crate::engine::entities::game_object::GameObject;

pub struct TileRow {
    tiles: Vec<Option<Tile>>
}

impl TileRow {
    pub fn new() -> TileRow{
        let tiles = Vec::new();
        return TileRow{tiles}
    }

    pub fn add_tile(&mut self, tile: &Tile) {
        self.tiles.push(Some(tile.clone()));
    }

    pub fn add_empty_tile(&mut self) {
        self.tiles.push(None);
    }
}

pub struct Tileset {
    rows: Vec<TileRow>,
    pos: Vec2,
    uuid: Uuid
}

impl Tileset {

    /// Create a new default tileset with the given position
    pub fn new(pos: Vec2) -> Tileset {
        let rows: Vec<TileRow> = Vec::new();
        return Tileset{rows, pos, uuid: Uuid::new_v4()}
    }

    /// Load the tilest from the given file
    pub fn load(texture_lib: &TextureLibrary, path: &str, pos: Vec2) -> Tileset {
        let mut set = Tileset::new(pos);
        for (y, line) in std::fs::read_to_string(path).unwrap().split("\n").into_iter().enumerate() {
            let mut  row = TileRow::new();

            for (x, texture) in line.split(",").into_iter().enumerate() {
                if texture.is_empty() {
                    row.add_empty_tile();
                    continue;
                }
                let mut tile = Tile::new_tex(texture_lib, texture.trim());
                tile.pos.x = pos.x + x as f32 * WORLD_UNIT;
                tile.pos.y = pos.y + y as f32 * WORLD_UNIT;
                row.add_tile(&tile);
            }
            set.add_row(row);
        }

        return set;
    }

    /// Add a new row into this tile set
    pub fn add_row(&mut self, row: TileRow) {
        self.rows.push(row);
    }
}

impl GameObject for Tileset {

    fn update(&mut self) {}

    fn render(&mut self) {
        for (row) in self.rows.iter_mut() {
            for (tile) in row.tiles.iter_mut() {
                // if tile is some or none
                match tile {
                    None => {}
                    Some(tile) => {
                        tile.render();
                    }
                }
            }
        }
    }

    fn uuid(&self) -> Uuid {
        self.uuid
    }
}