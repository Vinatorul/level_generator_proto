use sdl2::render::Renderer;
use sdl2::rect::Rect;
use sdl2::pixels::Color;
use game::Game;

pub struct Visualizer<'a> {
    renderer: Renderer<'a>,
}

impl<'a> Visualizer<'a> {
    pub fn new(renderer: Renderer<'a>) -> Visualizer<'a> {
        Visualizer {
            renderer: renderer,
        }
    }

    pub fn draw(&mut self, game: &Game, lag: f64) {
        let _ = self.renderer.set_draw_color(Color::RGB(0, 0, 0));
        let _ = self.renderer.clear();

        //let _ = self.renderer.set_draw_color(Color::RGB(255, 0, 0));
        //for i in game.tiles.get_tiles(0., 0., 800, 600, 1).iter() {
            //let rect = Rect::new(i[0], i[1], i[2] as u32, i[3] as u32).unwrap().unwrap();
            //let _ = self.renderer.draw_rect(rect);
        //}

        let _ = self.renderer.set_draw_color(Color::RGB(0, 0, 255));
        for tile in game.tiles.get_tiles(0., 0., 800, 600, 2).iter() {
            let rect = Rect::new(tile.x, tile.y, tile.width, tile.height).unwrap().unwrap();
            let _ = self.renderer.draw_rect(rect);
        }
        let _ = self.renderer.set_draw_color(Color::RGB(0, 255, 0));
        for tile in game.tiles.get_tiles(0., 0., 800, 600, 3).iter() {
            let rect = Rect::new(tile.x, tile.y, tile.width, tile.height).unwrap().unwrap();
            let _ = self.renderer.draw_rect(rect);
        }

        let _ = self.renderer.present();
    }
}
