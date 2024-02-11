#version 330 core
out vec4 FragColor;

in vec4 VertexColor;
in vec2 TexCoord;

uniform sampler2D texture1;
uniform sampler2D texture2;
uniform bool useColor;      // Uniform to toggle color on/off
uniform bool useTexture2;   // Uniform to toggle second texture on/off

void main() {
    vec4 texColor1 = texture(texture1, TexCoord);
    vec4 finalColor;
    
    if (useTexture2) {
        vec4 texColor2 = texture(texture2, TexCoord);
        finalColor = mix(texColor1, texColor2, 0.40);
    } else {
        finalColor = texColor1;
    }
    
    if (useColor) {
        FragColor = finalColor * VertexColor;
    } else {
        FragColor = finalColor;
    }
}
