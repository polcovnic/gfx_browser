attribute vec2 a_Pos;
attribute vec4 a_Color;
varying vec4 v_Color;

void main() {
    v_Color = vec4(a_Color);
    gl_Position = vec4(a_Pos, 0.0, 1.0);
}