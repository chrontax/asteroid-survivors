use std::{cell::RefCell, f32::consts::PI, rc::Rc};

use engine::{physics::PhysicsModule, Input, RenderLiteral};
use ultraviolet::{Rotor2, Vec2, Vec4};

pub struct Player {
    pub thrust: f32,
    pub physics_module: Rc<RefCell<PhysicsModule>>,
    pub rotation_rps: f32,
    steering_keys: SteeringKeys,
    shooting: Shooting,
    bullets: Vec<RenderLiteral>,
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
                cooldown: 1.,
                coolingdown: 0.,
            },
            bullets: Default::default(),
        }
    }

    pub fn update(&mut self, dt: f32) {
        let mut physics_module = self.physics_module.borrow_mut();

        if self.steering_keys.forward {
            // if physics_module.force.mag() > 1000. {}
            // physics_module.force =
            let force = Rotor2::from_angle(physics_module.rotation) * Vec2::unit_x() * self.thrust;
            if force.mag() <= 1000. {
                physics_module.force = force;
            }
        }

        physics_module.angular_velocity = match self.steering_keys.direction() {
            SteeringDirection::Left => -self.rotation_rps * 2. * PI,
            SteeringDirection::Right => self.rotation_rps * 2. * PI,
            SteeringDirection::None => 0.,
        };

        if self.shooting.shootnow && self.shooting.coolingdown <= 0. {
            // spawn bullet
            self.shooting.coolingdown = self.shooting.cooldown;
            dbg!("pow");
        }
        self.shooting.coolingdown -= dt;
    }

    pub fn polygons(&self) -> Vec<RenderLiteral> {
        let physics_module = self.physics_module.borrow();
        let vect: Vec<RenderLiteral> = vec![RenderLiteral::Game(engine::ShapeLiteral::Polygon {
            pos: physics_module.position,
            angles: [0., 2. / 3. * PI, 4. / 3. * PI]
                .iter()
                .map(|a| a + physics_module.rotation)
                .collect(),
            distances: vec![75., 50., 50.],
            border_thickness: 0.,
            colour: Vec4::new(1., 1., 1., 1.),
        })];
        return vect;
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
