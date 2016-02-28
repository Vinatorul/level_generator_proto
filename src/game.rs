use tile_engine::TileEngine;
use sdl2::event::Event;
use dungeon_generator::{BSPGenerator, Room, DungeonGenerator, RoomType};

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
        self.rooms = BSPGenerator::default().generate(seed, TEMP_WIDTH, TEMP_HEIGHT);
        for room in self.rooms.iter() {
            match room.room_type {
                RoomType::BasicRoom => self.tiles.add_tile(room.x as f64, room.y as f64, room.width as i32, room.height as i32, 2),
                RoomType::Coridor => self.tiles.add_tile(room.x as f64, room.y as f64, room.width as i32, room.height as i32, 3),
                _ => unreachable!(),
            }
        }
    }
}
