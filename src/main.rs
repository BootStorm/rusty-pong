use ggez::*;
use glam::*;

struct MainState {
    dt: std::time::Duration, // track delta time for frame rate
}

impl MainState {
    // create new state.
    fn new(_ctx: &mut Context) -> GameResult<MainState> {
        let s = MainState { dt: std::time::Duration::new(0, 0) };
        Ok(s)
    }
}

impl event::EventHandler<ggez::GameError> for MainState {

    fn update(&mut self, ctx: &mut Context) -> GameResult {
        self.dt = timer::delta(ctx); // Get the delta time
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {

        graphics::clear(ctx, [0.1, 0.2, 0.3, 1.0].into());

        let rect = graphics::Mesh::new_rectangle(
            ctx, 
            graphics::DrawMode::fill(), 
            graphics::Rect::new(100., 100., 50., 30.), 
            graphics::Color::WHITE )?;

        graphics::draw(ctx, &rect, (Vec2::new(100., 100.),))?;
        //println!("Hello there! dt = {}ms", self.dt.as_millis());
        graphics::present(ctx)?;
        Ok(())
    }
}

pub fn main() -> GameResult {

    //Create the context
    let c = conf::Conf::new();
    let (mut ctx, event_loop) = ContextBuilder::new("hello ggez", "awesome_person")
        .default_conf(c)
        .build()
        .unwrap();

    let state = MainState::new(&mut ctx)?;
    // kick off the main loop
    event::run(ctx, event_loop, state);
}
