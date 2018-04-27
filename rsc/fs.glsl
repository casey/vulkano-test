#version 450

layout(location = 0) out vec4 f_color;

layout(binding = 0) uniform Data {
  float time;
};

void main() {
    f_color = vec4(1.0, mod(time, 1.0), 1.0, 1.0);
}
