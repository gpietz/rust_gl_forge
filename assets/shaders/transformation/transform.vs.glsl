#version 330 core
layout (location = 0) in vec3 aPos;
layout (location = 1) in vec4 aColor;   // Updated to include color
layout (location = 2) in vec2 aTexCoord;

out vec4 VertexColor;
out vec2 TexCoord;

uniform mat4 transform;

void main() {
	gl_Position = transform * vec4(aPos, 1.0f);
	VertexColor = aColor;    // Pass color to the fragment shader
	TexCoord    = aTexCoord;
}
