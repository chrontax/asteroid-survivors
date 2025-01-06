#version 450

layout(location = 0) in vec2 frag_pos;
layout(location = 1) flat in uint polygon_start;
layout(location = 2) flat in uint vertex_count;
layout(location = 3) flat in vec2 polygon_center;

layout(set = 0, binding = 0) uniform vertices {
    vec2 pos[];
} POLYGON_VERTICES;

layout(push_constant) uniform consts {
    vec4 colour;
} CONSTS;

layout(location = 0) out vec4 colour;

void main() {
    colour = CONSTS.colour;
}
