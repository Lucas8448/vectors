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

    fn scale(self, scalar: f64) -> Vec2 {
        Vec2::new(self.x * scalar, self.y * scalar)
    }
}

struct Particle {
    pos: Vec2,
    vel: Vec2,
}

impl Particle {
    fn new(x: f64, y: f64, vx: f64, vy: f64) -> Self {
        Self {
            pos: Vec2::new(x, y),
            vel: Vec2::new(vx, vy),
        }
    }

    fn update(&mut self, dt: f64, bounds: (f64, f64)) {
        self.pos = self.pos.add(self.vel.scale(dt));

        if self.pos.x <= 0.0 || self.pos.x >= bounds.0 {
            self.vel.x = -self.vel.x;
        }
        if self.pos.y <= 0.0 || self.pos.y >= bounds.1 {
            self.vel.y = -self.vel.y;
        }
    }
}

fn main() {
    let bounds = (80.0, 24.0);
    let mut particles = vec![
        Particle::new(10.0, 5.0, 5.0, 3.0),
        Particle::new(30.0, 10.0, -2.0, 4.0),
        Particle::new(60.0, 15.0, -3.0, -2.0),
        Particle::new(20.0, 20.0, 4.0, -3.0),
        Particle::new(50.0, 8.0, -4.0, 2.0),
        Particle::new(70.0, 12.0, 3.0, -4.0),
        Particle::new(40.0, 18.0, 2.0, 3.0),
    ];

    let dt = 0.1;
    loop {
        print!("\x1B[2J\x1B[1;1H");

        let mut grid = vec![vec![' '; bounds.0 as usize]; bounds.1 as usize];

        for p in particles.iter_mut() {
            p.update(dt, bounds);
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