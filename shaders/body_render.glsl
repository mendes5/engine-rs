#pragma SHADER
#version 420
precision highp float;

in vec3 position;
in vec3 color;

uniform vec3 u_position;
uniform vec4 u_orientation;
uniform vec3 u_size;

layout (std140, binding = 0) uniform ViewportUBO
{
    uniform mat4 projection;
    uniform mat4 view;
};

out vec3 v_color;

vec3 rotate_vector( vec4 quat, vec3 vec )
{
return vec + 2.0 * cross( cross( vec, quat.xyz ) + quat.w * vec, quat.xyz );
}

// https://twistedpairdevelopment.wordpress.com/2013/02/11/rotating-a-vector-by-a-quaternion-in-glsl/

vec4 multQuat(vec4 q1, vec4 q2)
{
return vec4(
q1.w * q2.x + q1.x * q2.w + q1.z * q2.y - q1.y * q2.z,
q1.w * q2.y + q1.y * q2.w + q1.x * q2.z - q1.z * q2.x,
q1.w * q2.z + q1.z * q2.w + q1.y * q2.x - q1.x * q2.y,
q1.w * q2.w - q1.x * q2.x - q1.y * q2.y - q1.z * q2.z
);
}

void main() {
  vec3 vertext = rotate_vector(u_orientation.zyxw, (position * u_size)) + (u_position);
  

  gl_Position = projection * view * vec4(vertext, 1.0);
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

