use std::f32::consts::PI;

use engine::{
    run_game, EngineInitInfo, EverythingToDraw, Game as GameTrait, Input, RenderLiteral,
    ShapeLiteral,
};
use winit::dpi::PhysicalSize;

fn main() {
    run_game::<Game>().unwrap();
}

struct Game {
    rotation: f32,
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
            Self { rotation: 0. },
        )
    }

    fn draw(&self) -> EverythingToDraw {
        let rotation = self.rotation * PI;
        EverythingToDraw {
            scale: 1.,
            camera_pos: [0., 0.],
            colour: [1., 1., 1., 1.],
            inverted: false,
            shapes: vec![RenderLiteral::Game(ShapeLiteral::Polygon {
                pos: [0., 0.],
                angles: vec![
                    0. + rotation,
                    2. / 3. * PI + rotation,
                    4. / 3. * PI + rotation,
                ],
                distances: vec![100., 100., 100.],
                border_thickness: 0.,
            })],
        }
    }

    fn update(&mut self, dt: f32) {
        self.rotation = (self.rotation + dt) % 2.;
    }

    fn input(&mut self, _input: Input) {}
}
