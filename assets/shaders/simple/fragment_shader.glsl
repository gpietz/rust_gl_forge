#version 330 core
out vec4 FragColor;
in vec3 ourColor;
uniform float time;

void main()
{
    if (time > 0) {
        // Base wave effect
        float wave = sin(gl_FragCoord.x * 0.1 + time) * sin(gl_FragCoord.y * 0.1 + time);
        wave = wave * 0.5 + 0.5;
        
        // Additional wave for complexity
        float wave2 = sin(gl_FragCoord.x * 0.05 - time) * cos(gl_FragCoord.y * 0.05 - time);
        wave2 = wave2 * 0.5 + 0.5;
        
        // Color interpolation
        vec3 colorA = vec3(1.0, 0.5, 0.0); // orange
        vec3 colorB = vec3(0.0, 0.5, 1.0); // blue
        vec3 dynamicColor = mix(colorA, colorB, wave);
        
        // Combine effects
        vec3 finalColor = mix(dynamicColor, ourColor, wave2);
        
        FragColor = vec4(finalColor, 1.0);
    }
    else {
        FragColor = vec4(ourColor, 1.0);
    }
}
