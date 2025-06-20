use crate::{
    bullet::Bullet,
    res::SHOOT,
    upgradeManager::{UpgradeType, UPGRADES},
    utils::{get_orb, hit, HitType},
};
use engine::{
    audio::{self, AudioEngine, AudioPlayer},
    physics::{PhysicsEngine, PhysicsModule},
    Input, RenderLiteral,
};
use rand::Rng;
use std::{cell::RefCell, f32::consts::PI, rc::Rc};
use ultraviolet::{Rotor2, Vec2, Vec4};

pub struct Player {
    thrust: f32,
    pub physics_module: Rc<RefCell<PhysicsModule<HitType>>>,
    rotation_rps: f32,
    steering_keys: SteeringKeys,
    shooting: Shooting,
    bullets: Vec<Bullet>,
    pub upgrades: Upgrades,
    pub health: f32,
    pub max_health: f32,
    pub shield: f32,
    pub max_shield: f32,
    audio: engine::audio::AudioPlayer,
}

impl Player {
    pub fn new(physics_module: &mut PhysicsEngine<HitType>, audio: AudioPlayer) -> Self {
        let physics_module = physics_module.new_module(
            engine::ShapeLiteral::Polygon {
                pos: Vec2::zero(),
                angles: vec![0., 2. / 3. * PI, 4. / 3. * PI],
                distances: vec![75., 50., 50.],
                border_thickness: 0.,
                colour: Vec4::new(1., 1., 1., 1.),
            },
            &hit,
            HitType::Player {
                dmgp: 50.,
                dmg_takenp: 0.,
            },
            50.,
        );
        Self {
            physics_module,
            thrust: 12500.,
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
            health: 100.,
            max_health: 100.,
            shield: 0.,
            max_shield: 10.,
            audio,
        }
    }

    pub fn update(&mut self, dt: f32, physics_engine: &mut PhysicsEngine<HitType>) {
        if self.shield < self.max_shield {
            self.shield += dt * 0.01 * self.max_shield;
        }
        let mut physics_module = self.physics_module.borrow_mut();
        if let HitType::Player { dmg_takenp, .. } = &mut physics_module.inner {
            if self.shield > 0. {
                self.shield -= *dmg_takenp;
            } else {
                self.health -= *dmg_takenp;
            }
            *dmg_takenp = 0.;
        }

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
            self.audio.play(SHOOT.to_vec());
            let mut rng = rand::thread_rng();
            for _ in 0..self.upgrades.bullet_per_attack {
                self.bullets.push(Bullet::new(
                    physics_engine,
                    physics_module.position,
                    physics_module.rotation
                        + PI * (rng.gen_range(
                            1.0 - self.upgrades.accurancy..=1.0 + self.upgrades.accurancy,
                        ) + 1.),
                    physics_module.velocity,
                    (self.upgrades.dmg_add + 10.) * self.upgrades.dmg_mult,
                    self.upgrades.bounce + 1,
                    self.upgrades.pierce,
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
        if self.shield > 0. {
            vect.push(RenderLiteral::Game(engine::ShapeLiteral::Polygon {
                pos: physics_module.position,
                angles: [0., 2. / 3. * PI, 4. / 3. * PI]
                    .iter()
                    .map(|a| a + physics_module.rotation)
                    .collect(),
                distances: vec![75., 50., 50.]
                    .iter()
                    .map(|f| f * (self.shield / self.max_shield))
                    .collect(),
                border_thickness: 0.,
                colour: Vec4::new(0., 0., 1., 1.),
            }));
        }
        vect.push(get_orb(
            physics_module.position,
            self.health / self.max_health,
            9.,
        ));
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
        let upgrade = UPGRADES[value.parse::<usize>().unwrap()].upgrade;
        for i in upgrade {
            match i {
                (UpgradeType::DmgAdd, a) => self.upgrades.dmg_add += a,
                (UpgradeType::DmgMult, a) => self.upgrades.dmg_mult += a,
                (UpgradeType::ThrustAdd, a) => self.upgrades.thrust_add += a,
                (UpgradeType::ThrustMult, a) => self.upgrades.thrust_mult += a,
                (UpgradeType::RotationAdd, a) => self.upgrades.rotation_add += a,
                (UpgradeType::RotationMult, a) => self.upgrades.rotation_mult += a,
                (UpgradeType::Bounce, a) => self.upgrades.bounce += a.round() as i32,
                (UpgradeType::Pierce, a) => self.upgrades.pierce += a.round() as i32,
                (UpgradeType::Accurancy, a) => self.upgrades.accurancy += a,
                (UpgradeType::BulletPerAttack, a) => {
                    self.upgrades.bullet_per_attack += a.round() as i32
                }
                (UpgradeType::Empty, _) => (),
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
            pierce: 1,
            accurancy: 0.,
            bullet_per_attack: 1,
        }
    }
}
