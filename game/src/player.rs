use std::{cell::RefCell, f32::consts::PI, rc::Rc};

use engine::{physics::PhysicsModule, Input, RenderLiteral};
use ultraviolet::{Rotor2, Vec2, Vec4};

pub struct Player {
    pub thrust: f32,
    pub physics_module: Rc<RefCell<PhysicsModule>>,
    pub rotation_rps: f32,
    steering_keys: SteeringKeys,
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
        }
    }

    pub fn update(&mut self, _dt: f32) {
        let mut physics_module = self.physics_module.borrow_mut();

        if self.steering_keys.forward {
            physics_module.force =
                Rotor2::from_angle(physics_module.rotation) * Vec2::unit_x() * self.thrust;
        }

        physics_module.angular_velocity = match self.steering_keys.direction() {
            SteeringDirection::Left => -self.rotation_rps * 2. * PI,
            SteeringDirection::Right => self.rotation_rps * 2. * PI,
            SteeringDirection::None => 0.,
        };
    }

    pub fn polygons(&self) -> Vec<RenderLiteral> {
        let physics_module = self.physics_module.borrow();
        vec![RenderLiteral::Game(engine::ShapeLiteral::Polygon {
            pos: physics_module.position,
            angles: [0., 2. / 3. * PI, 4. / 3. * PI]
                .iter()
                .map(|a| a + physics_module.rotation)
                .collect(),
            distances: vec![75., 50., 50.],
            border_thickness: 0.,
            colour: Vec4::new(1., 0., 0., 1.),
        })]
    }

    pub fn input(&mut self, input: Input) {
        if let Input::Keyboard { key, state } = input {
            match key.to_text() {
                Some("w") => self.steering_keys.forward = state.is_pressed(),
                Some("a") => self.steering_keys.left = state.is_pressed(),
                Some("d") => self.steering_keys.right = state.is_pressed(),
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
