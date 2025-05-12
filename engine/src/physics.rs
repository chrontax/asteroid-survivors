use geo::{Contains, Point, Polygon};
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use std::{cell::RefCell, f32::consts::PI, rc::Rc};
use ultraviolet::{Lerp, Rotor2, Vec2};

use crate::ShapeLiteral;

type CollisionCallback<T> = Box<dyn Fn(&mut T, &T) -> CollisionResponse>;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CollisionResponse {
    Collide,
    Pass,
}

pub struct PhysicsModule<T: Clone> {
    pub position: Vec2,
    pub velocity: Vec2,
    pub mass: f32,
    pub force: Vec2,
    pub rotation: f32,
    pub angular_velocity: f32,
    pub hitbox: ShapeLiteral,
    pub inner: T,
    on_collision: CollisionCallback<T>,
    current_collider: Option<Rc<RefCell<PhysicsModule<T>>>>,
}

#[derive(Default)]
pub struct PhysicsEngine<T: Clone> {
    modules: Vec<Rc<RefCell<PhysicsModule<T>>>>,
}

impl<T: Default + Clone> PhysicsEngine<T> {
    pub fn new_module(
        &mut self,
        hitbox: ShapeLiteral,
        on_collision: impl Fn(&mut T, &T) -> CollisionResponse + 'static,
        inner: T,
        mass: f32,
    ) -> Rc<RefCell<PhysicsModule<T>>> {
        let module = Rc::new(RefCell::new(PhysicsModule {
            position: Vec2::zero(),
            velocity: Vec2::zero(),
            mass,
            force: Vec2::zero(),
            rotation: 0.,
            angular_velocity: 0.,
            hitbox,
            inner,
            on_collision: Box::new(on_collision),
            current_collider: None,
        }));
        self.modules.push(module.clone());
        module
    }

    pub fn update(&mut self, dt: f32) {
        self.modules.retain(|m| Rc::strong_count(m) > 1);
        for module_rc in &self.modules {
            let mut module = module_rc.borrow_mut();
            let acceleration = module.force / module.mass;
            module.force = Vec2::zero();
            module.velocity += acceleration * dt;
            module.position = module.position + module.velocity * dt;
            module.rotation += module.angular_velocity * dt;

            if let Some(collider) = &module.current_collider {
                let collider = collider.borrow();
                if !polygons_collide(
                    &module.hitbox,
                    module.rotation,
                    module.position,
                    &collider.hitbox,
                    collider.rotation,
                    collider.position,
                ) {
                    drop(collider);
                    module.current_collider = None;
                }
                continue;
            }

            let collision = self
                .modules
                .iter()
                .filter(|m| !Rc::ptr_eq(m, module_rc))
                .find(|m| {
                    polygons_collide(
                        &module.hitbox,
                        module.rotation,
                        module.position,
                        &m.borrow().hitbox,
                        m.borrow().rotation,
                        m.borrow().position,
                    )
                });
            if let Some(collider) = collision {
                let mut collider_ref = collider.borrow_mut();
                let mut inner = module.inner.clone();
                let res1 = (module.on_collision)(&mut inner, &collider_ref.inner);
                module.inner = inner;
                inner = collider_ref.inner.clone();
                let res2 = (collider_ref.on_collision)(&mut inner, &module.inner);
                collider_ref.inner = inner;
                if res1 != res2 {
                    panic!("Inconsistent collision response");
                }
                if matches!(res1, CollisionResponse::Collide) {
                    let v1 = module.velocity;
                    let v2 = collider_ref.velocity;
                    let m1 = module.mass;
                    let m2 = collider_ref.mass;
                    module.velocity = (m1 - m2) / (m1 + m2) * v1 + 2. * m2 / (m1 + m2) * v2;
                    collider_ref.velocity = (m2 - m1) / (m1 + m2) * v2 + 2. * m1 / (m1 + m2) * v1;
                    collider_ref.current_collider = Some(module_rc.clone());
                    module.current_collider = Some(collider.clone());
                }
            }
        }
    }
}

fn polygon_cartesian_coords(shape: &ShapeLiteral, rotation: f32, position: Vec2) -> Vec<Vec2> {
    let ShapeLiteral::Polygon {
        mut pos,
        angles,
        distances,
        ..
    } = shape
    else {
        panic!("non-polgon as collider");
    };
    pos += position;
    angles
        .iter()
        .zip(distances)
        .map(|(&a, &d)| Rotor2::from_angle(a + rotation) * Vec2::unit_x() * d + pos)
        .collect()
}

fn polygons_collide(
    p1: &ShapeLiteral,
    r1: f32,
    pos1: Vec2,
    p2: &ShapeLiteral,
    r2: f32,
    pos2: Vec2,
) -> bool {
    let ShapeLiteral::Polygon { distances, .. } = p1 else {
        panic!("twoja stara")
    };
    let ShapeLiteral::Polygon {
        distances: distances2,
        ..
    } = p2
    else {
        panic!("twoja stara")
    };

    let max = |a, b| if a > b { a } else { b };

    if (pos1 - pos2).mag()
        > distances.iter().reduce(max).unwrap() + distances2.iter().reduce(max).unwrap()
    {
        return false;
    }

    let p1_cartesian = polygon_cartesian_coords(p1, r1, pos1);
    let p2_cartesian = polygon_cartesian_coords(p2, r2, pos2)
        .iter()
        .map(|pos| Point::new(pos.x, pos.y))
        .collect::<Vec<_>>();

    let p1_geo_poly = Polygon::new(
        p1_cartesian
            .iter()
            .map(|pos| Point::new(pos.x, pos.y))
            .collect(),
        Vec::new(),
    );

    p2_cartesian.par_iter().any(|p| p1_geo_poly.contains(p))
}
