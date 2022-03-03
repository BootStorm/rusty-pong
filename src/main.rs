use ggez::*;

struct State {
    dt: std::time::Duration, // track delta time for frame rate
}

impl ggez::event::EventHandler<GameError> for State {

    fn update(&mut self, ctx: &mut Context) -> GameResult {
        self.dt = timer::delta(ctx); // Get the delta time
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        println!("Hello there! dt = {}ms", self.dt.as_millis());
        Ok(())
    }
}

pub fn main() {
    // Create instance of state
    let state = State {
        dt: std::time::Duration::new(0, 0),
    };

    //Create the context
    let c = conf::Conf::new();
    let (ctx, event_loop) = ContextBuilder::new("hello ggez", "awesome_person")
        .default_conf(c)
        .build()
        .unwrap();

    // kick off the main loop
    event::run(ctx, event_loop, state);
}
