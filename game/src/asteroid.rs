use engine::{physics::PhysicsModule, RenderLiteral};
use rand::thread_rng;
use rand::Rng;
use rand_distr::{Distribution, Normal};
use std::{cell::RefCell, f32::consts::PI, rc::Rc};
use ultraviolet::{Vec2, Vec4};

const MIN_VERTICES: f32 = 5.;

pub struct Asteroid {
    pub physics_module: Rc<RefCell<PhysicsModule>>,
    pub distances: Vec<f32>,
    pub angles: Vec<f32>,
    avel: f32,
    vel: Vec2,
    timer: f32,
    pub to_delete: bool,
}
impl Asteroid {
    pub fn new(physics_module: Rc<RefCell<PhysicsModule>>, postion: Vec2) -> Self {
        // let len = rand::thread_rng().gen_range(4..20);
        // let mut points: Vec<(f32, f32)> = Vec::with_capacity(len);
        // let mut last: f32 = 0.1;
        // for _ in 0..len {
        //     let distance = rand::thread_rng().gen_range(20f32..300f32);
        //     let angle = rand::thread_rng().gen_range(last..2f32 * PI);
        //     last = angle;
        //     points.push((distance, angle));
        // }
        // dbg!(&points);
        let mut last = 0.;
        let mut points = Vec::new();
        while last < 2. * PI {
            last += rand::thread_rng().gen_range(0.1..(2. * PI / MIN_VERTICES));
            points.push((rand::thread_rng().gen_range(20.0..100.), last));
        }

        // Sort points by angle to ensure a non-overlapping polygon
        // points.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());

        // Extract distances and angles from sorted points
        let distances: Vec<f32> = points.iter().map(|(dist, _)| *dist).collect();
        let angles: Vec<f32> = points.iter().map(|(_, angle)| *angle).collect();
        physics_module.borrow_mut().position = postion;

        let mean = 0.0_f32;
        let std_dev = 100.0_f32; // Adjust for your desired spread
        let normal = Normal::new(mean, std_dev).unwrap();

        let mut rng = thread_rng();
        let value: f32 = normal.sample(&mut rng).clamp(-900.0_f32, 900.0_f32);
        let value2: f32 = normal.sample(&mut rng).clamp(-900.0_f32, 900.0_f32);

        Self {
            physics_module,
            distances,
            angles,
            avel: rand::thread_rng().gen_range(-1f32..1f32) * PI,
            vel: Vec2 {
                x: value,
                y: value2,
            },
            timer: rand::thread_rng().gen_range(2000f32..10000f32),
            to_delete: false,
        }
    }

    pub fn update(&mut self, dt: f32) {
        let mut physics_module = self.physics_module.borrow_mut();
        physics_module.angular_velocity = self.avel;
        physics_module.force = self.vel;
        self.timer -= dt;
        if self.timer < 0. {
            self.to_delete = true
        }
    }

    pub fn polygon(&self) -> Vec<RenderLiteral> {
        let physics_module = self.physics_module.borrow();
        let vect: Vec<RenderLiteral> = vec![RenderLiteral::Game(engine::ShapeLiteral::Polygon {
            pos: physics_module.position,
            angles: self
                .angles
                .iter()
                .map(|a| a + physics_module.rotation)
                .collect(),
            distances: self.distances.clone(),
            border_thickness: 0.,
            colour: Vec4::new(1., 0., 0., 1.),
        })];
        return vect;
    }
}
