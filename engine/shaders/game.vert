#version 450

layout(location = 0) in vec2 in_pos;
// layout(location = 1) in uint in_polygon_start;
// layout(location = 2) in uint in_vertex_count;
// layout(location = 3) in vec2 in_polygon_center;

layout(push_constant) uniform consts {
    vec2 cam_pos;
    float scale;
    uint width;
    uint height;
} CONSTS;

// layout(location = 0) out vec2 frag_pos;
// layout(location = 1) out uint polygon_start;
// layout(location = 2) out uint vertex_count;
// layout(location = 3) out vec2 polygon_center;

vec2 transform(vec2 position) {
    vec2 result = position - CONSTS.cam_pos;
    result.x *= CONSTS.scale / CONSTS.width;
    result.y *= CONSTS.scale / CONSTS.height;

    return result;
}

void main() {
    // polygon_start = in_polygon_start;
    // vertex_count = in_vertex_count;
    // polygon_center = transform(in_polygon_center);
    gl_Position = vec4(transform(in_pos), 0, 1);
}
