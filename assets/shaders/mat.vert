#version 140

in mediump vec3 position;
uniform mediump mat4 mvp;

void main() {
    gl_Position = mvp * vec4(position, 1.0f);
}
