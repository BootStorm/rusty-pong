use ggez::*;
use ggez::event::*;
use glam::*;
use rand::Rng;

const PLAYER_WIDTH: f32 = 20.;
const PLAYER_HEIGHT: f32 = 100.;
// Padding to edge of screen
const BUFFER: f32 = 0.;
const BAT_SPEED: f32 = 10.;
const BALL_SPEED: f32 = 10.;
const BALL_DIA: f32 = 15.;
const BALL_RAD: f32 = BALL_DIA*0.5;

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
    position: Vec2,
    direction: Vec2
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
    score: i32
}

impl Ball {
    // create the ball
    fn new(_ctx: &mut Context) -> Self {
        let (screen_width, screen_height) = graphics::drawable_size(_ctx);
        // generate random vector information
        let mut rng = rand::thread_rng();
        let rand_x: f32 = rng.gen();
        let rand_y: f32 = rng.gen();

        let c = Ball {
            color: graphics::Color::WHITE,
            mode: graphics::DrawMode::fill(),
            point: Vec2::new(0., 0.),
            radius: BALL_DIA,
            tolerance: 0.5,
            position: Vec2::new(screen_width/2., screen_height/2.),
            direction: Vec2::new(rand_x, rand_y)
        };
        c
    }

    fn reset_ball( &mut self, _ctx: &mut Context) -> GameResult {
        let (screen_width, screen_height) = graphics::drawable_size(_ctx);
        let mut rng = rand::thread_rng();
        let rand_x: f32 = rng.gen();
        let rand_y: f32 = rng.gen();

        self.position = Vec2::new(screen_width/2., screen_height/2.);
        self.direction = Vec2::new(rand_x, rand_y);
        Ok(())
    }

    //move the ball in a direction
    fn move_ball(&mut self, _ctx: &mut Context, player1: &mut Player, player2: &mut Player) -> GameResult {

        let (screen_width, screen_height) = graphics::drawable_size(_ctx);

        // reverse the direction if it hits top or bottom boundary
        if self.position.y < BALL_RAD || self.position.y > (screen_height-BALL_RAD) {
            self.direction.y = -self.direction.y;
        }

        // If the ball bounces off player1
        if self.position.x < (PLAYER_WIDTH + BUFFER) {
            if self.position.y > player1.position.y && self.position.y < (player1.position.y+PLAYER_HEIGHT) {
                self.direction.x = -self.direction.x;
            } else {
                // Player2 scores
                player2.score += 1;
                self.reset_ball(_ctx)?;
            }
        }

                // If the ball bounces off player 2
        if self.position.x > screen_width-PLAYER_WIDTH-BUFFER {
            if self.position.y > player2.position.y && self.position.y < (player2.position.y+PLAYER_HEIGHT) {
                self.direction.x = -self.direction.x;
            } else {
                player1.score += 1;
                self.reset_ball(_ctx)?;
            }
        }

        self.position += self.direction * BALL_SPEED;

        Ok(())
    }

}

impl Player {
    // Create new player object
    fn new(_ctx: &mut Context) -> Self {
        let (_screen_width, screen_height) = graphics::drawable_size(_ctx);
        let p = Player { color: graphics::Color::WHITE, 
            mode: graphics::DrawMode::fill(),
            rect: graphics::Rect::new(0., 0., PLAYER_WIDTH, PLAYER_HEIGHT),
            position: Vec2::new(0., screen_height/2.),
            score: 0 };
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
        self.ball.move_ball(ctx, &mut self.player1, &mut self.player2)?;
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
        //graphics::draw(ctx, &ball, (Vec2::new(screen_width/2., screen_height/2.),))?;
        graphics::draw(ctx, &ball, (Vec2::new(self.ball.position.x, self.ball.position.y),))?;

        // Score
        let score = graphics::Text::new(format!("{}      {}", self.player1.score, self.player2.score));
        graphics::draw(ctx, &score, (Vec2::new((screen_width-score.width(ctx))/2., 20.),))?;

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
