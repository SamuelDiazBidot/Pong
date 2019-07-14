use ggez::*;
use rand::Rng;

//Paddle////////////////////
struct Paddle {
    x: f32,
    y: f32,
}

impl Paddle {
    fn new(x: f32, y: f32) -> Self {
        Paddle {
            x,
            y,
        }
    }
    fn update(&mut self, ctx: &mut Context, up: input::keyboard::KeyCode, down: input::keyboard::KeyCode) -> GameResult {
        if input::keyboard::is_key_pressed(ctx, up) {
            if self.y > 0.0 {
                self.y -= 5.0;
            }
        }
        else if input::keyboard::is_key_pressed(ctx, down){
            if self.y < 450.0 {
                self.y += 5.0;
            }
        }
        Ok(())
    }
    fn draw(&self, ctx: &mut Context) -> GameResult {
        let rect = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            ggez::graphics::Rect::new(self.x, self.y, 25.0, 150.0),
            graphics::WHITE).unwrap();
        graphics::draw(ctx, &rect, graphics::DrawParam::default()).unwrap();
        Ok(())
    }
}
///////////////////////////////
//Ball/////////////////////////
struct Ball {
    x: f32,
    y: f32,
    dx: f32,
    dy: f32,
}

impl Ball {
    fn new(x: f32, y: f32, dx: f32, dy: f32) -> Self {
        Ball {
            x,
            y,
            dx,
            dy,
        }
    }
    fn serve(&mut self) {
        self.dx = 5.0;
        self.dy = 3.0;
    }
    fn reset(&mut self) {
        self.x = 390.0;
        self.y = 300.0;
        self.dx = 0.0;
        self.dy = 0.0;
    }
    fn play_sound(&mut self, ctx: &mut Context) -> GameResult {
        use ggez::audio::SoundSource;
        let mut plop = audio::Source::new(ctx, "/plop.ogg")?;
        let _ = plop.play_detached();
        Ok(())        
    }
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        self.x += self.dx;
        self.y += self.dy;
        if self.y <= 0.0 {
            self.dy = -self.dy;
            self.play_sound(ctx);
        }
        if self.y >= 575.0 {
            self.dy = -self.dy;
            self.play_sound(ctx);
        }
        if (input::keyboard::is_key_pressed(ctx, input::keyboard::KeyCode::Space)) && self.dx == 0.0 {
            self.serve();
        }
        Ok(())
    }
    fn draw(&self, ctx: &mut Context) -> GameResult {
        let rect = graphics::Mesh::new_rectangle(
            ctx, 
            graphics::DrawMode::fill(), 
            graphics::Rect::new(self.x, self.y, 25.0, 25.0), 
            graphics::WHITE).unwrap();
        graphics::draw(ctx, &rect, graphics::DrawParam::default()).unwrap();
        Ok(())
    }
}
////////////////////////////////
//Score Board///////////////////
struct ScoreBoard {
    player1: i8,
    player2: i8,
}

impl ScoreBoard {
    fn new() -> Self {
        ScoreBoard {
            player1: 0,
            player2: 0,
        }
    }
    fn player1_scored(&mut self) {
        self.player1 += 1;
    }
    fn player2_scored(&mut self) {
        self.player2 += 1;
    }
    fn draw(&self, ctx: &mut Context) -> GameResult {
        let font = graphics::Font::new(ctx, "/font.ttf")?;
        let player1_score = graphics::Text::new((self.player1.to_string(), font, 120.0));
        let player2_score = graphics::Text::new((self.player2.to_string(), font, 120.0));
        graphics::draw(ctx, &player1_score, (mint::Point2{ x: 200.0, y: 220.0 }, graphics::WHITE),)?;
        graphics::draw(ctx, &player2_score, (mint::Point2{ x: 530.0, y: 220.0 }, graphics::WHITE),)?;
        Ok(())
    }
}
////////////////////////////////

struct GameState {
    player1: Paddle,
    player2: Paddle,
    ball: Ball,
    score_board: ScoreBoard,
}

impl GameState {
    fn new() -> Self {
        GameState {
            player1: Paddle::new(25.0, 150.0),
            player2: Paddle::new(750.0, 150.0),
            ball: Ball::new(390.0, 300.0, 0.0, 0.0),
            score_board: ScoreBoard::new(),
        }
    }
}

impl ggez::event::EventHandler for GameState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        self.player1.update(ctx, input::keyboard::KeyCode::W, input::keyboard::KeyCode::S)?;
        self.player2.update(ctx, input::keyboard::KeyCode::Up, input::keyboard::KeyCode::Down)?;
        self.ball.update(ctx)?;
        collition(ctx, &mut self.ball, &mut self.player1, &mut self.player2);
        scored(&mut self.ball, &mut self.score_board);
        Ok(())
    }
    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, graphics::Color::new(0.42, 0.42, 0.42, 1.0));
        self.player1.draw(ctx)?;
        self.player2.draw(ctx)?;
        self.ball.draw(ctx)?;
        self.score_board.draw(ctx)?;
        graphics::present(ctx)?;
        Ok(())
    }
}

fn main() {
    let state = &mut GameState::new();
    let cb = ggez::ContextBuilder::new("Pong", "Sammy");
    let (ref mut ctx, ref mut event_loop) = &mut cb.build().unwrap();
    event::run(ctx, event_loop, state).unwrap();
}

fn collition(ctx: &mut Context, ball: &mut Ball, paddle1: &mut Paddle, paddle2: &mut Paddle) {
    if (ball.x == paddle1.x + 25.0) && (ball.y >= paddle1.y) && (ball.y <= paddle1.y + 150.0) {
        ball.dx = -ball.dx;
        ball.play_sound(ctx);
    }
    if (ball.x + 25.0 == paddle2.x) && (ball.y >= paddle2.y) && (ball.y <= paddle2.y + 150.0) {
        ball.dx = -ball.dx;
        ball.play_sound(ctx);
    }
}

fn scored(ball: &mut Ball, score_board: &mut ScoreBoard) {
    //Player 2 scores
    if ball.x < -25.0 {
        score_board.player2_scored();
        ball.reset();
    }
    if ball.x > 800.0 {
        score_board.player1_scored();
        ball.reset();
    }
}