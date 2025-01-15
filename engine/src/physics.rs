use std::{cell::RefCell, rc::Rc};
use ultraviolet::Vec2;

#[non_exhaustive]
pub struct PhysicsModule {
    pub position: Vec2,
    pub velocity: Vec2,
    pub mass: f32,
    pub force: Vec2,
}

#[derive(Default)]
pub struct PhysicsEngine {
    modules: Vec<Rc<RefCell<PhysicsModule>>>,
}

impl PhysicsEngine {
    pub fn new_module(&mut self) -> Rc<RefCell<PhysicsModule>> {
        let module = Rc::new(RefCell::new(PhysicsModule {
            position: Vec2::zero(),
            velocity: Vec2::zero(),
            mass: 1.0,
            force: Vec2::zero(),
        }));
        self.modules.push(module.clone());
        module
    }

    pub fn update(&mut self, dt: f32) {
        for mut module in self.modules.iter().map(|m| m.borrow_mut()) {
            let acceleration = module.force / module.mass;
            module.force = Vec2::zero();
            module.velocity += acceleration * dt;
            module.position = module.position + module.velocity * dt;
        }
    }
}
