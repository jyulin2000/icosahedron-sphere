#version 330

in vec3 position;
uniform float time_s;

void main() {
    float s = 1 + 0.1 * sin(2.0 * time_s);
    //gl_Position = vec4(position, 1.0);
    //gl_Position = vec4(s * position.x, s * position.y, s * position.z, 1.0);
    gl_Position = vec4(position, 1.0);
}