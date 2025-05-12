use crate::button::Button;
use engine::{Input, RenderLiteral};

use std::f32::consts::PI;
use ultraviolet::{Vec2, Vec4};

#[derive(Clone, Debug)]
pub struct Menu<'a> {
    pub buttons: Vec<Button<'a>>,
    pub selected: i8,
    pub location: Vec2,
    pub out: Option<String>,
}

impl<'a> Menu<'a> {
    pub fn new(buttons: Vec<Button<'a>>, location: Vec2) -> Self {
        Self {
            buttons,
            selected: 0,
            location,
            out: None,
        }
    }
    pub fn new_main() -> Self {
        Menu {
            buttons: vec![
                Button {
                    placement: { Vec2 { x: -750., y: 0. } },
                    value: { "start".to_string() },
                    color: { Vec4::new(1., 1., 1., 1.) },
                    size: { vec![300., 300., 300., 300.] },
                    text: "start",
                },
                Button {
                    placement: { Vec2 { x: 0., y: 0. } },
                    value: { "high scores".to_string() },
                    color: { Vec4::new(1., 1., 1., 1.) },
                    size: { vec![300., 300., 300., 300.] },
                    text: "high scores",
                },
                Button {
                    placement: { Vec2 { x: 750., y: 0. } },
                    value: { "exit".to_string() },
                    color: { Vec4::new(1., 0., 0., 1.) },
                    size: { vec![300., 300., 300., 300.] },
                    text: "exit",
                },
            ],
            selected: 0,
            location: Vec2 { x: 0., y: 100. },
            out: None,
        }
    }
    pub fn new_pause() -> Self {
        Menu {
            buttons: vec![
                Button {
                    placement: { Vec2 { x: -750., y: 0. } },
                    value: { "unpause".to_string() },
                    color: { Vec4::new(1., 1., 1., 1.) },
                    size: { vec![300., 300., 300., 300.] },
                    text: "unpause",
                },
                Button {
                    placement: { Vec2 { x: 0., y: 0. } },
                    value: { "menu".to_string() },
                    color: { Vec4::new(1., 1., 1., 1.) },
                    size: { vec![300., 300., 300., 300.] },
                    text: "quit to menu",
                },
                Button {
                    placement: { Vec2 { x: 750., y: 0. } },
                    value: { "desktop".to_string() },
                    color: { Vec4::new(1., 0., 0., 1.) },
                    size: { vec![300., 300., 300., 300.] },
                    text: "quit to desktop",
                },
            ],
            selected: 0,
            location: Vec2 { x: 0., y: 100. },
            out: None,
        }
    }

    pub fn to_render(&self) -> Vec<RenderLiteral> {
        let mut vec = vec![];
        for button in self.buttons.iter() {
            vec.append(&mut button.to_render())
        }
        vec.push(RenderLiteral::UI {
            anchor: Vec2 { x: 0., y: 0. },
            shape: (engine::ShapeLiteral::Polygon {
                pos: self.location,
                angles: vec![3. / 2. * PI, 13. / 6. * PI, 17. / 6. * PI],
                distances: vec![75., 50., 50.],
                border_thickness: 0.,
                colour: Vec4::one(),
            }),
        });
        vec
    }

    pub fn get_out(&self) -> Option<&str> {
        self.out.as_deref()
    }

    pub fn input(&mut self, input: Input) {
        if let Input::Keyboard { key, state } = input {
            match (key.to_text(), state) {
                (Some("a"), winit::event::ElementState::Released) => {
                    self.selected = (((self.selected - 1) + 3) % 3).abs()
                }
                (Some("d"), winit::event::ElementState::Released) => {
                    self.selected = ((self.selected + 1) % 3).abs()
                }
                (Some(" "), winit::event::ElementState::Released) => {
                    self.out = Some(self.buttons[self.selected as usize].get_value())
                }
                _ => (),
            }
        }

        match self.selected {
            0 => self.location = Vec2 { x: -750., y: 200. },
            1 => self.location = Vec2 { x: 0., y: 200. },
            2 => self.location = Vec2 { x: 750., y: 200. },
            _ => unreachable!(),
        }
    }
}
