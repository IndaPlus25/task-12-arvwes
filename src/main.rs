use ggez::event::{self, EventHandler};
use ggez::graphics;
use ggez::{ContextBuilder, GameResult, conf};

struct GameState {
    x: f32,
}

impl EventHandler for GameState {
    fn update(&mut self, _ctx: &mut ggez::Context) -> GameResult {
        self.x += 1.0;
        Ok(())
    }

    fn draw(&mut self, ctx: &mut ggez::Context) -> Result<(), ggez::GameError> {
        let mut canvas = graphics::Canvas::from_frame(ctx, graphics::Color::WHITE);
  
        canvas.finish(ctx)
    }
}

fn main() -> GameResult {
    let window_mode = conf::WindowMode::default().dimensions(1280.0, 720.0);
    let cb = ContextBuilder::new("ggez_test", "arvwes").window_mode(window_mode);
    let (ctx, event_loop) = cb.build()?;
    let state = GameState { x: 100.0};
    event::run(ctx, event_loop, state)
}
