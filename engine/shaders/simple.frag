#version 450

#ifdef GAME
#define OFFSET 32
#endif

#ifdef UI
#define OFFSET 16
#endif

layout(push_constant) uniform consts {
    layout(offset = OFFSET) vec4 colour;
} CONSTS;

layout(location = 0) out vec4 colour;

void main() {
    colour = CONSTS.colour;
}
