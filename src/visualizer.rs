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

        //let _ = self.renderer.set_draw_color(Color::RGB(0, 255, 0));

        //let player_rect = Rect::new(game.player.x(lag), game.player.y(lag), game.player.width, game.player.height).unwrap().unwrap();
        //let _ = self.renderer.draw_rect(player_rect);

        //let _ = self.renderer.set_draw_color(Color::RGB(255, 0, 0));
        //for i in &game.obstacles {
            //let obst_rect = Rect::new(i.x(), i.y(), i.width, i.height).unwrap().unwrap();
            //let _ = self.renderer.draw_rect(obst_rect);
        //}

        let _ = self.renderer.present();
    }
}
