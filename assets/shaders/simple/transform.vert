#version 330 core
layout (location = 0) in vec2 aPos;
layout (location = 1) in vec2 aTexCoord;
layout (location = 2) in vec4 aColor;   // Updated to include color

out vec4 VertexColor;
out vec2 TexCoord;

uniform mat4 transform;

void main() {
    gl_Position = transform * vec4(aPos, 0.0, 1.0f);
    VertexColor = aColor;    // Pass color to the fragment shader
    TexCoord = aTexCoord;
}
