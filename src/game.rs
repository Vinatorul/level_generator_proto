use tile_engine::{TileEngine, TileRect};

type Room = [i32; 4];

#[derive(Default)]
pub struct Game {
    pub tiles: TileEngine,
    rooms: Vec<Room>,
}

impl Game {
    pub fn update(&mut self) {

    }
}
