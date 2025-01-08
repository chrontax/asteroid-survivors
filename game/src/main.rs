use std::f32::consts::PI;

use engine::{
    run_game, EngineInitInfo, EverythingToDraw, Game as GameTrait, Input, RenderLiteral,
    ShapeLiteral,
};
use winit::{dpi::PhysicalSize, event::ElementState, keyboard::Key};

fn main() {
    run_game::<Game>().unwrap();
}

struct Game {
    rotation: f32,
    position: [f32; 2],
    cam_position: [f32; 2],
    stering_direction: SteeringDirection,
    steering_keys: SteeringKeys,
}

struct SteeringKeys {
    left: bool,
    right: bool,
    forward: bool,
}

#[derive(PartialEq)]
enum SteeringDirection {
    Left,
    None,
    Right,
}

impl GameTrait for Game {
    fn init() -> (EngineInitInfo, Self) {
        (
            EngineInitInfo {
                windowed: true,
                resizeable: false,
                resolution: PhysicalSize {
                    width: 1280,
                    height: 720,
                },
            },
            Self {
                rotation: 0.,
                position: Default::default(),
                cam_position: Default::default(),
                stering_direction: SteeringDirection::None,
                steering_keys: SteeringKeys {
                    left: false,
                    right: false,
                    forward: false,
                },
            },
        )
    }

    fn draw(&self) -> EverythingToDraw {
        let rotation = self.rotation * PI;
        EverythingToDraw {
            scale: 1.,
            camera_pos: self.cam_position,
            colour: [1., 1., 1., 1.],
            inverted: false,
            shapes: vec![
                RenderLiteral::Game(ShapeLiteral::Polygon {
                    pos: self.position,
                    angles: vec![
                        0. + rotation,
                        2. / 3. * PI + rotation,
                        4. / 3. * PI + rotation,
                    ],
                    distances: vec![75., 50., 50.],
                    border_thickness: 0.,
                }),
                RenderLiteral::Game(ShapeLiteral::Polygon {
                    pos: [-200., 200.],
                    angles: vec![0., 2. / 3. * PI, 4. / 3. * PI, 6. / 3. * PI],
                    distances: vec![50., 50., 50., 50.],
                    border_thickness: 0.,
                }),
                RenderLiteral::Game(ShapeLiteral::Polygon {
                    pos: [200., 200.],
                    angles: vec![0., 2. / 3. * PI, 4. / 3. * PI, 6. / 3. * PI],
                    distances: vec![150., 50., 50., 50.],
                    border_thickness: 0.,
                }),
            ],
        }
    }

    fn update(&mut self, dt: f32) {
        let speed: f32 = 5.;
        self.rotation = match self.stering_direction {
            SteeringDirection::Left => (self.rotation + dt * 1.5) % 2.,
            SteeringDirection::Right => (self.rotation - dt * 1.5) % 2.,
            SteeringDirection::None => self.rotation,
        };

        self.cam_position = self.position;

        if self.steering_keys.forward {
            let angl = (self.rotation + 1.5) * PI;
            self.position[0] += speed * angl.cos();
            self.position[1] += speed * angl.sin() * -1.;
        }
    }

    fn input(&mut self, input: Input) {
        if let Input::Keyboard { key, state } = input {
            // zmienic to na match moze
            //
            match state {
                ElementState::Pressed => match key.to_text() {
                    Some("w") => self.steering_keys.forward = true,
                    Some("a") => self.steering_keys.left = true,
                    Some("d") => self.steering_keys.right = true,
                    _ => (),
                },
                ElementState::Released => match key.to_text() {
                    Some("w") => self.steering_keys.forward = false,
                    Some("a") => self.steering_keys.left = false,
                    Some("d") => self.steering_keys.right = false,
                    _ => (),
                },
            };
        }
        self.stering_direction = match self.steering_keys {
            SteeringKeys {
                left: true,
                right: false,
                ..
            } => SteeringDirection::Left,
            SteeringKeys {
                left: false,
                right: true,
                ..
            } => SteeringDirection::Right,
            _ => SteeringDirection::None,
        };
    }
}
