#pragma SHADER
#version 420
precision mediump float;

in vec2 position;
in vec2 uv;

layout (std140, binding = 0) uniform ViewportUBO
{
    uniform mat4 projection;
    uniform mat4 view;
};

out vec2 f_uv;

void main() {
    gl_Position = vec4(position, -1.0, 1.0);
    f_uv = uv;
}

#pragma SHADER
#version 420
precision mediump float;

in vec2 f_uv;

uniform sampler2D u_texture;

out vec4 fragColor;

void main() {
    fragColor = texture(u_texture, f_uv);
}
