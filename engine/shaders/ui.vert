#version 450

layout(location = 0) in vec2 offset;
layout(location = 1) in vec2 anchor;

layout(push_constant) uniform consts {
    uint height;
    uint width;
} CONSTS;

layout(location = 0) out vec2 frag_pos;

void main() {
    gl_Position = vec4(
            anchor.x + offset.x / CONSTS.width,
            anchor.y + offset.y / CONSTS.height,
            0,
            1
        );

    frag_pos = gl_Position.xy;
}
