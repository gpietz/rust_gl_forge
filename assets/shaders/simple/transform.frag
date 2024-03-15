#version 330 core
out vec4 FragColor;

in vec4 VertexColor;
in vec2 TexCoord;

uniform sampler2D texture1;
uniform sampler2D texture2;

void main() {
	vec4 texColor1  = texture(texture1, TexCoord);
	vec4 texColor2  = texture(texture2, TexCoord);
	vec4 finalColor = mix(texColor1, texColor2, 0.40);
	FragColor = finalColor * VertexColor;
}
