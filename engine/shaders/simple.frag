#version 450

#ifdef GAME
#define OFFSET 32
#elif defined(UI)
#define OFFSET 16
#endif

layout(location = 0) flat in vec4 colour;

layout(location = 0) out vec4 out_col;

layout(push_constant) uniform consts {
    layout(offset = OFFSET) bool inverted;
} CONSTS;

void main() {
    out_col = CONSTS.inverted ? vec4(1 - colour.rgb, colour.a) : colour;
}
