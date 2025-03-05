@echo on
glslc shaders/glsl.vert -o shaders/glsl.vert.spv
glslc shaders/glsl.frag -o shaders/glsl.frag.spv
spirv-val shaders/glsl.vert.spv
spirv-val shaders/glsl.frag.spv
pause