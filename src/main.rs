
use bracket_lib::prelude::*;

const SCREEN_WIDTH: i32 = 80;
const SCREEN_HEIGHT: i32 = 50;
const FRAME_DURATION: f32 = 75.0;

enum GameMode {
    Menu,
    Playing,
    End,
}
struct State {
    mode: GameMode,
    player: Player,
    frame_time: f32,
    obstacle: Obstacle,
    score: i32,
}
impl State {
    fn new() -> Self {
        State {
            mode: GameMode::Menu,
            player: Player::new(5, 25),
            frame_time: 0.0,
            obstacle: Obstacle::new(SCREEN_WIDTH, 0),
            score: 0,
        }
    }
    fn play(&mut self, ctx: &mut BTerm){
        // TDOD: Fill in this sub later
        ctx.cls_bg(NAVY);
        self.frame_time += ctx.frame_time_ms;
        if self.frame_time > FRAME_DURATION{
            self.frame_time = 0.0;
            self.player.gravity_and_move();
        }
        if let Some(VirtualKeyCode::Space) = ctx.key {
            self.player.flap();
        }
        self.player.render(ctx);
        ctx.print(0, 0, "Press Space to flap");
        ctx.print(0, 1, &format!("Score: {}", self.score));
        self.obstacle.render(ctx, self.player.x);
        if self.player.x > self.obstacle.x {
            self.score += 1;
            self.obstacle = Obstacle::new(self.player.x + SCREEN_WIDTH, self.score);
        }
        if self.player.y > SCREEN_HEIGHT || self.obstacle.hit_obstacle(&self.player){
            self.mode = GameMode::End;
        }
    }
    fn restart(&mut self){
        self.mode = GameMode::Playing;
        self.player = Player::new(5, 25);
        self.frame_time = 0.0;
        self.obstacle = Obstacle::new(SCREEN_WIDTH, 0);
        self.score = 0;
    }
    fn main_menu(&mut self, ctx: &mut BTerm){
        ctx.cls();
        ctx.print_centered(5, "Welcome to Simple Roguelike");
        ctx.print_centered(10, "(P) Play Game");
        ctx.print_centered(12, "(Q) Quit Game");
        match ctx.key {
            None => {}
            Some(key) => match key {
                VirtualKeyCode::P => self.restart(),
                VirtualKeyCode::Q => ctx.quitting = true,
                _ => {}
            },
        }
    }

    fn dead(&mut self, ctx: &mut BTerm){
        ctx.cls();
        ctx.print_centered(5, "You have died!");
        ctx.print_centered(8, &format!("You earned {} points", self.score));
        ctx.print_centered(10, "Press (P) to play again");
        ctx.print_centered(12, "Press (Q) to quit");

        if let Some(key) = ctx.key {
            match key {
                VirtualKeyCode::P => self.restart(),
                VirtualKeyCode::Q => ctx.quitting = true,
                _ => {}
            }
        }

    }
}


struct Player{
    x: i32,
    y: i32,
    volocity: f32,
}

impl Player {
    fn new(x: i32, y: i32) -> Self {
        Player {
            x,
            y,
            volocity: 0.0,
        }
    }

    fn render(&mut self, ctx: &mut BTerm) {
        ctx.set(
            0,
            self.y,
            RGB::named(YELLOW),
            RGB::named(BLACK),
            to_cp437('@'),
        );
    }

    fn gravity_and_move(&mut self){
        if self.volocity < 2.0 {
            self.volocity += 0.2;
        }
        self.y += self.volocity as i32;
        self.x += 1;
        if self.y < 0 {
            self.y = 0;
        }
    }
    fn flap(&mut self){
        self.volocity = -2.0;
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        match self.mode {
            GameMode::Menu => self.main_menu(ctx),
            GameMode::Playing => self.play(ctx),
            GameMode::End => self.dead(ctx),
        }
    }
}

struct Obstacle {
    x: i32,
    gap_y: i32,
    size: i32,
}
impl Obstacle {
    fn new(x: i32, score: i32) -> Self {
        let mut random = RandomNumberGenerator::new();
        Obstacle {
            x,
            gap_y: random.range(10, 40),
            size: i32::max(1, 20 - score),
        }
    }
    fn render(&mut self, ctx: &mut BTerm, player_x: i32){
        let screen_x = self.x - player_x;
        let half_size = self.size / 2;
        // Dtaw the top half of the obstacle
        for y in 0..self.gap_y - half_size {
            ctx.set(
                screen_x,
                y,
                RGB::named(RED),
                RGB::named(BLACK),
                to_cp437('|'),
            );
        }
        // Draw the bottom half of the obstacle
        for y in self.gap_y + half_size..SCREEN_HEIGHT {
            ctx.set(
                screen_x,
                y,
                RGB::named(RED),
                RGB::named(BLACK),
                to_cp437('|'),
            );
        }
    }
    fn hit_obstacle(&self, player: &Player) -> bool {
        let half_size = self.size / 2;
        let x_match = self.x == player.x;
        let player_above = player.y < self.gap_y - half_size;
        let player_below = player.y > self.gap_y + half_size;
        x_match && (player_above || player_below)
    }
}
fn main() -> BError {
    let context = BTermBuilder::simple80x50()
        .with_title("Bracket Terminal Example - Simple")
        .build()?;
    main_loop(context, State::new())
}
