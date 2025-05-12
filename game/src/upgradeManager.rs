use crate::button::Button;
use crate::menu::Menu;
use crate::utils::get_color_from_resource_type;
use engine::{Input, RenderLiteral};
use std::collections::HashMap;

// 0.7.2
use rand::{
    distributions::{Distribution, Standard},
    seq::SliceRandom,
    Rng,
};

use ultraviolet::{Vec2, Vec4};

#[derive(Clone)]
pub struct UpgradeManager<'a> {
    menu: Option<Menu<'a>>,
    pub resources: HashMap<ResourceType, i32>,
}
impl UpgradeManager<'_> {
    pub fn new() -> Self {
        // todo: zrobic zeby upgrady sie nie powtarzaly
        let resources: HashMap<ResourceType, i32> = vec![
            (ResourceType::Luna, 0),
            (ResourceType::Gaia, 0),
            (ResourceType::Mars, 0),
            (ResourceType::Mercury, 0),
            (ResourceType::Venus, 0),
            (ResourceType::Jupiter, 0),
            (ResourceType::Neptune, 0),
            (ResourceType::Uranus, 0),
            (ResourceType::Asteroid, 0),
            (ResourceType::General, 0),
        ]
        .into_iter()
        .collect();
        Self {
            menu: None,
            resources,
        }
    }
    pub fn to_render(self) -> Vec<RenderLiteral> {
        self.menu.unwrap().to_render()
    }
    pub fn input(&mut self, input: Input) {
        self.menu.as_mut().unwrap().input(input);
    }
    pub fn get_out(&mut self) -> Option<&str> {
        let a = self.menu.as_mut().and_then(|menu| menu.get_out());
        if a != None {
            *self
                .resources
                .get_mut(&UPGRADES[a.unwrap().parse::<usize>().unwrap()].resource_type)
                .unwrap() -= UPGRADES[a.unwrap().parse::<usize>().unwrap()].min_resource;
        }

        // dbg!(self.resources[&UPGRADES[a.unwrap().parse::<usize>().unwrap()].resource_type]);

        a
    }
    pub fn count_possible_upgrades(&self) -> i32 {
        let mut count = 0;
        let mut i: usize = 1;
        for upgrade in &UPGRADES[1..] {
            if upgrade.min_resource <= self.resources[&upgrade.resource_type] {
                count += 1
            }
            i += 1;
        }
        count
    }

    pub fn make_menu(&mut self) {
        let mut possibe_upgrades: Vec<(&Upgrade, usize)> = vec![];
        let mut i: usize = 1;
        for upgrade in &UPGRADES[1..] {
            if upgrade.min_resource <= self.resources[&upgrade.resource_type] {
                possibe_upgrades.push((upgrade, i));
            }
            i += 1;
        }
        if possibe_upgrades.len() == 0 {
            possibe_upgrades.push((&UPGRADES[0], 0 as usize));
        }

        let mut rngesus = rand::thread_rng();
        use rand::seq::SliceRandom;

        // Get up to 3 unique upgrades (or fewer if not enough exist)
        let upgrades: Vec<_> = if possibe_upgrades.len() >= 3 {
            // Choose 3 distinct upgrades randomly
            possibe_upgrades.choose_multiple(&mut rngesus, 3).collect()
        } else {
            // If fewer than 3 exist, just take what's available (or return None)
            possibe_upgrades
                .choose_multiple(&mut rngesus, possibe_upgrades.len())
                .collect()
        };

        let up1 = upgrades.get(0).copied();
        let up2 = upgrades.get(1).copied();
        let up3 = upgrades.get(3).copied();

        let default_upgrade = (UPGRADES.get(0).unwrap(), 0 as usize);
        let (up1, up2, up3) = match upgrades.len() {
            3 => (upgrades[0], upgrades[1], upgrades[2]),
            2 => (upgrades[0], upgrades[1], &default_upgrade),
            1 => (upgrades[0], &default_upgrade, &default_upgrade),
            _ => (&default_upgrade, &default_upgrade, &default_upgrade),
        };
        self.menu = Some(Menu::new(
            vec![
                Button::new(
                    Vec2 { x: -750., y: 0. },
                    up1.1.to_string(),
                    get_color_from_resource_type(up1.0.resource_type),
                    vec![300., 300., 300., 300.],
                    up1.0.description,
                ),
                Button::new(
                    Vec2 { x: 0., y: 0. },
                    up2.1.to_string(),
                    get_color_from_resource_type(up2.0.resource_type),
                    vec![300., 300., 300., 300.],
                    up2.0.description,
                ),
                Button::new(
                    Vec2 { x: 750., y: 0. },
                    up3.1.to_string(),
                    get_color_from_resource_type(up3.0.resource_type),
                    vec![300., 300., 300., 300.],
                    up3.0.description,
                ),
            ],
            Vec2 { x: 0., y: 0. },
        ))
    }

    pub fn add_resource(&mut self, res: ResourceType, amount: i32) {
        *self.resources.entry(res).or_insert(0) += amount;
    }
}

pub static UPGRADES: [Upgrade; 21] = [
    Upgrade {
        upgrade: [
            (UpgradeType::Empty, 1.),
            (UpgradeType::Empty, 1.),
            (UpgradeType::Empty, 0.),
        ],
        description: "no possible upgrades",
        resource_type: ResourceType::General,
        min_resource: 0,
    },
    // Upgrade {
    //     upgrade: [
    //         (UpgradeType::DmgAdd, 1.),
    //         (UpgradeType::ThrustAdd, 1.),
    //         (UpgradeType::Empty, 0.),
    //     ],
    //     description: "basic damage and thrust upgrade",
    //     resource_type: ResourceType::General,
    //     min_resource: 0,
    // },
    // Upgrade {
    //     upgrade: [
    //         (UpgradeType::DmgMult, 0.01),
    //         (UpgradeType::ThrustAdd, 1.),
    //         (UpgradeType::Empty, 0.),
    //     ],
    //     description: "basic mult damage and thrust upgrade",
    //     resource_type: ResourceType::General,
    //     min_resource: 0,
    // },
    // Upgrade {
    //     upgrade: [
    //         (UpgradeType::RotationAdd, 0.01),
    //         (UpgradeType::ThrustAdd, 1.),
    //         (UpgradeType::Empty, 0.),
    //     ],
    //     description: "basic rotation and thrust upgrade",
    //     resource_type: ResourceType::General,
    //     min_resource: 0,
    // },
    Upgrade {
        upgrade: [
            (UpgradeType::DmgAdd, 10.0),
            (UpgradeType::ThrustAdd, 5.0),
            (UpgradeType::Empty, 0.0),
        ],
        description: "Increases damage and thrust slightly.",
        resource_type: ResourceType::Mars,
        min_resource: 100,
    },
    Upgrade {
        upgrade: [
            (UpgradeType::DmgMult, 1.2),
            (UpgradeType::Pierce, 1.0),
            (UpgradeType::Empty, 0.0),
        ],
        description: "Boosts damage and adds piercing effect.",
        resource_type: ResourceType::Luna,
        min_resource: 150,
    },
    Upgrade {
        upgrade: [
            (UpgradeType::ThrustMult, 1.5),
            (UpgradeType::RotationAdd, 10.0),
            (UpgradeType::Empty, 0.0),
        ],
        description: "Enhances thrust and improves rotation.",
        resource_type: ResourceType::Mercury,
        min_resource: 120,
    },
    Upgrade {
        upgrade: [
            (UpgradeType::Bounce, 2.0),
            (UpgradeType::Accurancy, 0.8),
            (UpgradeType::Empty, 0.0),
        ],
        description: "Adds bounce effect and increases accuracy.",
        resource_type: ResourceType::Luna,
        min_resource: 200,
    },
    Upgrade {
        upgrade: [
            (UpgradeType::BulletPerAttack, 2.0),
            (UpgradeType::DmgMult, 1.1),
            (UpgradeType::Accurancy, 1.0),
        ],
        description: "Fires additional bullets per attack with a slight damage boost.",
        resource_type: ResourceType::Mars,
        min_resource: 180,
    },
    Upgrade {
        upgrade: [
            (UpgradeType::RotationMult, 1.3),
            (UpgradeType::ThrustAdd, 3.0),
            (UpgradeType::Empty, 0.0),
        ],
        description: "Improves rotation speed and adds thrust.",
        resource_type: ResourceType::Uranus,
        min_resource: 130,
    },
    Upgrade {
        upgrade: [
            (UpgradeType::DmgAdd, 10.0),
            (UpgradeType::Pierce, 1.5),
            (UpgradeType::Empty, 0.0),
        ],
        description: "Significantly increases damage and enhances piercing.",
        resource_type: ResourceType::Mars,
        min_resource: 250,
    },
    Upgrade {
        upgrade: [
            (UpgradeType::ThrustMult, 1.8),
            (UpgradeType::BulletPerAttack, 1.0),
            (UpgradeType::Empty, 0.0),
        ],
        description: "Greatly enhances thrust and adds an extra bullet per attack.",
        resource_type: ResourceType::Mercury,
        min_resource: 220,
    },
    Upgrade {
        upgrade: [
            (UpgradeType::Bounce, 3.0),
            (UpgradeType::Accurancy, 0.9),
            (UpgradeType::Empty, 0.0),
        ],
        description: "Maximizes bounce effect and improves accuracy.",
        resource_type: ResourceType::Luna,
        min_resource: 300,
    },
    Upgrade {
        upgrade: [
            (UpgradeType::DmgMult, 1.5),
            (UpgradeType::ThrustMult, 1.2),
            (UpgradeType::Empty, 0.0),
        ],
        description: "Boosts both damage and thrust multipliers.",
        resource_type: ResourceType::Gaia,
        min_resource: 400,
    },
    Upgrade {
        upgrade: [
            (UpgradeType::RotationAdd, 15.0),
            (UpgradeType::ThrustAdd, 5.0),
            (UpgradeType::Empty, 0.0),
        ],
        description: "Improves rotation and thrust significantly.",
        resource_type: ResourceType::Uranus,
        min_resource: 350,
    },
    Upgrade {
        upgrade: [
            (UpgradeType::Pierce, 2.0),
            (UpgradeType::DmgAdd, 8.0),
            (UpgradeType::Empty, 0.0),
        ],
        description: "Enhances piercing and adds substantial damage.",
        resource_type: ResourceType::Luna,
        min_resource: 280,
    },
    Upgrade {
        upgrade: [
            (UpgradeType::BulletPerAttack, 3.0),
            (UpgradeType::DmgMult, 1.3),
            (UpgradeType::Accurancy, 10.5),
        ],
        description: "Fires three bullets per attack with a strong damage boost.",
        resource_type: ResourceType::Mars,
        min_resource: 500,
    },
    Upgrade {
        upgrade: [
            (UpgradeType::ThrustAdd, 10.0),
            (UpgradeType::RotationMult, 1.4),
            (UpgradeType::Empty, 0.0),
        ],
        description: "Massively increases thrust and rotation speed.",
        resource_type: ResourceType::Mercury,
        min_resource: 450,
    },
    Upgrade {
        upgrade: [
            (UpgradeType::Bounce, 4.0),
            (UpgradeType::Accurancy, 1.0),
            (UpgradeType::Empty, 0.0),
        ],
        description: "Maximizes bounce and accuracy for unparalleled performance.",
        resource_type: ResourceType::Luna,
        min_resource: 600,
    },
    Upgrade {
        upgrade: [
            (UpgradeType::DmgMult, 2.0),
            (UpgradeType::ThrustMult, 1.5),
            (UpgradeType::Empty, 0.0),
        ],
        description: "Doubles damage and significantly boosts thrust.",
        resource_type: ResourceType::Gaia,
        min_resource: 700,
    },
    Upgrade {
        upgrade: [
            (UpgradeType::RotationAdd, 20.0),
            (UpgradeType::ThrustAdd, 8.0),
            (UpgradeType::Empty, 0.0),
        ],
        description: "Maximizes rotation and thrust for ultimate control.",
        resource_type: ResourceType::Uranus,
        min_resource: 650,
    },
    Upgrade {
        upgrade: [
            (UpgradeType::Pierce, 3.0),
            (UpgradeType::DmgAdd, 12.0),
            (UpgradeType::Empty, 0.0),
        ],
        description: "Ultimate piercing and massive damage increase.",
        resource_type: ResourceType::Luna,
        min_resource: 800,
    },
    Upgrade {
        upgrade: [
            (UpgradeType::BulletPerAttack, 4.0),
            (UpgradeType::DmgMult, 1.6),
            (UpgradeType::Accurancy, 100.),
        ],
        description: "Fires four bullets per attack with a massive damage boost.",
        resource_type: ResourceType::Mars,
        min_resource: 900,
    },
    Upgrade {
        upgrade: [
            (UpgradeType::ThrustAdd, 500.0),
            (UpgradeType::RotationMult, 5.),
            (UpgradeType::Empty, 0.0),
        ],
        description: "Unmatched thrust and rotation speed for ultimate performance.",
        resource_type: ResourceType::Mercury,
        min_resource: 850,
    },
];

#[derive(Debug, PartialEq)]
pub struct Upgrade {
    pub upgrade: [(UpgradeType, f32); 3],
    description: &'static str,
    resource_type: ResourceType,
    min_resource: i32,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UpgradeType {
    DmgAdd,
    DmgMult,
    ThrustAdd,
    ThrustMult,
    RotationAdd,
    RotationMult,
    Bounce,
    Pierce,
    Accurancy,
    BulletPerAttack,
    Empty,
}

// dict: HashMap<String, Vec4> = hashmap! {
//     "white".to_string() => Vec4 { x: 1., y: 1., z: 1., w: 1. },
// };

#[derive(PartialEq, Eq, Hash, Debug, Clone, Copy)]
pub enum ResourceType {
    Luna,     // pierce or bounce
    Gaia,     // a bit of everything
    Mars,     // DAMAGE
    Mercury,  // speed
    Venus,    // heal
    Jupiter,  // weight + resource gain
    Neptune,  // attack speed
    Uranus,   // rotation
    Asteroid, // forges of the asteroid belt shield shield
    General,
}

impl Distribution<ResourceType> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> ResourceType {
        match rng.gen_range(0..=8) {
            0 => ResourceType::Luna,
            1 => ResourceType::Gaia,
            2 => ResourceType::Mars,
            3 => ResourceType::Mercury,
            4 => ResourceType::Venus,
            5 => ResourceType::Jupiter,
            6 => ResourceType::Neptune,
            7 => ResourceType::Uranus,
            _ => ResourceType::Asteroid,
        }
    }
}
