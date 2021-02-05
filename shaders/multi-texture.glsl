#pragma SHADER
#version 420
precision mediump float;

in vec3 position;
in vec2 uv;

layout (std140, binding = 0) uniform ViewportUBO
{
    uniform mat4 projection;
    uniform mat4 view;
};


out vec2 f_uv;

void main() {
    gl_Position = projection * view * vec4(position, 1.0);
    f_uv = uv;
}

#pragma SHADER
#version 420
precision mediump float;

in vec2 f_uv;

uniform sampler2D texture1;
uniform sampler2D texture2;

out vec4 fragColor;

void main() {
    if (f_uv.x > 0.5) {
        fragColor = texture(texture1, f_uv);
    } else {
        fragColor = texture(texture2, f_uv);
    }
}
