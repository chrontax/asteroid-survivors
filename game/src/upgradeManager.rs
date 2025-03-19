use crate::button::Button;
use crate::menu::Menu;
use engine::{Input, RenderLiteral};
// 0.7.2
use rand::{
    distributions::{Distribution, Standard},
    Rng,
};

use ultraviolet::{Vec2, Vec4};

#[derive(Clone)]
pub struct UpgradeManager<'a> {
    menu: Menu<'a>,
}
impl UpgradeManager<'_> {
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
            (UpgradeType::dmg_mult, 0.2),
            (UpgradeType::accurancy, 0.01),
            (UpgradeType::bullet_per_attack, 1.0),
        ],
        description: "Boost damage, decrease accuracy, and add one bullet per attack.",
        color: Vec4::new(0.0, 1.0, 0.0, 1.0), // Green
    },
    Upgrade {
        upgrade: [
            (UpgradeType::thrust_mult, 0.5),
            (UpgradeType::pierce, 1.0),
            (UpgradeType::accurancy, 0.1),
        ],
        description: "Multiply thrust speed, add piercing effect, and improve accuracy.",
        color: Vec4::new(0.0, 0.0, 1.0, 1.0), // Blue
    },
    Upgrade {
        upgrade: [
            (UpgradeType::dmg_add, 10.0),
            (UpgradeType::rotation_mult, 0.3),
            (UpgradeType::bounce, 1.0),
        ],
        description:
            "Significantly increase damage and multiply rotation, and add bounce effect to bullets.",
        color: Vec4::new(1.0, 1.0, 0.0, 1.0), // Yellow
    },
    Upgrade {
        upgrade: [
            (UpgradeType::dmg_mult, 0.5),
            (UpgradeType::pierce, 1.0),
            (UpgradeType::thrust_add, 5.0),
        ],
        description: "Multiply damage and increase thrust, with added piercing effect.",
        color: Vec4::new(1.0, 0.5, 0.0, 1.0), // Orange
    },
    Upgrade {
        upgrade: [
            (UpgradeType::thrust_add, 5.0),
            (UpgradeType::dmg_mult, 0.3),
            (UpgradeType::bounce, 1.0),
        ],
        description: "Increase thrust, boost damage, and add bounce effect.",
        color: Vec4::new(0.5, 0.0, 0.5, 1.0), // Purple
    },
    Upgrade {
        upgrade: [
            (UpgradeType::rotation_mult, 0.4),
            (UpgradeType::bullet_per_attack, 2.0),
            (UpgradeType::accurancy, 0.05),
        ],
        description: "Increase rotation speed and fire extra bullets, slightly improving accuracy.",
        color: Vec4::new(0.0, 1.0, 1.0, 1.0), // Cyan
    },
    Upgrade {
        upgrade: [
            (UpgradeType::thrust_add, 10.0),
            (UpgradeType::thrust_mult, 0.2),
            (UpgradeType::thrust_add, 10.0),
        ],
        description: "Boost thrust significantly.",
        color: Vec4::new(1.0, 0.75, 0.8, 1.0), // Pink
    },
    Upgrade {
        upgrade: [
            (UpgradeType::dmg_add, 15.0),
            (UpgradeType::pierce, 2.0),
            (UpgradeType::accurancy, 0.2),
        ],
        description: "Massively increase damage, improve piercing effect, and enhance accuracy.",
        color: Vec4::new(0.8, 0.3, 0.0, 1.0), // Brown
    },
    Upgrade {
        upgrade: [
            (UpgradeType::thrust_mult, 0.4),
            (UpgradeType::rotation_add, 1.0),
            (UpgradeType::bullet_per_attack, 1.0),
        ],
        description: "Multiply thrust, improve rotation, and fire an extra bullet per attack.",
        color: Vec4::new(0.2, 0.8, 0.2, 1.0), // Light Green
    },
    Upgrade {
        upgrade: [
            (UpgradeType::dmg_mult, 0.3),
            (UpgradeType::bounce, 2.0),
            (UpgradeType::accurancy, 0.15),
        ],
        description:
            "Increase damage multiplier, add double bounce effect, and slightly enhance accuracy.",
        color: Vec4::new(0.7, 0.7, 0.2, 1.0), // Olive
    },
    Upgrade {
        upgrade: [
            (UpgradeType::dmg_add, 8.0),
            (UpgradeType::thrust_mult, 0.3),
            (UpgradeType::pierce, 1.0),
        ],
        description: "Increase base damage, multiply thrust, and add piercing.",
        color: Vec4::new(0.3, 0.2, 0.8, 1.0), // Indigo
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
    empty,
}

// dict: HashMap<String, Vec4> = hashmap! {
//     "white".to_string() => Vec4 { x: 1., y: 1., z: 1., w: 1. },
// };

#[derive(Debug)]
pub enum ResourceType {
    Luna,     // range sight
    Gaia,     // a bit of everything
    Mars,     // DAMAGE
    Mercury,  // speed
    Venus,    // pierce or bounce or heal
    Jupiter,  // weight + resource gain
    Neptune,  // attack speed
    Uranus,   // rotation
    Asteroid, // forges of the asteroid belt shield
}

impl Distribution<ResourceType> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> ResourceType {
        match rng.gen_range(0..=7) {
            0 => ResourceType::Luna,
            1 => ResourceType::Gaia,
            2 => ResourceType::Mars,
            3 => ResourceType::Mercury,
            4 => ResourceType::Venus,
            5 => ResourceType::Jupiter,
            6 => ResourceType::Neptune,
            _ => ResourceType::Uranus,
        }
    }
}
