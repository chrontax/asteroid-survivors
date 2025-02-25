use rand::seq::SliceRandom;
use rand::Rng;
use std::f32::consts::PI;
use whoami;

use asteroid::Asteroid;
use engine::{
    physics::PhysicsEngine, run_game, EngineInitInfo, EverythingToDraw, Game as GameTrait, Input,
    RenderLiteral, ShapeLiteral,
};
use player::Player;

use ultraviolet::{Vec2, Vec4};
use winit::dpi::PhysicalSize;

mod asteroid;
mod player;

const MAX_ZOOM_OUT: f32 = 0.5;

fn main() {
    run_game::<Game>().unwrap();
}

struct Game {
    cam_position: Vec2,
    physics: PhysicsEngine,
    player: Player,
    asteroid_vec: Vec<Asteroid>,
    speed: f32,
    game_state: GameState,
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
                asteroid_vec: Default::default(),
                speed: Default::default(),
                game_state: GameState::Paused,
            },
        )
    }

    fn draw(&self) -> EverythingToDraw {
        match self.game_state {
            GameState::Running => {
                let mut shapes = vec![];
                shapes.append(&mut self.player.polygons());
                shapes.extend(self.asteroid_vec.iter().flat_map(|a| a.polygon()));

                return EverythingToDraw {
                    scale: 1. - (MAX_ZOOM_OUT / (1. + (4. + -0.008 * self.speed).exp())),
                    camera_pos: self.cam_position,
                    inverted: false,
                    shapes,
                };
            }
            GameState::MainMenu => {
                let shapes = vec![
                    RenderLiteral::UI {
                        anchor: Vec2 { x: 0., y: 0. },
                        shape: (engine::ShapeLiteral::Polygon {
                            pos: Vec2 { x: 0., y: 0. },
                            angles: vec![
                                (15. / 360.) * 2. * PI,
                                (165. / 360.) * 2. * PI,
                                (195. / 360.) * 2. * PI,
                                (345.0) / 360. * 2. * PI,
                            ],
                            distances: vec![300., 300., 300., 300.],
                            border_thickness: 0.,
                            colour: Vec4::new(0., 1., 0., 1.),
                        }),
                    },
                    RenderLiteral::UI {
                        anchor: Vec2 { x: 0., y: 0. },
                        shape: (engine::ShapeLiteral::Polygon {
                            pos: Vec2 { x: 0., y: 200. },
                            angles: vec![
                                (15. / 360.) * 2. * PI,
                                (165. / 360.) * 2. * PI,
                                (195. / 360.) * 2. * PI,
                                (345.0) / 360. * 2. * PI,
                            ],
                            distances: vec![300., 300., 300., 300.],
                            border_thickness: 0.,
                            colour: Vec4::new(1., 0., 0., 1.),
                        }),
                    },
                    RenderLiteral::UI {
                        anchor: Vec2 { x: 0., y: 0. },
                        shape: (engine::ShapeLiteral::Polygon {
                            pos: Vec2 { x: -400., y: 200. },
                            angles: vec![0., 2. / 3. * PI, 4. / 3. * PI],
                            distances: vec![50., 50., 50.],
                            border_thickness: 0.,
                            colour: Vec4::new(1., 1., 1., 1.),
                        }),
                    },
                ];
                return EverythingToDraw {
                    scale: 1. - (MAX_ZOOM_OUT / (1. + (4. + -0.008 * self.speed).exp())),
                    camera_pos: self.cam_position,
                    inverted: false,
                    shapes,
                };
            }
            GameState::Paused => {
                let shapes = vec![RenderLiteral::UI {
                    anchor: Vec2 { x: 0., y: 0. },
                    shape: (engine::ShapeLiteral::Polygon {
                        pos: Vec2 { x: 0., y: 0. },
                        angles: vec![
                            (15. / 360.) * 2. * PI,
                            (165. / 360.) * 2. * PI,
                            (195. / 360.) * 2. * PI,
                            (345.) / 360. * 2. * PI,
                        ],
                        distances: vec![300., 300., 300., 300.],
                        border_thickness: 0.,
                        colour: Vec4::new(0., 1., 0., 1.),
                    }),
                }];
                return EverythingToDraw {
                    scale: 1.,
                    camera_pos: self.cam_position,
                    inverted: false,
                    shapes,
                };
            }
            GameState::Upgradeing => {
                todo!("upgardy")
            }
        }
    }

    fn update(&mut self, dt: f32) {
        match self.game_state {
            GameState::Running => {
                let player_physics = self.player.physics_module.borrow();
                self.cam_position = player_physics.position;
                self.speed = player_physics.velocity.mag();
                drop(player_physics);

                self.player.update(dt);

                self.physics.update(dt);

                if rand::thread_rng().gen::<f32>() < 1. / 200. {
                    let x = vec![1500.0f32, -1500.0f32];
                    self.asteroid_vec.push(Asteroid::new(
                        self.physics.new_module(),
                        self.cam_position
                            + Vec2::new(
                                *x.choose(&mut rand::thread_rng()).unwrap(),
                                *x.choose(&mut rand::thread_rng()).unwrap(),
                            ),
                    ));
                }
                for asteroid in self.asteroid_vec.iter_mut() {
                    asteroid.update(dt);
                    if asteroid.to_delete && asteroid.timer > 0. {
                        // tutaj dodanie materialow do upgrade managera kiedy bedzie
                    }
                }
                self.asteroid_vec.retain(|a| !a.to_delete);
            }
            _ => (),
        }
    }

    fn input(&mut self, input: Input) {
        if let Input::Keyboard { key, state } = input.clone() {
            dbg!(key.to_text());
            dbg!(state);
            match (key.to_text(), self.game_state, state) {
                (Some("\u{1b}"), GameState::Paused, winit::event::ElementState::Released) => {
                    self.game_state = GameState::Running
                }
                (Some("\u{1b}"), GameState::Running, winit::event::ElementState::Released) => {
                    self.game_state = GameState::Paused
                }
                _ => (),
            }
        }

        if self.game_state == GameState::Running {
            self.player.input(input);
        } else {
            // todo!("uzywanie menu")
        }
    }
}
#[derive(Copy, Clone, PartialEq)]
enum GameState {
    MainMenu,
    Paused,
    Running,
    Upgradeing,
}
