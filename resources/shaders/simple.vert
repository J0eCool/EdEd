#version 330 core

layout (location = 0) in vec3 Position;
layout (location = 1) in vec2 Color;

out vec3 vColor;

void main() {
    gl_Position = vec4(Position, 1.0);
    vColor = vec3(Color, 1.0);
}
