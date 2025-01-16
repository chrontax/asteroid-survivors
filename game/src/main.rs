use std::f32::consts::PI;

use engine::{
    physics::PhysicsEngine, run_game, EngineInitInfo, EverythingToDraw, Game as GameTrait, Input,
    RenderLiteral, ShapeLiteral,
};
use player::Player;
use ultraviolet::{Vec2, Vec4};
use winit::dpi::PhysicalSize;

mod player;

fn main() {
    run_game::<Game>().unwrap();
}

struct Game {
    cam_position: Vec2,
    physics: PhysicsEngine,
    player: Player,
    speed: f32,
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
                speed: Default::default(),
            },
        )
    }

    fn draw(&self) -> EverythingToDraw {
        let mut shapes = vec![
            RenderLiteral::Game(ShapeLiteral::Polygon {
                // pos: [-200., 200.],
                pos: Vec2::new(-200., 200.),
                angles: vec![0., 2. / 3. * PI, 4. / 3. * PI, 6. / 3. * PI],
                distances: vec![50., 50., 50., 50.],
                border_thickness: 0.,
                colour: Vec4::one(),
            }),
            RenderLiteral::Game(ShapeLiteral::Polygon {
                // pos: [200., 200.],
                pos: Vec2::new(200., 200.),
                angles: vec![0., 2. / 3. * PI, 4. / 3. * PI, 6. / 3. * PI],
                distances: vec![150., 50., 50., 50.],
                border_thickness: 0.,
                colour: Vec4::one(),
            }),
        ];
        shapes.append(&mut self.player.polygons());
        EverythingToDraw {
            scale: 1. - (0.6 / (1.0_f32 + (10.0_f32 * (0.57_f32 * self.speed).exp()))),
            camera_pos: self.cam_position,
            inverted: false,
            shapes,
        }
    }

    fn update(&mut self, dt: f32) {
        let player_physics = self.player.physics_module.borrow();
        self.cam_position = player_physics.position;
        self.speed = player_physics.velocity.mag();
        drop(player_physics);

        self.player.update(dt);

        self.physics.update(dt);
    }

    fn input(&mut self, input: Input) {
        self.player.input(input);
    }
}
