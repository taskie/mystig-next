#version 140

in mediump vec2 position;
uniform mediump float z;

void main() {
    gl_Position = vec4(position.x / 320.0f - 1.0f,
                       -position.y / 240.0f + 1.0f,
                       z,
                       1.0f);
}
