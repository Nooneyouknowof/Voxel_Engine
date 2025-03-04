#version 460

layout(location = 0) in vec3 inPosition;  // Vertex position input
layout(location = 1) in vec3 inColor;     // Vertex color input

layout(location = 0) out vec3 fragColor;  // Output color to fragment shader

layout(set = 0, binding = 0) uniform UniformBufferObject {
    mat4 model;
    mat4 view;
    mat4 proj;
} ubo;

void main() {
    gl_Position = ubo.proj * ubo.view * ubo.model * vec4(inPosition, 1.0);
    fragColor = inColor; // Pass the color to the fragment shader
}