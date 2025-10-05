use macroquad::prelude::*;

const GRAVITY: f64 = 500.0;

#[derive(Clone, Copy, Debug)]
struct Vec2f {
    x: f64,
    y: f64,
}

impl Vec2f {
    fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }

    fn add(self, other: Vec2f) -> Vec2f {
        Vec2f::new(self.x + other.x, self.y + other.y)
    }

    fn sub(self, other: Vec2f) -> Vec2f {
        Vec2f::new(self.x - other.x, self.y - other.y)
    }

    fn scale(self, s: f64) -> Vec2f {
        Vec2f::new(self.x * s, self.y * s)
    }

    fn dot(self, other: Vec2f) -> f64 {
        self.x * other.x + self.y * other.y
    }

    fn magnitude(self) -> f64 {
        (self.x * self.x + self.y * self.y).sqrt()
    }

    fn normalize(self) -> Vec2f {
        let m = self.magnitude();
        if m == 0.0 { self } else { self.scale(1.0 / m) }
    }
}

struct Particle {
    pos: Vec2f,
    vel: Vec2f,
    radius: f64,
    color: Color,
}

impl Particle {
    fn new(x: f64, y: f64, vx: f64, vy: f64, radius: f64, color: Color) -> Self {
        Self {
            pos: Vec2f::new(x, y),
            vel: Vec2f::new(vx, vy),
            radius,
            color,
        }
    }

    fn update(&mut self, dt: f64, bounds: (f64, f64)) {
        self.vel.y += GRAVITY * dt;

        self.pos = self.pos.add(self.vel.scale(dt));

        if self.pos.x - self.radius < 0.0 {
            self.pos.x = self.radius;
            self.vel.x = -self.vel.x;
        }
        if self.pos.x + self.radius > bounds.0 as f64 {
            self.pos.x = bounds.0 as f64 - self.radius;
            self.vel.x = -self.vel.x;
        }
        if self.pos.y - self.radius < 0.0 {
            self.pos.y = self.radius;
            self.vel.y = -self.vel.y;
        }
        if self.pos.y + self.radius > bounds.1 as f64 {
            self.pos.y = bounds.1 as f64 - self.radius;
            self.vel.y = -self.vel.y;
        }
    }

    fn draw(&self) {
        draw_circle(
            self.pos.x as f32,
            self.pos.y as f32,
            self.radius as f32,
            self.color,
        );
    }
}

fn resolve_collision(a: &mut Particle, b: &mut Particle) {
    let normal = a.pos.sub(b.pos);
    let dist = normal.magnitude();
    let min_dist = a.radius + b.radius;

    if dist == 0.0 || dist >= min_dist {
        return;
    }

    let n = normal.normalize();
    let rv = a.vel.sub(b.vel);
    let vel_along_normal = rv.dot(n);

    if vel_along_normal > 0.0 {
        return;
    }

    let restitution = 1.0;
    let j = -(1.0 + restitution) * vel_along_normal / 2.0;

    let impulse = n.scale(j);
    a.vel = a.vel.add(impulse);
    b.vel = b.vel.sub(impulse);

    let percent = 0.8;
    let correction = n.scale((min_dist - dist) * percent / 2.0);
    a.pos = a.pos.add(correction);
    b.pos = b.pos.sub(correction);
}

#[macroquad::main("Particle Simulation")]
async fn main() {
    let screen_w = 800.0;
    let screen_h = 600.0;
    let mut particles = vec![
        Particle::new(100.0, 100.0, 200.0, 80.0, 10.0, RED),
        Particle::new(400.0, 300.0, -150.0, 60.0, 10.0, BLUE),
        Particle::new(600.0, 200.0, -200.0, -100.0, 10.0, GREEN),
        Particle::new(200.0, 400.0, 120.0, -150.0, 10.0, YELLOW),
        Particle::new(300.0, 500.0, 100.0, -120.0, 10.0, ORANGE),
        Particle::new(700.0, 100.0, -180.0, 90.0, 10.0, PURPLE),
        Particle::new(500.0, 400.0, 160.0, -70.0, 10.0, PINK),
        Particle::new(350.0, 150.0, 140.0, 130.0, 10.0, MAGENTA),
        Particle::new(450.0, 350.0, -160.0, -90.0, 10.0, LIGHTGRAY),
    ];

    loop {
        let dt = get_frame_time() as f64;
        let bounds = (screen_w, screen_h);

        for p in particles.iter_mut() {
            p.update(dt, bounds);
        }

        let len = particles.len();
        for i in 0..len {
            for j in i + 1..len {
                let (left, right) = particles.split_at_mut(j);
                resolve_collision(&mut left[i], &mut right[0]);
            }
        }

        clear_background(BLACK);
        for p in &particles {
            p.draw();
        }

        next_frame().await;
    }
}