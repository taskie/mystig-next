#version 140

in mediump vec2 position;
in mediump vec4 vert_color;
out mediump vec4 my_color;

void main() {
    gl_Position = vec4(position, 0.0, 1.0);
    my_color = vert_color;
}
