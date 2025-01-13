use std::{cell::RefCell, f32::consts::PI, rc::Rc};

use engine::{physics::PhysicsModule, Input, RenderLiteral};
use ultraviolet::{Rotor2, Vec2};

pub struct Player {
    rotation: f32,
    pub thrust: f32,
    pub physics_module: Rc<RefCell<PhysicsModule>>,
    steering_keys: SteeringKeys,
}

impl Player {
    pub fn new(physics_module: Rc<RefCell<PhysicsModule>>) -> Self {
        Self {
            rotation: 0.,
            physics_module,
            thrust: 100.,
            steering_keys: SteeringKeys {
                left: false,
                right: false,
                forward: false,
            },
        }
    }

    pub fn update(&mut self, dt: f32) {
        let mut physics_module = self.physics_module.borrow_mut();

        if self.steering_keys.forward {
            physics_module.force = Rotor2::from_angle(self.rotation) * Vec2::unit_x() * self.thrust;
        }

        self.rotation = match self.steering_keys.direction() {
            SteeringDirection::Left => self.rotation - dt,
            SteeringDirection::Right => self.rotation + dt,
            SteeringDirection::None => self.rotation,
        } % (2. * PI);
    }

    pub fn polygons(&self) -> Vec<RenderLiteral> {
        vec![RenderLiteral::Game(engine::ShapeLiteral::Polygon {
            pos: self.physics_module.borrow().position.into(),
            angles: [0., 2. / 3. * PI, 4. / 3. * PI]
                .iter()
                .map(|a| a + self.rotation)
                .collect(),
            distances: vec![75., 50., 50.],
            border_thickness: 0.,
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
