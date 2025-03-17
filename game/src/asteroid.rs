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

    pub timer: f32,
    pub to_delete: bool,
    pub resorces: String,
}
impl Asteroid {
    pub fn new(physics_module: Rc<RefCell<PhysicsModule>>, postion: Vec2) -> Self {
        let mut last = 0.;
        let mut points = Vec::new();
        while last < 2. * PI {
            last += rand::thread_rng().gen_range(0.1..(2. * PI / MIN_VERTICES));
            points.push((rand::thread_rng().gen_range(20.0..100.), last));
        }

        let distances: Vec<f32> = points.iter().map(|(dist, _)| *dist).collect();
        let angles: Vec<f32> = points.iter().map(|(_, angle)| *angle).collect();
        physics_module.borrow_mut().position = postion;

        let mean = 0.0_f32;
        let std_dev = 100.0_f32;
        let normal = Normal::new(mean, std_dev).unwrap();

        let mut rng = thread_rng();
        let value: f32 = normal.sample(&mut rng).clamp(-900.0_f32, 900.0_f32);
        let value2: f32 = normal.sample(&mut rng).clamp(-900.0_f32, 900.0_f32);

        let mut physics_module_borowed = physics_module.borrow_mut();
        physics_module_borowed.angular_velocity = rand::thread_rng().gen_range(-1f32..1f32) * PI;
        physics_module_borowed.velocity = Vec2 {
            x: value,
            y: value2,
        };
        drop(physics_module_borowed);

        Self {
            physics_module,
            distances,
            angles,
            timer: rand::thread_rng().gen_range(2000f32..10000f32),
            to_delete: false,
            resorces: "None".to_string(),
        }
    }

    pub fn update(&mut self, dt: f32) {
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
