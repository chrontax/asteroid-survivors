use engine::{
    physics::{CollisionResponse, PhysicsEngine, PhysicsModule},
    RenderLiteral,
};
use std::cell::RefCell;
use std::f32::consts::PI;
use std::rc::Rc;
use ultraviolet::{Rotor2, Vec2, Vec4};
use whoami::distro;

use crate::utils::{hit, HitType};

pub struct Bullet {
    pub physics_module: Rc<RefCell<PhysicsModule<HitType>>>,
    pub distances: Vec<f32>,
    pub angles: Vec<f32>,
    pub timer: f32,
    pub to_delete: bool,
    dmg: f32,
    bounce: i32,
    pierce: i32,
}
impl Bullet {
    pub fn new(
        physics_engine: &mut PhysicsEngine<HitType>,
        postion: Vec2,
        rotation: f32,
        velocity: Vec2,
        dmg: f32,
        bounce: i32,
        pierce: i32,
    ) -> Self {
        let distances: Vec<f32> = vec![10., 10., 10., 10., 10.];
        let angles: Vec<f32> = vec![0., 2. / 5. * PI, 4. / 5. * PI, 6. / 5. * PI, 8. / 5. * PI];
        let mut physics_module = physics_engine.new_module(
            engine::ShapeLiteral::Polygon {
                pos: Vec2::zero(),
                angles: angles.clone(),
                distances: distances.clone(),
                border_thickness: 0.,
                colour: Vec4::new(1., 1., 1., 1.),
            },
            hit,
            HitType::Bullet {
                dmgb: dmg,
                bounce,
                pierce,
            },
            1.,
        );
        physics_module.borrow_mut().position =
            postion + Rotor2::from_angle(rotation) * Vec2::new(75., 0.);
        physics_module.borrow_mut().rotation = rotation;

        let mut physics_module_borowed = physics_module.borrow_mut();
        physics_module_borowed.velocity =
            Rotor2::from_angle(rotation) * Vec2::new(1000., 0.) + velocity;
        drop(physics_module_borowed);

        Self {
            physics_module,
            distances,
            angles,
            timer: 10.,
            to_delete: false,
            dmg,
            bounce,
            pierce,
        }
    }

    pub fn update(&mut self, dt: f32) {
        self.timer -= dt;
        if self.timer < 0. {
            self.to_delete = true
        }
        if let HitType::Bullet { bounce, pierce, .. } = &mut self.physics_module.borrow_mut().inner
        {
            if *bounce >= 0 {
                self.bounce = *bounce;
            }
            if *pierce >= 0 {
                *pierce -= 1;
                self.pierce = *pierce;
            }
            if self.bounce <= 0 && self.pierce <= 0 {
                self.to_delete = true;
            }
        }
    }

    pub fn polygon(&self) -> RenderLiteral {
        let physics_module = self.physics_module.borrow();
        let vect: RenderLiteral = RenderLiteral::Game(engine::ShapeLiteral::Polygon {
            pos: physics_module.position,
            angles: self.angles.clone(),
            distances: self.distances.clone(),
            border_thickness: 0.,
            colour: Vec4::new(1., 1., 1., 1.),
        });
        vect
    }
}
