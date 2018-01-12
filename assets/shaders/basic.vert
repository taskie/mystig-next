#version 300 es

in mediump vec2 position;
uniform mediump float z;
uniform mediump mat4 mvp_matrix;

void main() {
    gl_Position = mvp_matrix * vec4(position.x / 320.0f - 1.0f,
                                    -position.y / 240.0f + 1.0f,
                                    z,
                                    1.0f);
}
