use ggez::*;
use ggez::event::*;
use glam::*;

const PLAYER_WIDTH: f32 = 20.;
const PLAYER_HEIGHT: f32 = 100.;
// Padding to edge of screen
const BUFFER: f32 = 10.;
const BAT_SPEED: f32 = 10.;

#[derive(PartialEq)]
enum Direction {
    Up,
    Down,
}

struct Ball {
    color: graphics::Color,
    mode: graphics::DrawMode,
    point: Vec2,
    radius: f32,
    tolerance: f32,
    position: Vec2
}

struct MainState {
    dt: std::time::Duration, // track delta time for frame rate
    player1: Player,
    player2: Player,
    ball: Ball
}

struct Player {
    color: graphics::Color,
    mode: graphics::DrawMode,
    rect: graphics::Rect,
    position: Vec2,
}

impl Ball {
    // create the ball
    fn new(_ctx: &mut Context) -> Self {
        let (screen_width, screen_height) = graphics::drawable_size(_ctx);
        let c = Ball {
            color: graphics::Color::WHITE,
            mode: graphics::DrawMode::fill(),
            point: Vec2::new(0., 0.),
            radius: 15.,
            tolerance: 1.,
            position: Vec2::new(screen_width/2., screen_height/2.)
        };
        c
    }

}

impl Player {
    // Create new player object
    fn new(_ctx: &mut Context) -> Self {
        let (_screen_width, screen_height) = graphics::drawable_size(_ctx);
        let p = Player { color: graphics::Color::WHITE, 
            mode: graphics::DrawMode::fill(),
            rect: graphics::Rect::new(0., 0., PLAYER_WIDTH, PLAYER_HEIGHT),
            position: Vec2::new(0., screen_height/2.) };
        p
    }

    fn update_position(&mut self, ctx: &mut Context, direction: Direction) -> GameResult {
        
        let (_screen_width, screen_height) = graphics::drawable_size(ctx);
        if direction == Direction::Up {
            self.position.y -= BAT_SPEED;   
        } else {
            self.position.y += BAT_SPEED;
        }
        
        // If the movement would be out of bounds, stop the movement.
        if self.position.y < BUFFER {
            self.position.y = BUFFER;
        } else if self.position.y > (screen_height-(PLAYER_HEIGHT+BUFFER*2.)) {
            self.position.y = screen_height-(PLAYER_HEIGHT+BUFFER*2.);
            //println!("My position is {}", self.position.y);
        } 
        Ok(())
    }
}

impl MainState {
    // create new state.
    fn new(_ctx: &mut Context) -> GameResult<MainState> {
        let s = MainState { dt: std::time::Duration::new(0, 0),
                            player1: Player::new(_ctx),
                            player2: Player::new(_ctx),
                            ball: Ball::new(_ctx) };
        Ok(s)
    }
}

impl event::EventHandler<ggez::GameError> for MainState {

       
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        self.dt = timer::delta(ctx); // Get the delta time
        
        if input::keyboard::is_key_pressed(ctx, KeyCode::W) {
            self.player1.update_position(ctx, Direction::Up)?;
        }
        else if input::keyboard::is_key_pressed(ctx, KeyCode::S) {
            self.player1.update_position(ctx, Direction::Down)?;
        }
        if input::keyboard::is_key_pressed(ctx,KeyCode::Up) {
            self.player2.update_position(ctx, Direction::Up)?;
        }
        else if input::keyboard::is_key_pressed(ctx, KeyCode::Down) {
            self.player2.update_position(ctx, Direction::Down)?;
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {

        graphics::clear(ctx, [0., 0., 0., 1.0].into());
        let (screen_width, screen_height) = graphics::drawable_size(ctx);

        // Draw the center net
        let net_points = [(mint::Point2 {x:screen_width/2., y:0.}),(mint::Point2 {x: screen_width/2., y: screen_height})];
        let net = graphics::Mesh::new_line(ctx, &net_points, 5., graphics::Color::WHITE)?;
        graphics::draw(ctx, &net, (Vec2::new(0.,0.),))?;

        // Player 1
        let player1 = graphics::Mesh::new_rectangle(ctx, self.player1.mode, self.player1.rect, self.player1.color)?;
        graphics::draw(ctx, &player1, (Vec2::new(0.+BUFFER, self.player1.position.y + BUFFER),))?;

        //Player 2
        let player2 = graphics::Mesh::new_rectangle(ctx, self.player2.mode, self.player2.rect, self.player2.color)?;
        graphics::draw(ctx, &player2, (Vec2::new(screen_width-(PLAYER_WIDTH+BUFFER), self.player2.position.y +BUFFER),))?;

        //Ball
        let ball = graphics::Mesh::new_circle(ctx, self.ball.mode, self.ball.point, self.ball.radius, self.ball.tolerance, self.ball.color)?;
        graphics::draw(ctx, &ball, (Vec2::new(screen_width/2., screen_height/2.),))?;

        //println!("Hello there! dt = {}ms", self.dt.as_millis());
        graphics::present(ctx)?;
        Ok(())
    }
}

pub fn main() -> GameResult {

    //Create the context
    let (mut ctx, event_loop) = ContextBuilder::new("Rusty Pong", "BootStorm")
        .window_setup(ggez::conf::WindowSetup::default().title("Rusty Pong!"))
        .build()
        .unwrap();

    let state = MainState::new(&mut ctx)?;
    // kick off the main loop
    event::run(ctx, event_loop, state);
}
