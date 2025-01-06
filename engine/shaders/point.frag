#version 450

layout(push_constant) uniform consts {
    vec4 colour;
} CONSTS;

layout(location = 0) out vec4 out_colour;

void main() {
    out_colour = CONSTS.colour;
}
