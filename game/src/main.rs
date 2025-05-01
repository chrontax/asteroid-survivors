use asteroid::Asteroid;
use engine::text::TextBox;
use engine::text::DEFAULT_FONT;
use engine::{
    physics::PhysicsEngine, run_game, EngineInitInfo, EverythingToDraw, Game as GameTrait, Input,
};

use player::Player;
use rand::seq::SliceRandom;
use rand::Rng;
use ultraviolet::{Vec2, Vec4};
use upgradeManager::UpgradeManager;
use utils::HitType;
use utils::{get_orb, get_ui_orb};
use winit::dpi::PhysicalSize;

mod asteroid;
mod bullet;
mod button;
mod menu;
mod player;
mod upgradeManager;
use menu::Menu;
mod utils;

const MAX_ZOOM_OUT: f32 = 0.1;

fn main() {
    run_game::<Game>().unwrap();
}

struct Game<'a> {
    cam_position: Vec2,
    physics: PhysicsEngine<HitType>,
    player: Player,
    asteroid_vec: Vec<Asteroid>,
    speed: f32,
    game_state: GameState,
    menu: Option<Menu<'a>>,
    upgrade_manager: Option<UpgradeManager<'a>>,
    time_elapsed: f64,
}

impl GameTrait for Game<'_> {
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
                player: Player::new(&mut physics),
                physics,
                asteroid_vec: Default::default(),
                speed: Default::default(),
                game_state: GameState::MainMenu,
                menu: Some(Menu::new_main()),
                upgrade_manager: Option::None,
                time_elapsed: 0.,
            },
        )
    }

    fn draw(&self) -> EverythingToDraw {
        match self.game_state {
            GameState::Running => {
                let mut shapes = vec![];
                shapes.append(&mut self.player.polygons());
                shapes.extend(self.asteroid_vec.iter().flat_map(|a| a.polygon()));

                EverythingToDraw {
                    scale: 0.7 - (MAX_ZOOM_OUT / (1. + (4. + -0.008 * self.speed).exp())),
                    camera_pos: self.cam_position,
                    inverted: false,
                    shapes,
                }
            }
            GameState::MainMenu => {
                let mut shapes = vec![];
                shapes.append(&mut self.menu.as_ref().unwrap().to_render());
                EverythingToDraw {
                    scale: 1. - (MAX_ZOOM_OUT / (1. + (4. + -0.008 * self.speed).exp())),
                    camera_pos: self.cam_position,
                    inverted: false,
                    shapes,
                }
            }
            GameState::Paused => {
                let mut shapes = vec![];
                shapes.append(&mut self.menu.as_ref().unwrap().to_render());
                EverythingToDraw {
                    scale: 1. - (MAX_ZOOM_OUT / (1. + (4. + -0.008 * self.speed).exp())),
                    camera_pos: self.cam_position,
                    inverted: false,
                    shapes,
                }
            }
            GameState::Upgradeing => {
                let mut shapes = vec![];
                shapes.append(
                    &mut <UpgradeManager<'_> as Clone>::clone(
                        self.upgrade_manager.as_ref().unwrap(),
                    )
                    .to_render(),
                );
                EverythingToDraw {
                    scale: 1.,
                    camera_pos: self.cam_position,
                    inverted: false,
                    shapes,
                }
            }
            GameState::Frenia => {
                let mut shapes = vec![];
                shapes.push(get_ui_orb(
                    Vec2 { x: -0.5, y: -0.5 },
                    self.player.health / self.player.max_health,
                    30.,
                ));

                shapes.append(
                    &mut TextBox {
                        pos: Vec2 { x: 0., y: 0. },
                        font_size: 10.,
                        string: &format!(
                            "Good day to hunt, Captain!\nhull integrity at:  {}% \nshields at:  {}%\nwe can currently afford {} upgrades",
                            (self.player.health / self.player.max_health) * 100.,
                            (self.player.shield / self.player.max_shield) * 100.,
                            self.upgrade_manager.as_ref().unwrap().count_possible_upgrades()
                        ),

                        space_width: 2.,
                        ui_anchor: Some(Vec2 { x: -0.3, y: -0.5 }),
                        char_set: &DEFAULT_FONT,
                        line_gap: 5.,
                        width: 10000.,
                        colour: Vec4::one(),
                    }
                    .laid_out(),
                );

                EverythingToDraw {
                    scale: 1.,
                    camera_pos: self.cam_position,
                    inverted: false,
                    shapes,
                }
            }
            GameState::Loss => {
                let mut shapes = vec![];
                shapes.push(get_ui_orb(
                    Vec2 { x: -0.5, y: -0.5 },
                    self.player.health / self.player.max_health,
                    30.,
                ));

                shapes.append(
                    &mut TextBox {
                        pos: Vec2 { x: 0., y: 0. },
                        font_size: 10.,
                        string:
                            &format!("You have lost the game \n press space to return to the main menu \n your score is \n {:#?} ", self.upgrade_manager.as_ref().unwrap().resources.values().cloned().collect::<Vec<i32>>().iter().map(|a| a as &i32).sum::<i32>() as f64 * self.time_elapsed ),
                        space_width: 2.,
                        ui_anchor: Some(Vec2 { x: -0.3, y: -0.5 }),
                        char_set: &DEFAULT_FONT,
                        line_gap: 5.,
                        width: 10000.,
                        colour: Vec4::one(),
                    }
                    .laid_out(),
                );

                EverythingToDraw {
                    scale: 1.,
                    camera_pos: self.cam_position,
                    inverted: false,
                    shapes,
                }
            }
        }
    }

    fn update(&mut self, dt: f32) {
        if self.game_state == GameState::Running {
            self.time_elapsed += dt as f64;
            let player_physics = self.player.physics_module.borrow();
            self.cam_position = player_physics.position;
            self.speed = player_physics.velocity.mag();
            drop(player_physics);

            self.player.update(dt, &mut self.physics);

            self.physics.update(dt);
            if rand::thread_rng().gen::<f64>()
                < 1. / 1000. * ((self.time_elapsed / 1000.).floor() + 1.)
            {
                let x = [1500.0f32, -1500.0f32];
                self.asteroid_vec.push(Asteroid::new(
                    &mut self.physics,
                    self.cam_position
                        + Vec2::new(
                            *x.choose(&mut rand::thread_rng()).unwrap(),
                            *x.choose(&mut rand::thread_rng()).unwrap(),
                        ),
                ));
            }
            for asteroid in self.asteroid_vec.iter_mut() {
                asteroid.update(dt);
                if asteroid.to_delete && asteroid.timer > 0.0 {
                    self.upgrade_manager
                        .as_mut()
                        .unwrap()
                        .add_resource(asteroid.resorces.0, asteroid.resorces.1)
                }
            }

            self.asteroid_vec.retain(|a| !a.to_delete);
            if self.player.health <= 0. {
                self.game_state = GameState::Loss;
            }
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
                    self.upgrade_manager.as_mut().unwrap().make_menu();
                    self.game_state = GameState::Upgradeing
                }

                (Some("u"), GameState::Upgradeing, winit::event::ElementState::Released) => {
                    self.game_state = GameState::Running
                }
                (Some("f"), GameState::Running, winit::event::ElementState::Released) => {
                    self.game_state = GameState::Frenia
                }

                (Some("f"), GameState::Frenia, winit::event::ElementState::Released) => {
                    self.game_state = GameState::Running
                }
                (Some(" "), GameState::Loss, winit::event::ElementState::Released) => {
                    self.game_state = GameState::MainMenu;
                }

                _ => (),
            }
        }
        match self.game_state {
            GameState::Running => {
                self.player.input(input);
            }
            GameState::MainMenu => {
                match self.menu.as_ref().unwrap().get_out() {
                    None => (),
                    Some("exit") => panic!(),
                    Some("start") => {
                        self.upgrade_manager = Some(UpgradeManager::new());
                        self.asteroid_vec = vec![];
                        self.cam_position = Vec2::new(0., 0.);
                        self.player = Player::new(&mut self.physics);
                        self.speed = 0.;
                        self.game_state = GameState::Running;
                    }
                    _ => (),
                }
                self.menu.as_mut().unwrap().input(input);
            }
            GameState::Paused => {
                match self.menu.as_ref().unwrap().get_out() {
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
            GameState::Upgradeing => {
                if let Some(a) = self.upgrade_manager.as_mut().unwrap().get_out() {
                    self.player.upgrade(a);
                    self.game_state = GameState::Running
                }
                self.upgrade_manager.as_mut().unwrap().input(input)
            }
            GameState::Loss => (),
            _ => (),
        }
    }
}
#[derive(Copy, Clone, PartialEq)]
enum GameState {
    MainMenu,
    Paused,
    Running,
    Upgradeing,
    Frenia,
    Loss,
}
