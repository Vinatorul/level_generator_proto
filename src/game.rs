use tile_engine::TileEngine;
use sdl2::event::Event;
use level_generator;
use level_generator::{Room};

const TEMP_WIDTH: u32 = 800;
const TEMP_HEIGHT: u32 = 600;

#[derive(Default)]
pub struct Game {
    pub tiles: TileEngine,
    rooms: Vec<Room>,
}

impl Game {
    pub fn update(&mut self) {

    }

    pub fn proc_event(&mut self, event: Event) {

    }

    pub fn generate_level(&mut self, seed: &[usize]) {
        self.rooms = level_generator::generate_level(&mut self.tiles, seed, TEMP_WIDTH, TEMP_HEIGHT);
    }
}
