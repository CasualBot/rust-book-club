#![warn(clippy::all, clippy::pedantic)]
use bracket_lib::prelude::*;

const SCREEN_WIDTH : f32 = 80.0;
const SCREEN_HEIGHT : f32 =  50.0;
const FRAME_DURATION : f32 = 75.0;
const DRAGON_FRAMES : [u16; 6] = [ 64, 1, 2, 3, 2, 1 ];

struct Obstacle {
    x: f32,
    gap_y: f32,
    size: f32,
}

impl Obstacle {
    fn new(x: f32, score: f32) -> Self {
        let mut random = RandomNumberGenerator::new();
        Obstacle {
            x,
            gap_y: random.range(10.0,50.0),
            size: f32::max(2.0,20.0 - score),
        }
    }

    fn render(&mut self, ctx: &mut BTerm, player_x: f32) {
        let screen_x = self.x - player_x;
        let half_size = self.size / 2.0;

        // Draw the top half of the obstacle
        for y in 0..(self.gap_y - half_size) as i32{
            ctx.set(
                screen_x as i32,
                y,
                RED,
                BLACK,
                to_cp437('|'),
            );
        }
        

        // Draw the bottom half of the obstacle
        for y in (self.gap_y + half_size) as i32..SCREEN_HEIGHT as i32 {
            ctx.set(
                screen_x as i32,
                y,
                RED,
                BLACK,
                to_cp437('|')
            );
        } 
    }

    fn hit_obstacle(&self, player: &Player) -> bool {
        let half_size = self.size / 2.0;
        let does_x_match = player.x == self.x; 
        let player_above_gap = player.y < self.gap_y - half_size; // is the player above the gap
        let player_below_gap = player.y > self.gap_y + half_size; // is the player below the gap

        does_x_match && (player_above_gap || player_below_gap) // this is a return statement
    }
}

struct Player {
    x: f32,
    y: f32,
    velocity: f32,
    frame: usize
}

impl Player {
    fn new(x: f32, y: f32) -> Self {
        Player {
            x,
            y,
            velocity:0.0,
            frame:0
        }
    }

    fn render(&mut self, ctx: &mut BTerm) {
        ctx.set_active_console(1);
        ctx.cls();
        ctx.set_fancy(
            PointF::new(0.0, self.y as f32),
            1,
            Degrees::new(0.0),
            PointF::new(2.0, 2.0),
            WHITE,
            NAVY,
            DRAGON_FRAMES[self.frame]
        );
        ctx.set_active_console(0);
    }
    
    fn gravity_and_move(&mut self) {
        if self.velocity < 2.0 {
            self.velocity += 0.1;
        }

        self.y += self.velocity;
        if self.y < 0.0 {
            self.y = 0.0;
        }

        self.x += 1.0;
        self.frame += 1;
        self.frame = self.frame % 6;
    }

    fn flap(&mut self) {
        // Remember 0 is the top of the screen, using -2.0 goes up
        self.velocity = -2.0;
    }
    
}
struct State {
    player: Player,
    frame_time: f32,
    mode: GameMode,
    obstacle: Obstacle,
    score: f32
}

impl State {
    fn new() -> Self {
        State {
            player: Player::new(5.0, 25.0),
            frame_time: 0.0,
            mode: GameMode::Menu,
            obstacle: Obstacle::new(SCREEN_WIDTH, 0.0),
            score: 0.0,
        }
    }

    fn play(&mut self, ctx: &mut BTerm) {
        ctx.cls_bg(NAVY); // similar to cls() but lets you choose background color

        self.frame_time += ctx.frame_time_ms; // get tick() in milliseconds 
        if self.frame_time > FRAME_DURATION { // check that tick isn't running faster than const duration
            self.frame_time = 0.0; // Reset it if it is
            self.player.gravity_and_move();
        }

        // Check to see if the SPACE key is pressed and call flap 
        if let Some(VirtualKeyCode::Space) = ctx.key {
            self.player.flap();
        }

        self.player.render(ctx);  // render the player

        ctx.print(0,0, "Press SPACE to Flap"); // output text to screen
        ctx.print(0,1, &format!("Score: {}", self.score as i32)); // Time to show some score

        self.obstacle.render(ctx, self.player.x); // Render the obstacle

        if self.player.x > self.obstacle.x {
            self.score += 1.0;
            self.obstacle = Obstacle::new(
                self.player.x + SCREEN_WIDTH, self.score 
            );
        }

        if self.player.y > SCREEN_HEIGHT || self.obstacle.hit_obstacle(&self.player) { // is player position greater than max screen height
            self.mode = GameMode::End; // end game if so
        }
    }

    fn restart(&mut self) {
        self.player = Player::new(5.0, 25.0);
        self.frame_time = 0.0;
        self.obstacle = Obstacle::new(SCREEN_WIDTH, 0.0);
        self.mode = GameMode::Playing;
        self.score = 0.0;
    } 

    fn main_menu(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        ctx.print_centered(5, "Welcome to Flappy Dragon");
        ctx.print_centered(8, "(P) Play Game");
        ctx.print_centered(9, "(Q) Quit Game");

        if let Some(key) = ctx.key {
            match key {
                VirtualKeyCode::P => self.restart(),
                VirtualKeyCode::Q => ctx.quitting = true,
                _ => {}
            }
        }
    }

    fn dead(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        ctx.print_centered(5, "You are dead");
        ctx.print_centered(6, &format!("You eared {} points", self.score));
        ctx.print_centered(8, "(P) Play Again");
        ctx.print_centered(9 , "(Q) Quit Game");

        if let Some(key) = ctx.key {
            match key {
                VirtualKeyCode::P => self.restart(),
                VirtualKeyCode::Q => ctx.quitting = true,
                _ => {}
            }
        }
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        ctx.print(1,1, "Hello Bracket Terminal");
        match self.mode {
            GameMode::Menu => self.main_menu(ctx),
            GameMode::End => self.dead(ctx),
            GameMode::Playing => self.play(ctx),
        }
    } 
}

enum GameMode { 
    Menu,
    Playing,
    End
}

fn main() -> BError {

    let context = BTermBuilder::new()
        .with_title("Flappy Dragon")
        .with_font("../resources/flappy32.png", 32, 32)
        .with_simple_console(SCREEN_WIDTH as i32, SCREEN_HEIGHT as i32, "../resources/flappy32.png")
        .with_fancy_console(SCREEN_WIDTH as i32, SCREEN_HEIGHT as i32, "../resources/flappy32.png")
        .with_tile_dimensions(16, 16)
        .build()?;

    main_loop(context, State::new())
}