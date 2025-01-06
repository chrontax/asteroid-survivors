use inline_spirv::include_spirv as i_spirv;

macro_rules! include_spirv {
    ($path:expr, $stage:ident$(, $a:ident)*) => {
        i_spirv!($path, $stage, glsl, entry = "main", $(D $a),*)
    };
}

pub const UI_VERTEX_SHADER: &[u32] = include_spirv!("shaders/ui.vert", vert);
pub const GAME_VERTEX_SHADER: &[u32] = include_spirv!("shaders/game.vert", vert);

pub const POINT_FRAGMENT_SHADER: &[u32] = include_spirv!("shaders/point.frag", frag);
pub const POLYGON_FRAGMENT_SHADER: &[u32] = include_spirv!("shaders/polygon.frag", frag);
pub const SIMPLE_GAME_FRAGMENT_SHADER: &[u32] = include_spirv!("shaders/simple.frag", frag, GAME);
pub const SIMPLE_UI_FRAGMENT_SHADER: &[u32] = include_spirv!("shaders/simple.frag", frag, UI);
