#version 330 core
out vec4 FragColor;

in vec4 VertexColor;
in vec2 TexCoord;

uniform sampler2D texture1;
uniform bool useColor; // Uniform to toggle color on/off

void main() {
    if (useColor) {
        FragColor = texture(texture1, TexCoord) * VertexColor; // Apply color
    } else {
        FragColor = texture(texture1, TexCoord); // Ignore color
    }
}
