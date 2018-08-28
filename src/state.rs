use std::time;

const GRAVITY: f64 = 150.0;
const MAX_VERTICAL_SPEED: f64 = 100.0;
const SIDEWAYS_MOVEMENT_PER_SECOND: f64 = 200.0;
const JUMP_INTERVAL_MILLIS: u64 = 250;

// Game state.
pub struct State {
    pub players: [Player; 2],
}

pub struct Player {
    // Coordinates on screen, in pixels.
    pub x: f64,
    // Smaller values towards bottom of the screen.
    pub y: f64,
    // Vertical speed.
    pub dy: f64,
    // Angle we are leaning (rad). Non-zero if moving left/right.
    pub lean_angle: f64,
    // Are we jumping?
    jumping: bool,
    // Time when the jump was started.
    started_jump: time::Instant,
}

impl State {
    pub fn new() -> Self {
        State {
            players: [Player::new(100.0), Player::new(200.0)],
        }
    }
}

impl Player {
    fn new(x: f64) -> Self {
        Player {
            x: x,
            y: 50.0,
            dy: 0.0,
            lean_angle: 0.0,
            jumping: false,
            started_jump: time::Instant::now(),
        }
    }

    pub fn update(&mut self, dt: f64, left: bool, right: bool, up: bool) {
        if up && !self.jumping {
            self.jumping = true;
            self.started_jump = time::Instant::now();
            self.dy = 1.3 * MAX_VERTICAL_SPEED;
        }
        if !up && (self.started_jump.elapsed() > time::Duration::from_millis(JUMP_INTERVAL_MILLIS))
        {
            self.jumping = false;
        }

        self.lean_angle = 0.0;
        if left {
            self.x -= SIDEWAYS_MOVEMENT_PER_SECOND * dt;
            self.lean_angle = -0.05;
        }
        if right {
            self.x += SIDEWAYS_MOVEMENT_PER_SECOND * dt;
            self.lean_angle = 0.05;
        }
        // Update vertical position, use Verlet integration
        // (https://gamedev.stackexchange.com/questions/15708/).
        self.y += dt * (self.dy - dt * GRAVITY);
        if self.y < 0.0 {
            self.y = 0.0;
        }
        // Update vertical speed.
        self.dy = (self.dy - dt * GRAVITY).max(-MAX_VERTICAL_SPEED);
    }
}
