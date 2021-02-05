#pragma SHADER
#version 420
precision highp float;

in vec3 position;
in vec3 color;

layout (std140, binding = 0) uniform ViewportUBO
{
    uniform mat4 projection;
    uniform mat4 view;
};

out vec3 v_color;

void main() {
    gl_Position = projection * view * vec4(position, 1.0);
    v_color = color;
}

#pragma SHADER
#version 420
precision highp float;

in vec3 v_color;

out vec4 fragColor;

void main() {
    fragColor = vec4(v_color, 1.0);
}

