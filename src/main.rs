use std::{thread, time::Duration};

#[derive(Clone, Copy, Debug)]
struct Vec2 {
    x: f64,
    y: f64,
}

impl Vec2 {
    fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }

    fn add(self, other: Vec2) -> Vec2 {
        Vec2::new(self.x + other.x, self.y + other.y)
    }

    fn sub(self, other: Vec2) -> Vec2 {
        Vec2::new(self.x - other.x, self.y - other.y)
    }

    fn scale(self, s: f64) -> Vec2 {
        Vec2::new(self.x * s, self.y * s)
    }

    fn dot(self, other: Vec2) -> f64 {
        self.x * other.x + self.y * other.y
    }

    fn magnitude(self) -> f64 {
        (self.x * self.x + self.y * self.y).sqrt()
    }

    fn normalize(self) -> Vec2 {
        let m = self.magnitude();
        if m == 0.0 { self } else { self.scale(1.0 / m) }
    }
}

struct Particle {
    pos: Vec2,
    vel: Vec2,
    radius: f64,
}

impl Particle {
    fn new(x: f64, y: f64, vx: f64, vy: f64, radius: f64) -> Self {
        Self {
            pos: Vec2::new(x, y),
            vel: Vec2::new(vx, vy),
            radius,
        }
    }

    fn update(&mut self, dt: f64, bounds: (f64, f64)) {
        self.pos = self.pos.add(self.vel.scale(dt));

        if self.pos.x - self.radius <= 0.0 || self.pos.x + self.radius >= bounds.0 {
            self.vel.x = -self.vel.x;
        }
        if self.pos.y - self.radius <= 0.0 || self.pos.y + self.radius >= bounds.1 {
            self.vel.y = -self.vel.y;
        }
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

fn main() {
    let bounds = (80.0, 24.0);
    let mut particles = vec![
        Particle::new(10.0, 5.0, 6.0, 2.0, 1.0),
        Particle::new(30.0, 10.0, -3.0, 1.0, 1.0),
        Particle::new(60.0, 15.0, -5.0, -2.0, 1.0),
        Particle::new(40.0, 8.0, 2.0, -4.0, 1.0),
        Particle::new(20.0, 16.0, 0.0, 5.0, 1.0),
    ];

    let dt = 0.1;

    loop {
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

        print!("\x1B[2J\x1B[1;1H");
        let mut grid = vec![vec![' '; bounds.0 as usize]; bounds.1 as usize];
        for p in &particles {
            let x = p.pos.x as usize;
            let y = p.pos.y as usize;
            if y < grid.len() && x < grid[0].len() {
                grid[y][x] = '*';
            }
        }
        for row in grid {
            println!("{}", row.into_iter().collect::<String>());
        }

        thread::sleep(Duration::from_millis(50));
    }
}