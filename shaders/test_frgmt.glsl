#version 450 core
out vec4 frag_color;
in vec2 TexCoord;

uniform sampler2D ourTexture;

void main() {
    vec4 texColor = texture(ourTexture, TexCoord);
    if (texColor.a < 0.1) {
        discard;
    }
    frag_color = texColor;
}