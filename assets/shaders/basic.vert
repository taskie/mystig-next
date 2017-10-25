#version 140

in mediump vec2 position;
uniform mediump float z;

void main() {
    gl_Position = vec4(position, z, 1.0);
}
