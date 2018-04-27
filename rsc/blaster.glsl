#version 450

//! VERTEX

layout(location = 0) in vec2 position;

void main() {
  gl_Position = vec4(position, 0.0, 1.0);
}

//! FRAGMENT

layout(location = 0) out vec4 f_color;

void main() {
  f_color = vec4(1.0, 0.0, 0.0, 1.0);
}
