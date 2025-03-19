use engine::text::TextBox;
use engine::text::DEFAULT_FONT;
use engine::RenderLiteral;
use std::f32::consts::PI;
use ultraviolet::{Vec2, Vec4};
#[derive(Clone)]
pub struct Button<'a> {
    pub placement: Vec2,
    pub color: Vec4,
    pub size: Vec<f32>,
    pub value: String,
    pub text: &'a str,
}

impl<'a> Button<'a> {
    pub fn new(placement: Vec2, value: String, color: Vec4, size: Vec<f32>, text: &'a str) -> Self {
        Button {
            value,
            placement,
            color,
            size,
            text,
        }
    }

    pub fn to_render(&self) -> Vec<RenderLiteral> {
        let mut vec = vec![RenderLiteral::UI {
            anchor: Vec2 { x: 0., y: 0. },
            shape: (engine::ShapeLiteral::Polygon {
                pos: self.placement,
                angles: vec![
                    (15. / 360.) * 2. * PI,
                    (165. / 360.) * 2. * PI,
                    (195. / 360.) * 2. * PI,
                    (345.0) / 360. * 2. * PI,
                ],
                distances: self.size.clone(),
                border_thickness: 0.,
                colour: self.color,
            }),
        }];
        vec.append(
            &mut TextBox {
                pos: self.placement - Vec2 { x: 50., y: 50. },
                font_size: 10.,
                string: self.text,
                space_width: 0.5,
                ui_anchor: Some(Vec2 { x: 0., y: 0. }),
                char_set: &DEFAULT_FONT,
                line_gap: 1.,
                width: 80.,
                colour: Vec4::one(),
            }
            .laid_out(),
        );
        vec
    }
    pub fn get_value(&self) -> String {
        self.value.clone()
    }
}
