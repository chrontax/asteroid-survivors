use engine::{physics::PhysicsModule, RenderLiteral};
//
//
//
struct UIValues {
    value: str,
    me: RenderLiteral::UI,
}

impl UIValues {
    fn new(placement: Vec2, value: str, color: Vec4) -> Self {
        UIValues {
            value: value,
            me: RenderLiteral::UI {
                anchor: Vec2 { x: 0., y: 0. },
                shape: (engine::ShapeLiteral::Polygon {
                    pos: placement,
                    angles: vec![
                        (15. / 360.) * 2. * PI,
                        (165. / 360.) * 2. * PI,
                        (195. / 360.) * 2. * PI,
                        (345.0) / 360. * 2. * PI,
                    ],
                    distances: vec![300., 300., 300., 300.],
                    border_thickness: 0.,
                    colour: color,
                }),
            },
        }
    }
}
