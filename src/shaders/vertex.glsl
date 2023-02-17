#version 100
attribute vec3 position;
attribute vec2 texcoord;
varying lowp vec2 uv;
uniform mat4 Model;
uniform mat4 Projection;

uniform float aspect;

void main() {
    uv = texcoord - 0.5;
	uv.y /= aspect;
    gl_Position = Projection * Model * vec4(position, 1);
}