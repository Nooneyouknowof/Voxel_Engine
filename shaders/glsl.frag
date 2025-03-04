#version 460
layout(location = 0) in vec3 fragColor;  // Color from vertex shader
layout(location = 0) out vec4 outColor;  // Output final color

void main() {
    outColor = vec4(fragColor, 1.0); // Set fragment color
}