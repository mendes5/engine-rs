#pragma SHADER
#version 420
precision highp float;

in float v_scalar;

layout (std140, binding = 0) uniform ViewportUBO
{
    uniform mat4 projection;
    uniform mat4 view;
};

void main() {
    gl_Position = projection * view * vec4(v_scalar);
}

#pragma SHADER
#version 420
precision highp float;

out vec4 fragColor;

void main() {
    fragColor = vec4(0.0);
}

