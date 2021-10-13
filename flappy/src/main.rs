use bracket_lib::prelude::*;

const SCREEN_WIDTH : i32 = 80;
const SCREEN_HEIGHT : i32 =  50;
const FRAME_DURATION : f32 = 75.0;

struct Player {
    x: i32,
    y: i32,
    velocity: f32,
}

impl Player {
    fn new(x: i32, y: i32) -> Self {
        Player {
            x,
            y,
            velocity:0.0,
        }
    }

    fn render(&mut self, ctx: &mut BTerm) {
        ctx.set(
            0,
            self.y,
            YELLOW,
            BLACK,
            to_cp437('@')
        )
    }
    
    fn gravity_and_move(&mut self) {
        // Check for terminal velocity
        if self.velocity < 2.0 {
            self.velocity += 0.2;
        }

        // Move the player up or down
        self.y += self.velocity as i32;
        // Move the player right
        self.x += 1;

        if self.y < 0 {
            self.y = 0;
        }
    }

    fn flap(&mut self) {
        // Remember 0 is the top of the screen, using -2.0 goes up
        self.velocity = -2.0
    }
    
}
struct State {
    player: Player,
    frame_time: f32,
    mode: GameMode
}

impl State {
    fn new() -> Self {
        State {
            player: Player::new(5,25),
            frame_time: 0.0,
            mode: GameMode::Menu,
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

        if self.player.y > SCREEN_HEIGHT { // is player position greater than max screen heigh
            self.mode = GameMode::End; // end game if so
        }
    }

    fn restart(&mut self) {
        self.player = Player::new(5,25);
        self.frame_time = 0.0;
        self.mode = GameMode::Playing;
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

    let context = BTermBuilder::simple80x50()
        .with_title("Flappy Dragon")
        .build()?;

    main_loop(context, State::new())
}