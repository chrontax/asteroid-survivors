use crate::upgradeManager::ResourceType;
use engine::{physics::PhysicsEngine, physics::PhysicsModule, Input, RenderLiteral, ShapeLiteral};
use rand::Rng;
use std::{cell::RefCell, f32::consts::PI, rc::Rc};
use ultraviolet::{Vec2, Vec4};

pub fn get_orb(pos: Vec2, hp_precent: f32, size: f32) -> RenderLiteral {
    let mut rng = rand::thread_rng();
    return RenderLiteral::Game(ShapeLiteral::Polygon {
        pos: pos,
        angles: (0..20).map(|_| rng.gen_range(0f32..2f32) * PI).collect(),
        distances: (0..20).map(|_| rng.gen_range(0f32..size) * PI).collect(),
        border_thickness: 1.,
        colour: Vec4::new(
            rand::thread_rng().gen_range(0f32..1f32),
            rand::thread_rng().gen_range(0f32..1f32),
            rand::thread_rng().gen_range(0f32..1f32),
            hp_precent,
        ),
    });
}

pub fn get_ui_orb(pos: Vec2, hp_precent: f32, size: f32) -> RenderLiteral {
    let mut rng = rand::thread_rng();
    return RenderLiteral::UI {
        anchor: pos,
        shape: ShapeLiteral::Polygon {
            pos: Vec2 { x: 0., y: 0. },
            angles: (0..40).map(|_| rng.gen_range(0f32..2f32) * PI).collect(),
            distances: (0..40).map(|_| rng.gen_range(0f32..size) * PI).collect(),
            border_thickness: 1.,
            colour: Vec4::new(
                rand::thread_rng().gen_range(0f32..1f32),
                rand::thread_rng().gen_range(0f32..1f32),
                rand::thread_rng().gen_range(0f32..1f32),
                hp_precent,
            ),
        },
    };
}

pub fn get_color_from_resource_type(res: ResourceType) -> Vec4 {
    let out: Vec4 = match res {
        ResourceType::Luna => Vec4::new(1.0, 1.0, 1.0, 1.0),
        ResourceType::Gaia => Vec4::new(0.0, 1.0, 0.0, 1.0), // Example color for Gaia
        ResourceType::Mars => Vec4::new(1.0, 0.0, 0.0, 1.0), // Example color for Mars
        ResourceType::Mercury => Vec4::new(0.5, 0.5, 0.5, 1.0), // Example color for Mercury
        ResourceType::Venus => Vec4::new(1.0, 1.0, 0.0, 1.0), // Example color for Venus
        ResourceType::Jupiter => Vec4::new(1.0, 0.6, 0.0, 1.0), // Example color for Jupiter
        ResourceType::Neptune => Vec4::new(0.0, 0.0, 1.0, 1.0), // Example color for Neptune
        ResourceType::Uranus => Vec4::new(0.0, 1.0, 1.0, 1.0), // Example color for Uranus
        ResourceType::Asteroid => Vec4::new(0.5, 0.5, 0.5, 1.0), // Example color for Asteroid
        ResourceType::General => Vec4::new(0., 0., 0., 1.0),
    };
    out
}
