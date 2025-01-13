#version 450

layout(location = 0) in vec2 in_pos;

layout(push_constant) uniform consts {
    vec2 cam_pos;
    float scale;
    uint width;
    uint height;
} CONSTS;

vec2 transform(vec2 position) {
    vec2 result = position - CONSTS.cam_pos;
    result.x *= CONSTS.scale / CONSTS.width;
    result.y *= CONSTS.scale / CONSTS.height;

    return result;
}

void main() {
    gl_Position = vec4(transform(in_pos), 0, 1);
}
