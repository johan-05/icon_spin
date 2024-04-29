#version 450 core
layout (location = 0) in vec3 pos;
uniform vec2 window_dimensions;

out vec2 TexCoord;

void main() {
    gl_Position = vec4(
        pos.x * 0.6 * window_dimensions.y/window_dimensions.x,
        pos.y * 0.6,
        0.0,
        1.0);
    TexCoord = vec2(pos.x+0.5, pos.y+0.5);
}
