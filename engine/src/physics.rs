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
    ) -> Rc<RefCell<PhysicsModule<T>>> {
        let module = Rc::new(RefCell::new(PhysicsModule {
            position: Vec2::zero(),
            velocity: Vec2::zero(),
            mass: 1.,
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
                // println!("Colliding");
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
                println!("Collision");
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
                collider_ref.current_collider = Some(module_rc.clone());
                module.current_collider = Some(collider.clone());
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
    } = shape;
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
    let p2_cartesian = polygon_cartesian_coords(p2, r2, pos2);
    let ShapeLiteral::Polygon {
        pos: mut p1_pos,
        angles: p1_angles,
        distances: p1_distances,
        ..
    } = p1;
    p1_pos += pos1;
    p2_cartesian
        .iter()
        .map(|&point| {
            let point = point - p1_pos;
            let angle = point.y.atan2(point.x);
            Vec2::new(
                point.mag(),
                if angle < 0. { angle + 2. * PI } else { angle },
            )
        })
        .any(|point| {
            let mut left = if p1_angles[0] + r1 > point.y {
                (
                    p1_distances[p1_distances.len() - 1],
                    p1_angles[p1_angles.len() - 1] - 2. * PI,
                )
            } else {
                p1_distances
                    .iter()
                    .copied()
                    .zip(p1_angles.iter().copied())
                    .rev()
                    .find(|&(_, a)| a + r1 < point.y)
                    .unwrap()
            };
            left.1 += r1;
            let mut right = p1_distances
                .iter()
                .copied()
                .zip(p1_angles.iter().copied())
                // .skip(1)
                .find(|&(_, angle)| angle + r1 > point.y)
                .unwrap_or((p1_distances[0], p1_angles[0] + 2. * PI));
            right.1 += r1;

            let lerped_distance = (Rotor2::from_angle(right.1) * Vec2::unit_x() * right.0)
                .lerp(
                    Rotor2::from_angle(left.1) * Vec2::unit_x() * left.0,
                    (point.y - left.1) / (right.1 - left.1),
                )
                .mag();
            println!(
                "{:.3}\t{:.3}\t{:.3}\t{:.3}\t\t{:.3}\t{:.3}",
                r1, left.1, right.1, point.y, lerped_distance, point.x
            );

            lerped_distance >= point.x
        })
}
