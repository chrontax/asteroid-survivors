#version 450

layout(location = 0) in vec2 offset;
layout(location = 1) in vec2 anchor;
layout(location = 2) in vec4 in_col;
layout(location = 3) in float point_size;

layout(location = 0) out vec4 colour;

layout(push_constant) uniform consts {
    uint height;
    uint width;
} CONSTS;

void main() {
    gl_Position = vec4(
            anchor.x + offset.x / CONSTS.width,
            anchor.y + offset.y / CONSTS.height,
            0,
            1
        );
    gl_PointSize = point_size / 2 * CONSTS.width;

    colour = in_col;
}
