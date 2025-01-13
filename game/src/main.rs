use std::{cell::RefCell, f32::consts::PI, rc::Rc};

use engine::{
    physics::{PhysicsEngine, PhysicsModule},
    run_game, EngineInitInfo, EverythingToDraw, Game as GameTrait, Input, RenderLiteral,
    ShapeLiteral,
};
use player::Player;
use ultraviolet::{Rotor2, Vec2};
use winit::{dpi::PhysicalSize, event::ElementState, keyboard::Key};

mod player;

fn main() {
    run_game::<Game>().unwrap();
}

struct Game {
    cam_position: [f32; 2],
    physics: PhysicsEngine,
    player: Player,
}

impl GameTrait for Game {
    fn init() -> (EngineInitInfo, Self) {
        let mut physics = PhysicsEngine::default();
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
                cam_position: Default::default(),
                player: Player::new(physics.new_module()),
                physics,
            },
        )
    }

    fn draw(&self) -> EverythingToDraw {
        let mut shapes = vec![
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
        ];
        shapes.append(&mut self.player.polygons());
        EverythingToDraw {
            scale: 1.,
            camera_pos: self.cam_position,
            colour: [1., 1., 1., 1.],
            inverted: false,
            shapes,
        }
    }

    fn update(&mut self, dt: f32) {
        self.cam_position = self.player.physics_module.borrow().position.into();

        self.player.update(dt);

        self.physics.update(dt);
    }

    fn input(&mut self, input: Input) {
        self.player.input(input);
    }
}
