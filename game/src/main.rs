use asteroid::Asteroid;
use engine::{
    physics::PhysicsEngine, run_game, EngineInitInfo, EverythingToDraw, Game as GameTrait, Input,
    RenderLiteral, ShapeLiteral,
};
use player::Player;
use rand::seq::SliceRandom;
use rand::Rng;
use std::{f32::consts::PI, ptr::null};
use ultraviolet::{Vec2, Vec4};
use upgradeManager::{Upgrade, UpgradeManager};
use whoami;
use winit::dpi::PhysicalSize;

mod asteroid;
mod bullet;
mod button;
mod menu;
mod player;
mod upgradeManager;
use button::Button;
use menu::Menu;

const MAX_ZOOM_OUT: f32 = 0.5;

fn main() {
    run_game::<Game>().unwrap();
}

struct Game<'a> {
    cam_position: Vec2,
    physics: PhysicsEngine,
    player: Player,
    asteroid_vec: Vec<Asteroid>,
    speed: f32,
    game_state: GameState,
    menu: Option<Menu<'a>>,
    upgrade_manager: Option<UpgradeManager<'a>>,
}

impl<'a> GameTrait for Game<'a> {
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
                game_state: GameState::MainMenu,
                menu: Some(Menu::new_main()),
                upgrade_manager: Option::None,
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
                let mut shapes = vec![];
                shapes.append(&mut self.menu.as_ref().unwrap().to_render());
                return EverythingToDraw {
                    scale: 1. - (MAX_ZOOM_OUT / (1. + (4. + -0.008 * self.speed).exp())),
                    camera_pos: self.cam_position,
                    inverted: false,
                    shapes,
                };
            }
            GameState::Paused => {
                let mut shapes = vec![];
                shapes.append(&mut self.menu.as_ref().unwrap().to_render());
                return EverythingToDraw {
                    scale: 1. - (MAX_ZOOM_OUT / (1. + (4. + -0.008 * self.speed).exp())),
                    camera_pos: self.cam_position,
                    inverted: false,
                    shapes,
                };
            }
            GameState::Upgradeing => {
                let mut shapes = vec![];
                shapes.append(
                    &mut <UpgradeManager<'_> as Clone>::clone(
                        &self.upgrade_manager.as_ref().unwrap(),
                    )
                    .to_render(),
                );
                return EverythingToDraw {
                    scale: 1. - (MAX_ZOOM_OUT / (1. + (4. + -0.008 * self.speed).exp())),
                    camera_pos: self.cam_position,
                    inverted: false,
                    shapes,
                };
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

                self.player.update(dt, &mut self.physics);

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
            match (key.to_text(), self.game_state, state) {
                (Some("\u{1b}"), GameState::Paused, winit::event::ElementState::Released) => {
                    self.game_state = GameState::Running
                }
                (Some("\u{1b}"), GameState::Running, winit::event::ElementState::Released) => {
                    self.menu = Some(Menu::new_pause());
                    self.game_state = GameState::Paused
                }
                (Some("u"), GameState::Running, winit::event::ElementState::Released) => {
                    self.upgrade_manager = Some(UpgradeManager::new());
                    self.game_state = GameState::Upgradeing
                }

                (Some("u"), GameState::Upgradeing, winit::event::ElementState::Released) => {
                    self.game_state = GameState::Running
                }
                _ => (),
            }
        }
        match (self.game_state) {
            (GameState::Running) => {
                self.player.input(input);
            }
            (GameState::MainMenu) => {
                match (self.menu.as_ref().unwrap().get_out()) {
                    None => (),
                    Some("exit") => panic!(),
                    Some("start") => {
                        self.asteroid_vec = vec![];
                        self.cam_position = Vec2::new(0., 0.);
                        self.player = Player::new(self.physics.new_module());
                        self.speed = 0.;
                        self.game_state = GameState::Running;
                    }
                    _ => (),
                }
                self.menu.as_mut().unwrap().input(input);
            }
            (GameState::Paused) => {
                match (self.menu.as_ref().unwrap().get_out()) {
                    None => (),
                    Some("unpause") => self.game_state = GameState::Running,
                    Some("menu") => {
                        self.game_state = GameState::MainMenu;
                        self.menu = Some(Menu::new_main());
                    }
                    Some("desktop") => panic!(),
                    _ => (),
                }
                self.menu.as_mut().unwrap().input(input)
            }
            (GameState::Upgradeing) => {
                match self.upgrade_manager.as_ref().unwrap().get_out() {
                    Some(a) => {
                        self.player.upgrade(a);
                        self.game_state = GameState::Running
                    }
                    _ => (),
                }
                self.upgrade_manager.as_mut().unwrap().input(input)
            }
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
