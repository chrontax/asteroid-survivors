use crate::button::Button;
use crate::menu::{self, Menu};
use crate::player::Upgrades;
use engine::{physics::PhysicsModule, Input, RenderLiteral};
use maplit::hashmap;
use rand::seq::SliceRandom; // 0.7.2
use rand::thread_rng;
use rand::Rng;
use std::collections::HashMap;

use std::sync::LazyLock;
use ultraviolet::{Vec2, Vec3, Vec4};

#[derive(Clone)]
pub struct UpgradeManager<'a> {
    menu: Menu<'a>,
}
impl<'a> UpgradeManager<'a> {
    pub fn new() -> Self {
        // todo: zrobic zeby upgrady sie nie powtarzaly
        let up1: usize = rand::thread_rng().gen_range(0..upgradesList.len());
        let up2: usize = rand::thread_rng().gen_range(0..upgradesList.len());
        let up3: usize = rand::thread_rng().gen_range(0..upgradesList.len());

        Self {
            menu: Menu::new(
                vec![
                    Button::new(
                        Vec2 { x: -750., y: 0. },
                        up1.to_string(),
                        upgradesList[up1].color,
                        vec![300., 300., 300., 300.],
                        upgradesList[up1].description,
                    ),
                    Button::new(
                        Vec2 { x: 0., y: 0. },
                        up2.to_string(),
                        upgradesList[up2].color,
                        vec![300., 300., 300., 300.],
                        upgradesList[up2].description,
                    ),
                    Button::new(
                        Vec2 { x: 750., y: 0. },
                        up3.to_string(),
                        upgradesList[up3].color,
                        vec![300., 300., 300., 300.],
                        upgradesList[up3].description,
                    ),
                ],
                Vec2 { x: 0., y: 0. },
            ),
        }
    }
    pub fn to_render(self) -> Vec<RenderLiteral> {
        self.menu.to_render()
    }
    pub fn input(&mut self, input: Input) {
        self.menu.input(input);
    }
    pub fn get_out(&self) -> Option<&str> {
        self.menu.get_out()
    }
}

pub static upgradesList: &[Upgrade] = &[
    Upgrade {
        upgrade: [
            (UpgradeType::dmg_add, 5.0),
            (UpgradeType::thrust_add, 20.0),
            (UpgradeType::rotation_add, 0.5),
        ],
        description: "Increase damage, thrust, and rotation speed.",
        color: Vec4::new(1.0, 0.0, 0.0, 1.0), // Red
    },
    Upgrade {
        upgrade: [
            (UpgradeType::dmg_mult, 1.2),
            (UpgradeType::speed_limit, -0.5),
            (UpgradeType::bullet_per_attack, 1.0),
        ],
        description: "Boost damage, reduce speed limit, and add one bullet per attack.",

        color: Vec4::new(0.0, 1.0, 0.0, 1.0), // Green
    },
    Upgrade {
        upgrade: [
            (UpgradeType::thrust_mult, 1.5),
            (UpgradeType::pierce, 1.0),
            (UpgradeType::accurancy, 0.1),
        ],
        description: "Increase thrust speed, add piercing effect, and improve accuracy.",

        color: Vec4::new(0.0, 0.0, 1.0, 1.0), // Blue
    },
    Upgrade {
        upgrade: [
            (UpgradeType::dmg_add, 10.0),
            (UpgradeType::rotation_mult, 1.3),
            (UpgradeType::bounce, 1.0),
        ],
        description:
            "Significantly increase damage and rotation, and add bounce effect to bullets.",

        color: Vec4::new(1.0, 1.0, 0.0, 1.0), // Yellow
    },
    Upgrade {
        upgrade: [
            (UpgradeType::dmg_mult, 1.5),
            (UpgradeType::pierce, 1.0),
            (UpgradeType::thrust_add, 5.0),
        ],
        description: "Massively increase damage and thrust, with added piercing effect.",
        color: Vec4::new(1.0, 0.5, 0.0, 1.0), // Orange
    },
];

pub struct Upgrade {
    pub upgrade: [(UpgradeType, f32); 3],
    description: &'static str,
    color: Vec4,
}

#[derive(Clone, Copy)]
pub enum UpgradeType {
    dmg_add,
    dmg_mult,
    thrust_add,
    thrust_mult,
    rotation_add,
    rotation_mult,
    bounce,
    pierce,
    accurancy,
    bullet_per_attack,
    speed_limit,
}

// dict: HashMap<String, Vec4> = hashmap! {
//     "white".to_string() => Vec4 { x: 1., y: 1., z: 1., w: 1. },
// };
