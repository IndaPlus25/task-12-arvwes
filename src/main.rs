use rand::Rng;
use std::ops::{Add, AddAssign, Sub, SubAssign};

use ggez::event::{self, EventHandler};
use ggez::graphics;
use ggez::{ContextBuilder, GameResult, conf};

struct GameState {
    veichle: Vehicle,
    target: Target,
}

#[derive(Copy, Clone, Debug)]
struct Vehicle {
    max_speed: f32,
    max_force: f32,
    position: Vector,
    velosity: Vector,
    acceleration: Vector,
    prediction: Vector,
}

impl Vehicle {
    fn seek(&mut self, target: Vector) {
        let mut desired = target - self.position;
        desired.set_max_len(self.max_speed);
        let mut steer = desired - self.velosity;
        steer.set_max_len(0.2);
        self.apply_fource(steer);
    }

    fn apply_fource(&mut self, force: Vector) {
        self.acceleration += force;
    }
    fn persuit(&mut self, target: &Target) {
        let distance = (target.position - self.position).get_len();
             let mut pred_fact = 0.1;
        if  self.velosity.get_len() != 0.0{
             pred_fact =target.velosity.get_len() / (self.velosity.get_len()* self.velosity.get_len());
        }
        
        println!("{}", pred_fact);

        let prediction = Vector {
            x: target.position.x + target.velosity.x * distance * 0.1,
            y: target.position.y + target.velosity.y * distance * 0.1,
        };
        self.prediction = prediction;
        let mut desired = prediction - self.position;
        desired.set_max_len(self.max_speed);
        let mut steer = desired - self.velosity;
        steer.set_max_len(self.max_force);
        self.apply_fource(steer);
    }
}

struct Target {
    position: Vector,
    velosity: Vector,
}

impl Target {
    fn new() -> Self {
        let mut rng = rand::thread_rng();
        let x = rng.gen_range(0.0..1280.0);
        let y = rng.gen_range(0.0..720.0);
        let mut v = Vector { x: x, y: y };
        if x > 1280.0 / 2.0 {
            v = Vector { x: -x, y: y };
        }
        v.set_max_len(6.0);

        Target {
            position: Vector { x: x, y: y },
            velosity: v,
        }
    }
}

#[derive(Copy, Clone, Debug)]
struct Vector {
    x: f32,
    y: f32,
}
impl AddAssign for Vector {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl Sub for Vector {
    type Output = Vector;
    fn sub(self, rhs: Self) -> Vector {
        Vector {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}
impl Vector {
    fn set_max_len(&mut self, max: f32) {
        let current = self.get_len();
        if current > 0.0 && current > max {
            let k = max / current;
            self.x *= k;
            self.y *= k;
        }
    }
    fn get_len(&self) -> f32 {
        (self.x * self.x + self.y * self.y).sqrt()
    }
}

impl EventHandler for GameState {
    fn update(&mut self, _ctx: &mut ggez::Context) -> GameResult {
        //self.veichle.seek(self.veichle.target);
        self.veichle.persuit(&self.target);
        self.veichle.velosity += self.veichle.acceleration;
        self.veichle.position += self.veichle.velosity;

        self.target.position += self.target.velosity;

        let dx = self.veichle.position.x - self.target.position.x;
        let dy = self.veichle.position.y - self.target.position.y;
        if dx * dx + dy * dy < 40.0 {
  
            self.target = Target::new();
        }

        if self.target.position.x > 1280.0 {
            self.target.position.x = 0.0;
        } else if self.target.position.x < 0.0 {
            self.target.position.x = 1280.0;
        } else if self.target.position.y < 0.0 {
            self.target.position.y = 720.0;
        } else if self.target.position.y > 720.0 {
            self.target.position.y = 0.0;
        }
        self.veichle.acceleration = Vector { x: 0.0, y: 0.0 };

        Ok(())
    }

    fn draw(&mut self, ctx: &mut ggez::Context) -> Result<(), ggez::GameError> {
        let mut canvas = graphics::Canvas::from_frame(ctx, graphics::Color::WHITE);
        let angle = self.veichle.velosity.y.atan2(self.veichle.velosity.x);
        let veichle = graphics::Mesh::new_circle(
            ctx,
            graphics::DrawMode::fill(),
            [self.veichle.position.x, self.veichle.position.y],
            50.0,
            2.0,
            graphics::Color::from_rgb(0, 128, 255),
        )?;
        let triangle = graphics::Mesh::new_polygon(
            ctx,
            graphics::DrawMode::fill(),
            &[[20.0, 0.0], [-10.0, 10.0], [-10.0, -10.0]],
            graphics::Color::from_rgb(0, 128, 255),
        )?;

        let target = graphics::Mesh::new_circle(
            ctx,
            graphics::DrawMode::fill(),
            [self.target.position.x, self.target.position.y],
            20.0,
            0.1,
            graphics::Color::from_rgb(255, 0, 100),
        )?;
        let prediction = graphics::Mesh::new_circle(
            ctx,
            graphics::DrawMode::fill(),
            [self.veichle.prediction.x, self.veichle.prediction.y],
            10.0,
            0.1,
            graphics::Color::from_rgb(0, 255, 50),
        )?;
        canvas.draw(
            &triangle,
            graphics::DrawParam::default()
                .dest([self.veichle.position.x, self.veichle.position.y])
                .rotation(angle),
        );
        canvas.draw(&target, graphics::DrawParam::default());
        canvas.draw(&prediction, graphics::DrawParam::default());
        canvas.finish(ctx)
    }
}

fn main() -> GameResult {
    let vehicle: Vehicle = Vehicle {
        max_speed: 10.0,
        max_force: 0.8,
        position: Vector { x: 100.0, y: 300.0 },
        velosity: Vector { x: 0.0, y: 0.0 },
        acceleration: Vector { x: 0.0, y: 0.0 },
        prediction: Vector { x: 0.0, y: 0.0 },
    };
    let target = Target::new();

    let window_mode = conf::WindowMode::default().dimensions(1280.0, 720.0);
    let cb = ContextBuilder::new("ggez_test", "arvwes").window_mode(window_mode);
    let (ctx, event_loop) = cb.build()?;
    let state = GameState {
        veichle: vehicle,
        target: target,
    };
    event::run(ctx, event_loop, state)
}
