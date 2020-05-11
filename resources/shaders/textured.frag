#version 330 core

in vec2 uvPos;
uniform sampler2D Texture;
out vec4 Color;

void main() {
    float v = texture(Texture, uvPos).x;
    Color = vec4(v, v, v, 1.0);
}
