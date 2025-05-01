use crate::utils::{get_color_from_resource_type, hit, HitType};
use engine::physics::PhysicsEngine;
use engine::{physics::PhysicsModule, RenderLiteral};
use rand::thread_rng;
use rand::Rng;
use rand_distr::{Distribution, Normal};
use std::{cell::RefCell, f32::consts::PI, rc::Rc};
use ultraviolet::{Vec2, Vec4};

use crate::upgradeManager::ResourceType;

const MIN_VERTICES: f32 = 5.;

pub struct Asteroid {
    pub physics_module: Rc<RefCell<PhysicsModule<HitType>>>,
    pub distances: Vec<f32>,
    pub angles: Vec<f32>,
    pub timer: f32,
    pub to_delete: bool,
    pub resorces: (ResourceType, i32),
    pub max_health: f32,
    pub health: f32,
}
impl Asteroid {
    pub fn new(physics_engine: &mut PhysicsEngine<HitType>, postion: Vec2) -> Self {
        let mut last = 0.;
        let mut points = Vec::new();
        while last < 2. * PI {
            points.push((rand::thread_rng().gen_range(20.0..100.), last));
            last += rand::thread_rng().gen_range(0.1..(2. * PI / MIN_VERTICES));
        }
        let physics_module: Rc<RefCell<PhysicsModule<HitType>>> = physics_engine.new_module(
            engine::ShapeLiteral::Polygon {
                pos: Vec2::zero(),
                angles: points.iter().map(|(_, angle)| *angle).collect(),
                distances: points.iter().map(|(dist, _)| *dist).collect(),
                border_thickness: 0.,
                colour: get_color_from_resource_type(ResourceType::Asteroid),
            },
            &hit,
            HitType::Asteroid {
                dmg: 100.,
                dmg_taken: 0.,
            },
            rand::thread_rng().gen_range(100. ..2000.),
        );

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
        let mean2 = 50.0_f32;
        let std_dev2 = 10.0_f32;
        let normal2 = Normal::new(mean2, std_dev2).unwrap();
        let heal: f32 = normal2.sample(&mut rng).clamp(10.0_f32, 100.0_f32);
        Self {
            physics_module,
            distances,
            angles,
            timer: rand::thread_rng().gen_range(100f32..1000f32),
            to_delete: false,
            resorces: (rand::random(), rand::thread_rng().gen_range(10..200)),
            max_health: heal,
            health: heal,
        }
    }

    // pub fn new(physics_engine: &mut PhysicsEngine<HitType>, position: Vec2) -> Self {
    //     let mut points = vec![
    //         (50.0, 0.0),            // Right
    //         (50.0, PI / 2.0),       // Up
    //         (50.0, PI),             // Left
    //         (50.0, 3.0 * PI / 2.0), // Down
    //     ];

    //     let physics_module: Rc<RefCell<PhysicsModule<HitType>>> = physics_engine.new_module(
    //         engine::ShapeLiteral::Polygon {
    //             pos: Vec2::zero(),
    //             angles: points.iter().map(|(_, angle)| *angle).collect(),
    //             distances: points.iter().map(|(dist, _)| *dist).collect(),
    //             border_thickness: 0.,
    //             colour: get_color_from_resource_type(ResourceType::Asteroid),
    //         },
    //         &hit,
    //         HitType::Asteroid {
    //             dmg: 100.,
    //             dmg_taken: 0.,
    //         },
    //         100.,
    //     );

    //     let distances: Vec<f32> = points.iter().map(|(dist, _)| *dist).collect();
    //     let angles: Vec<f32> = points.iter().map(|(_, angle)| *angle).collect();
    //     physics_module.borrow_mut().position = position;

    //     let mean = 0.0_f32;
    //     let std_dev = 100.0_f32;
    //     let normal = Normal::new(mean, std_dev).unwrap();
    //     let mut rng = thread_rng();
    //     let value: f32 = normal.sample(&mut rng).clamp(-900.0, 900.0);
    //     let value2: f32 = normal.sample(&mut rng).clamp(-900.0, 900.0);

    //     {
    //         let mut physics_module_borrowed = physics_module.borrow_mut();
    //         physics_module_borrowed.angular_velocity = rand::thread_rng().gen_range(-1.0..1.0) * PI;
    //         physics_module_borrowed.velocity = Vec2 {
    //             x: value,
    //             y: value2,
    //         };
    //     }

    //     let mean2 = 50.0_f32;
    //     let std_dev2 = 10.0_f32;
    //     let normal2 = Normal::new(mean2, std_dev2).unwrap();
    //     let heal: f32 = normal2.sample(&mut rng).clamp(10.0, 100.0);

    //     Self {
    //         physics_module,
    //         distances,
    //         angles,
    //         timer: rand::thread_rng().gen_range(2.0..100.0),
    //         to_delete: false,
    //         resorces: (rand::random(), rand::thread_rng().gen_range(100..2000)),
    //         max_health: heal,
    //         health: heal - (5.0 * (heal / 10.0)),
    //     }
    // }

    pub fn update(&mut self, dt: f32) {
        self.timer -= dt;
        if self.timer < 0. {
            self.to_delete = true
        }
        if let HitType::Asteroid { dmg_taken, .. } = &mut self.physics_module.borrow_mut().inner {
            if *dmg_taken >= 0. {
                self.health -= *dmg_taken;
                *dmg_taken = 0.;
            }
            if self.health <= 0. {
                self.to_delete = true;
            }
        }
    }

    pub fn polygon(&self) -> Vec<RenderLiteral> {
        let physics_module = self.physics_module.borrow();
        let mut vect: Vec<RenderLiteral> = vec![
            RenderLiteral::Game(engine::ShapeLiteral::Polygon {
                pos: physics_module.position,
                angles: self
                    .angles
                    .iter()
                    .map(|a| a + physics_module.rotation)
                    .collect(),
                distances: self.distances.clone(),
                border_thickness: 0.,
                colour: get_color_from_resource_type(self.resorces.0.clone()),
            }),
            RenderLiteral::Game(engine::ShapeLiteral::Polygon {
                pos: physics_module.position,
                angles: self
                    .angles
                    .iter()
                    .map(|a| a + physics_module.rotation)
                    .collect(),
                distances: self
                    .distances
                    .clone()
                    .iter()
                    .map(|f| f * (self.health / self.max_health))
                    .collect(),
                border_thickness: 0.,
                colour: get_color_from_resource_type(self.resorces.0.clone()),
            }),
        ];

        vect
    }
}
