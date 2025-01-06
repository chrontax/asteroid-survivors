#version 450

layout(location = 0) in vec2 offset;
layout(location = 1) in vec2 anchor;
// layout(location = 1) in uint in_polygon_start;
// layout(location = 2) in uint in_vertex_count;

layout(push_constant) uniform consts {
    // vec2 anchor;
    uint height;
    uint width;
} CONSTS;

layout(location = 0) out vec2 frag_pos;
// layout(location = 1) out uint polygon_start;
// layout(location = 2) out uint vertex_count;

void main() {
    gl_Position = vec4(
            anchor.x + offset.x / CONSTS.width,
            anchor.y + offset.y / CONSTS.height,
            0,
            1
        );

    // polygon_start = in_polygon_start;
    // vertex_count = in_vertex_count;
    frag_pos = gl_Position.xy;
}
