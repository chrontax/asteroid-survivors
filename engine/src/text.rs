use phf::phf_map;
use ultraviolet::{Vec2, Vec4};

use crate::{RenderLiteral, ShapeLiteral};

pub type Glyph = [[bool; 5]; 5];

#[derive(Clone, Copy, Debug)]
pub struct Character {
    pub glyph: Glyph,
    pub width: f32,
}

pub trait CharSet {
    fn get_char(&self, c: char) -> Option<Character>;
}

impl CharSet for phf::Map<char, Character> {
    fn get_char(&self, c: char) -> Option<Character> {
        self.get(&c).copied()
    }
}

pub struct TextBox<'a> {
    pub char_set: &'static dyn CharSet,
    pub string: &'a str,
    pub pos: Vec2,
    pub font_size: f32,
    pub space_width: f32,
    pub line_gap: f32,
    pub width: f32,
    pub colour: Vec4,
    pub ui_anchor: Option<Vec2>,
}

impl TextBox<'_> {
    pub fn laid_out(&self) -> Vec<RenderLiteral> {
        let mut render_literals = vec![];
        let mut cur_line_width = 0.;
        let mut cur_line = 0;
        for line in self.string.lines() {
            for word in line.split_whitespace() {
                let word_len = word
                    .chars()
                    .map(|c| {
                        self.char_set
                            .get_char(c)
                            .unwrap_or_else(|| panic!("char `{}` not in char_set", c))
                            .width
                    })
                    .sum::<f32>();
                if cur_line_width + word_len > self.width / self.font_size && cur_line_width != 0. {
                    cur_line += 1;
                    cur_line_width = 0.;
                }
                for c in word.chars() {
                    let c = self.char_set.get_char(c).unwrap();
                    let shape = ShapeLiteral::Glyph {
                        pos: self.pos
                            + Vec2::new(cur_line_width, cur_line as f32 * (5. + self.line_gap))
                                * self.font_size,
                        colour: self.colour,
                        glyph: c.glyph,
                        size: self.font_size,
                    };
                    render_literals.push(if let Some(anchor) = self.ui_anchor {
                        RenderLiteral::UI { anchor, shape }
                    } else {
                        RenderLiteral::Game(shape)
                    });
                    cur_line_width += c.width;
                }
                cur_line_width += self.space_width;
            }
            cur_line_width = 0.;
        }

        render_literals
    }
}

pub static DEFAULT_FONT: phf::Map<char, Character> = phf_map! {
    'A' => Character {
        glyph: [
            [false, true, true, false, false],
            [true, false, false, true, false],
            [true, true, true, true, false],
            [true, false, false, true, false],
            [true, false, false, true, false]
        ],
        width: 4.5,
    },
    'a' => Character {
        glyph: [
            [false, false, false, false, false],
            [false, false, false, false, false],
            [false, true, true, false, false],
            [true, false, true, false, false],
            [false, true, true, false, false]
        ],
        width: 3.5,
    },
    'B' => Character {
        glyph: [
            [true, true, true, false, false],
            [true, false, false, true, false],
            [true, true, true, true, false],
            [true, false, false, true, false],
            [true, true, true, false, false]
        ],
        width: 4.5,
    },
    'b' => Character {
        glyph: [
            [true, false, false, false, false],
            [true, false, false, false, false],
            [true, true, true, false, false],
            [true, false, true, false, false],
            [true, true, true, false, false]
        ],
        width: 3.5,
    },
    'C' => Character {
        glyph: [
            [false, true, true, false, false],
            [true, false, false, true, false],
            [true, false, false, false, false],
            [true, false, false, true, false],
            [false, true, true, false, false]
        ],
        width: 4.5,
    },
    'c' => Character {
        glyph: [
            [false, false, false, false, false],
            [false, true, true, false, false],
            [true, false, false, false, false],
            [true, false, false, false, false],
            [false, true, true, false, false]
        ],
        width: 3.5,
    },
    'D' => Character {
        glyph: [
            [true, true, true, false, false],
            [true, false, false, true, false],
            [true, false, false, true, false],
            [true, false, false, true, false],
            [true, true, true, false, false]
        ],
        width: 4.5,
    },
    'd' => Character {
        glyph: [
            [false, false, true, false, false],
            [false, false, true, false, false],
            [false, true, true, false, false],
            [true, false, true, false, false],
            [false, true, true, false, false]
        ],
        width: 3.5,
    },
    'E' => Character {
        glyph: [
            [true, true, true, true, false],
            [true, false, false, false, false],
            [true, true, true, false, false],
            [true, false, false, false, false],
            [true, true, true, true, false]
        ],
        width: 4.5,
    },
    'e' => Character {
        glyph: [
            [false, false, false, false, false],
            [false, true, false, false, false],
            [true, true, true, false, false],
            [true, false, false, false, false],
            [false, true, true, false, false]
        ],
        width: 3.5,
    },
    'F' => Character {
        glyph: [
            [true, true, true, true, false],
            [true, false, false, false, false],
            [true, true, true, false, false],
            [true, false, false, false, false],
            [true, false, false, false, false]
        ],
        width: 4.5,
    },
    'f' => Character {
        glyph: [
            [false, false, true, false, false],
            [false, true, false, false, false],
            [true, true, true, false, false],
            [false, true, false, false, false],
            [false, true, false, false, false]
        ],
        width: 3.5,
    },
    'G' => Character {
        glyph: [
            [false, true, true, false, false],
            [true, false, false, true, false],
            [true, false, false, false, false],
            [true, false, true, true, false],
            [false, true, true, false, false]
        ],
        width: 4.5,
    },
    'g' => Character {
        glyph: [
            [false, true, false, false, false],
            [true, false, true, false, false],
            [false, true, false, false, false],
            [false, false, true, false, false],
            [false, true, false, false, false]
        ],
        width: 3.5,
    },
    'H' => Character {
        glyph: [
            [true, false, false, true, false],
            [true, false, false, true, false],
            [true, true, true, true, false],
            [true, false, false, true, false],
            [true, false, false, true, false]
        ],
        width: 4.5,
    },
    'h' => Character {
        glyph: [
            [true, false, false, false, false],
            [true, false, false, false, false],
            [true, true, true, false, false],
            [true, false, true, false, false],
            [true, false, true, false, false]
        ],
        width: 3.5,
    },
    'I' => Character {
        glyph: [
            [true, true, true, false, false],
            [false, true, false, false, false],
            [false, true, false, false, false],
            [false, true, false, false, false],
            [true, true, true, false, false]
        ],
        width: 3.5,
    },
    'i' => Character {
        glyph: [
            [true, false, false, false, false],
            [false, false, false, false, false],
            [true, false, false, false, false],
            [true, false, false, false, false],
            [true, false, false, false, false]
        ],
        width: 1.5,
    },
    'J' => Character {
        glyph: [
            [false, false, true, false, false],
            [false, false, true, false, false],
            [false, false, true, false, false],
            [true, false, true, false, false],
            [false, true, true, false, false]
        ],
        width: 3.5,
    },
    'j' => Character {
        glyph: [
            [false, true, false, false, false],
            [false, false, false, false, false],
            [false, true, false, false, false],
            [false, true, false, false, false],
            [true, false, false, false, false]
        ],
        width: 2.5,
    },
    'K' => Character {
        glyph: [
            [true, false, false, true, false],
            [true, false, true, false, false],
            [true, true, false, false, false],
            [true, false, true, false, false],
            [true, false, false, true, false]
        ],
        width: 4.5,
    },
    'k' => Character {
        glyph: [
            [true, false, false, false, false],
            [true, false, false, false, false],
            [true, false, true, false, false],
            [true, true, false, false, false],
            [true, false, true, false, false]
        ],
        width: 3.5,
    },
    'L' => Character {
        glyph: [
            [true, false, false, false, false],
            [true, false, false, false, false],
            [true, false, false, false, false],
            [true, false, false, false, false],
            [true, true, true, true, false]
        ],
        width: 4.5,
    },
    'l' => Character {
        glyph: [
            [true, false, false, false, false],
            [true, false, false, false, false],
            [true, false, false, false, false],
            [true, false, false, false, false],
            [false, true, false, false, false]
        ],
        width: 2.5,
    },
    'M' => Character {
        glyph: [
            [true, false, false, false, true],
            [true, true, false, true, true],
            [true, false, true, false, true],
            [true, false, false, false, true],
            [true, false, false, false, true]
        ],
        width: 5.5,
    },
    'm' => Character {
        glyph: [
            [false, false, false, false, false],
            [false, false, false, false, false],
            [true, true, false, true, false],
            [true, false, true, false, true],
            [true, false, true, false, true]
        ],
        width: 5.5,
    },
    'N' => Character {
        glyph: [
            [true, false, false, false, true],
            [true, true, false, false, true],
            [true, false, true, false, true],
            [true, false, false, true, true],
            [true, false, false, false, true]
        ],
        width: 5.5,
    },
    'n' => Character {
        glyph: [
            [false, false, false, false, false],
            [false, false, false, false, false],
            [true, true, false, false, false],
            [true, false, true, false, false],
            [true, false, true, false, false]
        ],
        width: 3.5,
    },
    'O' => Character {
        glyph: [
            [false, true, true, false, false],
            [true, false, false, true, false],
            [true, false, false, true, false],
            [true, false, false, true, false],
            [false, true, true, false, false]
        ],
        width: 4.5,
    },
    'o' => Character {
        glyph: [
            [false, false, false, false, false],
            [false, false, false, false, false],
            [false, true, false, false, false],
            [true, false, true, false, false],
            [false, true, false, false, false]
        ],
        width: 3.5,
    },
    'P' => Character {
        glyph: [
            [true, true, true, false, false],
            [true, false, false, true, false],
            [true, true, true, false, false],
            [true, false, false, false, false],
            [true, false, false, false, false]
        ],
        width: 4.5,
    },
    'p' => Character {
        glyph: [
            [false, false, false, false, false],
            [true, true, false, false, false],
            [true, false, true, false, false],
            [true, true, false, false, false],
            [true, false, false, false, false]
        ],
        width: 3.5,
    },
    'Q' => Character {
        glyph: [
            [false, true, true, false, false],
            [true, false, false, true, false],
            [true, false, false, true, false],
            [true, false, true, false, false],
            [false, true, true, true, false]
        ],
        width: 4.5,
    },
    'q' => Character {
        glyph: [
            [false, false, false, false, false],
            [false, true, true, false, false],
            [true, false, true, false, false],
            [false, true, true, false, false],
            [false, false, true, false, false]
        ],
        width: 3.5,
    },
    'R' => Character {
        glyph: [
            [true, true, true, false, false],
            [true, false, false, true, false],
            [true, true, true, false, false],
            [true, false, true, false, false],
            [true, false, false, true, false]
        ],
        width: 4.5,
    },
    'r' => Character {
        glyph: [
            [false, false, false, false, false],
            [false, false, false, false, false],
            [false, true, false, false, false],
            [true, false, true, false, false],
            [true, false, false, false, false]
        ],
        width: 3.5,
    },
    'S' => Character {
        glyph: [
            [false, true, true, true, false],
            [true, false, false, false, false],
            [false, true, true, false, false],
            [false, false, false, true, false],
            [true, true, true, false, false]
        ],
        width: 4.5,
    },
    's' => Character {
        glyph: [
            [false, false, false, false, false],
            [false, true, false, false, false],
            [true, false, false, false, false],
            [false, true, false, false, false],
            [true, false, false, false, false]
        ],
        width: 2.5,
    },
    'T' => Character {
        glyph: [
            [true, true, true, false, false],
            [false, true, false, false, false],
            [false, true, false, false, false],
            [false, true, false, false, false],
            [false, true, false, false, false]
        ],
        width: 3.5,
    },
    't' => Character {
        glyph: [
            [false, true, false, false, false],
            [true, true, true, false, false],
            [false, true, false, false, false],
            [false, true, false, false, false],
            [false, false, true, false, false]
        ],
        width: 3.5,
    },
    'U' => Character {
        glyph: [
            [true, false, false, true, false],
            [true, false, false, true, false],
            [true, false, false, true, false],
            [true, false, false, true, false],
            [false, true, true, false, false]
        ],
        width: 4.5,
    },
    'u' => Character {
        glyph: [
            [false, false, false, false, false],
            [false, false, false, false, false],
            [true, false, false, true, false],
            [true, false, false, true, false],
            [false, true, true, false, false]
        ],
        width: 4.5,
    },
    'V' => Character {
        glyph: [
            [true, false, true, false, false],
            [true, false, true, false, false],
            [true, false, true, false, false],
            [true, false, true, false, false],
            [false, true, false, false, false]
        ],
        width: 3.5,
    },
    'v' => Character {
        glyph: [
            [false, false, false, false, false],
            [false, false, false, false, false],
            [true, false, true, false, false],
            [true, false, true, false, false],
            [false, true, false, false, false]
        ],
        width: 3.5,
    },
    'W' => Character {
        glyph: [
            [true, false, false, false, true],
            [true, false, false, false, true],
            [true, false, false, false, true],
            [true, false, true, false, true],
            [true, true, false, true, true],
        ],
        width: 5.5,
    },
    'w' => Character {
        glyph: [
            [false, false, false, false, false],
            [false, false, false, false, false],
            [true, false, false, false, true],
            [true, false, true, false, true],
            [false, true, false, true, false]
        ],
        width: 5.5,
    },
    'X' => Character {
        glyph: [
            [true, false, false, true, false],
            [true, false, false, true, false],
            [false, true, true, false, false],
            [true, false, false, true, false],
            [true, false, false, true, false]
        ],
        width: 4.5,
    },
    'x' => Character {
        glyph: [
            [false, false, false, false, false],
            [false, false, false, false, false],
            [true, false, true, false, false],
            [false, true, false, false, false],
            [true, false, true, false, false]
        ],
        width: 3.5,
    },
    'Y' => Character {
        glyph: [
            [true, false, true, false, false],
            [true, false, true, false, false],
            [false, true, false, false, false],
            [false, true, false, false, false],
            [false, true, false, false, false]
        ],
        width: 3.5,
    },
    'y' => Character {
        glyph: [
            [false, false, false, false, false],
            [false, false, false, false, false],
            [true, false, true, false, false],
            [false, true, false, false, false],
            [true, false, false, false, false]
        ],
        width: 3.5,
    },
    'Z' => Character {
        glyph: [
            [true, true, true, false, false],
            [false, false, true, false, false],
            [false, true, false, false, false],
            [true, false, false, false, false],
            [true, true, true, false, false]
        ],
        width: 3.5,
    },
    'z' => Character {
        glyph: [
            [false, false, false, false, false],
            [true, false, false, false, false],
            [false, true, false, false, false],
            [true, false, false, false, false],
            [false, true, false, false, false]
        ],
        width: 2.5,
    },
    '0' => Character {
        glyph: [
            [false, true, true, false, false],
            [true, false, true, true, false],
            [true, false, false, true, false],
            [true, true, false, true, false],
            [false, true, true, false, false]
        ],
        width: 4.5,
    },
    '1' => Character {
        glyph: [
            [false, true, false, false, false],
            [true, true, false, false, false],
            [false, true, false, false, false],
            [false, true, false, false, false],
            [true, true, true, false, false]
        ],
        width: 3.5,
    },
    '2' => Character {
        glyph: [
            [false, true, true, false, false],
            [true, false, false, true, false],
            [false, false, true, false, false],
            [false, true, false, false, false],
            [true, true, true, true, false]
        ],
        width: 4.5,
    },
    '3' => Character {
        glyph: [
            [true, true, false, false, false],
            [false, false, true, false, false],
            [true, true, false, false, false],
            [false, false, true, false, false],
            [true, true, false, false, false]
        ],
        width: 3.5,
    },
    '4' => Character {
        glyph: [
            [true, false, true, false, false],
            [true, false, true, false, false],
            [true, true, true, false, false],
            [false, false, true, false, false],
            [false, false, true, false, false]
        ],
        width: 3.5,
    },
    '5' => Character {
        glyph: [
            [true, true, true, false, false],
            [true, false, false, false, false],
            [true, true, true, false, false],
            [false, false, true, false, false],
            [true, true, false, false, false]
        ],
        width: 3.5,
    },
    '6' => Character {
        glyph: [
            [false, true, true, false, false],
            [true, false, false, false, false],
            [true, true, true, false, false],
            [true, false, true, false, false],
            [false, true, true, false, false]
        ],
        width: 3.5,
    },
    '7' => Character {
        glyph: [
            [true, true, true, false, false],
            [false, false, true, false, false],
            [false, true, false, false, false],
            [false, true, false, false, false],
            [false, true, false, false, false]
        ],
        width: 3.5,
    },
    '8' => Character {
        glyph: [
            [false, true, false, false, false],
            [true, false, true, false, false],
            [false, true, false, false, false],
            [true, false, true, false, false],
            [false, true, false, false, false]
        ],
        width: 3.5,
    },
    '9' => Character {
        glyph: [
            [true, true, false, false, false],
            [true, false, true, false, false],
            [true, true, true, false, false],
            [false, false, true, false, false],
            [true, true, false, false, false]
        ],
        width: 3.5,
    },
    '.' => Character {
        glyph: [
            [false, false, false, false, false],
            [false, false, false, false, false],
            [false, false, false, false, false],
            [false, false, false, false, false],
            [true, false, false, false, false]
        ],
        width: 1.5,
    },
    ',' => Character {
        glyph: [
            [false, false, false, false, false],
            [false, false, false, false, false],
            [false, false, false, false, false],
            [false, true, false, false, false],
            [true, false, false, false, false]
        ],
        width: 2.5,
    },
    '!' => Character {
        glyph: [
            [true, false, false, false, false],
            [true, false, false, false, false],
            [true, false, false, false, false],
            [false, false, false, false, false],
            [true, false, false, false, false]
        ],
        width: 1.5,
    },
    '?' => Character {
        glyph: [
            [true, true, false, false, false],
            [false, false, true, false, false],
            [false, true, false, false, false],
            [false, false, false, false, false],
            [false, true, false, false, false]
        ],
        width: 3.5,
    },
};
