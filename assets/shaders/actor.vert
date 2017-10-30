#version 140

in mediump vec2 position;
uniform mediump float angle;
uniform mediump float scale;
uniform mediump vec2 translate;
uniform mediump float z;

void main() {
    float theta = 3.14159265359f * (angle + 270.0f) / 180.0f;
    float ct = cos(theta);
    float st = sin(theta);
    mat2 rotation = mat2( ct, st,
                         -st, ct);
    vec2 view_scale = vec2(1.0f / 320.0f, 1.0f / 240.0f);
    vec2 view_translation = vec2(-1.0f, -1.0f);
    vec2 xy = view_translation + view_scale * (translate + scale * (rotation * position));
    gl_Position = vec4(xy, z, 1.0f);
}
