use crate::{
    bullet::Bullet,
    upgradeManager::{upgradesList, UpgradeType},
};
use engine::{physics::PhysicsEngine, physics::PhysicsModule, Input, RenderLiteral};
use rand::Rng;
use std::{cell::RefCell, f32::consts::PI, rc::Rc};
use ultraviolet::{Rotor2, Vec2, Vec4};

pub struct Player {
    pub thrust: f32,
    pub physics_module: Rc<RefCell<PhysicsModule>>,
    pub rotation_rps: f32,
    steering_keys: SteeringKeys,
    shooting: Shooting,
    bullets: Vec<Bullet>,
    pub upgrades: Upgrades,
}

impl Player {
    pub fn new(physics_module: Rc<RefCell<PhysicsModule>>) -> Self {
        Self {
            physics_module,
            thrust: 500.,
            rotation_rps: 1.,
            steering_keys: SteeringKeys {
                left: false,
                right: false,
                forward: false,
            },
            shooting: Shooting {
                shootnow: false,
                cooldown: 0.5,
                coolingdown: 0.,
            },
            bullets: Default::default(),
            upgrades: Upgrades::default(),
        }
    }

    pub fn update(&mut self, dt: f32, physics_engine: &mut PhysicsEngine) {
        let mut physics_module = self.physics_module.borrow_mut();

        if self.steering_keys.forward {
            let force = Rotor2::from_angle(physics_module.rotation)
                * Vec2::unit_x()
                * (self.thrust + self.upgrades.thrust_add)
                * self.upgrades.thrust_mult;
            physics_module.force = force;
        }

        physics_module.angular_velocity = match self.steering_keys.direction() {
            SteeringDirection::Left => {
                (-self.rotation_rps - self.upgrades.rotation_add)
                    * self.upgrades.rotation_mult
                    * 2.
                    * PI
            }
            SteeringDirection::Right => {
                (self.rotation_rps + self.upgrades.rotation_add)
                    * self.upgrades.rotation_mult
                    * 2.
                    * PI
            }
            SteeringDirection::None => 0.,
        };
        for i in self.bullets.iter_mut() {
            i.update(dt);
        }

        self.bullets.retain(|a| !a.to_delete);

        if self.shooting.shootnow && self.shooting.coolingdown <= 0. {
            let mut rng = rand::thread_rng();
            for _ in 0..self.upgrades.bullet_per_attack {
                self.bullets.push(Bullet::new(
                    physics_engine.new_module(),
                    physics_module.position,
                    physics_module.rotation
                        + PI * (rng.gen_range(
                            1.0 - self.upgrades.accurancy..=1.0 + self.upgrades.accurancy,
                        ) + 1.),
                    physics_module.velocity,
                    self.upgrades.pierce,
                    self.upgrades.bounce,
                ));
            }

            self.shooting.coolingdown = self.shooting.cooldown;
        }
        self.shooting.coolingdown -= dt;
    }

    pub fn polygons(&self) -> Vec<RenderLiteral> {
        let physics_module = self.physics_module.borrow();
        let mut vect: Vec<RenderLiteral> =
            vec![RenderLiteral::Game(engine::ShapeLiteral::Polygon {
                pos: physics_module.position,
                angles: [0., 2. / 3. * PI, 4. / 3. * PI]
                    .iter()
                    .map(|a| a + physics_module.rotation)
                    .collect(),
                distances: vec![75., 50., 50.],
                border_thickness: 0.,
                colour: Vec4::new(1., 1., 1., 1.),
            })];
        for i in self.bullets.iter() {
            vect.push(i.polygon())
        }
        vect
    }

    pub fn input(&mut self, input: Input) {
        if let Input::Keyboard { key, state } = input {
            match key.to_text() {
                Some("w") => self.steering_keys.forward = state.is_pressed(),
                Some("a") => self.steering_keys.left = state.is_pressed(),
                Some("d") => self.steering_keys.right = state.is_pressed(),
                Some(" ") => self.shooting.shootnow = state.is_pressed(),
                _ => (),
            }
        }
    }
    pub fn upgrade(&mut self, value: &str) {
        let upgrade = upgradesList[value.parse::<usize>().unwrap()].upgrade;
        for i in upgrade {
            match i {
                (UpgradeType::dmg_add, a) => self.upgrades.dmg_add += a,
                (UpgradeType::dmg_mult, a) => self.upgrades.dmg_mult += a,
                (UpgradeType::thrust_add, a) => self.upgrades.thrust_add += a,
                (UpgradeType::thrust_mult, a) => self.upgrades.thrust_mult += a,
                (UpgradeType::rotation_add, a) => self.upgrades.rotation_add += a,
                (UpgradeType::rotation_mult, a) => self.upgrades.rotation_mult += a,
                (UpgradeType::bounce, a) => self.upgrades.bounce += a.round() as i32,
                (UpgradeType::pierce, a) => self.upgrades.pierce += a.round() as i32,
                (UpgradeType::accurancy, a) => self.upgrades.accurancy += a,
                (UpgradeType::bullet_per_attack, a) => {
                    self.upgrades.bullet_per_attack += a.round() as i32
                }
                (UpgradeType::empty, a) => (),
            }
        }
    }
}

struct SteeringKeys {
    left: bool,
    right: bool,
    forward: bool,
}

impl SteeringKeys {
    fn direction(&self) -> SteeringDirection {
        match (self.left, self.right) {
            (true, false) => SteeringDirection::Left,
            (false, true) => SteeringDirection::Right,
            _ => SteeringDirection::None,
        }
    }
}

#[derive(PartialEq)]
enum SteeringDirection {
    Left,
    None,
    Right,
}

struct Shooting {
    shootnow: bool,
    cooldown: f32,
    coolingdown: f32,
}

pub struct Upgrades {
    dmg_add: f32,
    dmg_mult: f32,
    thrust_add: f32,
    thrust_mult: f32,
    rotation_add: f32,
    rotation_mult: f32,
    bounce: i32,
    pierce: i32,
    accurancy: f32,
    bullet_per_attack: i32,
}

impl Default for Upgrades {
    fn default() -> Self {
        Self {
            dmg_add: 0.,
            dmg_mult: 1.,
            thrust_add: 0.,
            thrust_mult: 1.,
            rotation_add: 0.,
            rotation_mult: 1.,
            bounce: 0,
            pierce: 0,
            accurancy: 0.,
            bullet_per_attack: 1,
        }
    }
}
