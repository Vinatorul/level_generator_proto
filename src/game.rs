use tile_engine::{TileEngine, TileType};
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
                RoomType::BasicRoom => self.tiles.add_tile(room.x as i32, room.y as i32, room.width, room.height, 2, TileType::Floor, 1),
                RoomType::Coridor => self.tiles.add_tile(room.x as i32, room.y as i32, room.width, room.height, 3, TileType::Floor, 1),
                _ => unreachable!(),
            }
        }
    }
}
