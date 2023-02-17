#version 100
precision highp float;

varying vec2 uv;
uniform float time;

// TOO SLOW, NOT WORTH IT

const vec2 start_pos = vec2(-1.0, 0.0);
const float start_rot = 0.0;
const float start_zoom = 3.5;
const int start_iters = 80;

// const vec2 end_pos = vec2(-1.254802703, -0.382753676);
// const float end_rot = 2.16;
// const float end_zoom = 0.01;
// const int end_iters = 500;
const vec2 end_pos = vec2(-1.786201, 0.0);
const float end_rot = 0.0;
const float end_zoom = 0.0012;
const int end_iters = 500;

vec3 hsv2rgb(vec3 c) {
	c = clamp(c, 0.0, 1.0);
	vec4 K = vec4(1.0, 2.0/3.0, 1.0/3.0, 3.0);
	vec3 p = abs(fract(c.xxx + K.xyz) * 6.0 - K.www);
	return clamp(c.z * mix(K.xxx, clamp(p - K.xxx, 0.0, 1.0), c.y), 0.0, 1.0);
}

vec2 square_complex(vec2 z) {
	return vec2(z.x*z.x - z.y*z.y, 2.0*z.x*z.y);
	// a2 - b2 + 2abi
}

vec2 lerp(vec2 a, vec2 b, float t) {
	return b*t + (1.0-t)*a;
}
float lerp(float a, float b, float t) {
	return b*t + (1.0-t)*a;
}
float inv_lerp(float v, float a, float b) {
	return (v-a)/(b-a);
}

float map(float v, float a1, float b1, float a2, float b2) {
	return lerp(inv_lerp(v, a1, b1), a2, b2);
}

vec2 rotate(vec2 v, float r) {
	return vec2(cos(r)*v.x - sin(r)*v.y, sin(r)*v.x+cos(r)*v.y);
}

float get_mandelbrot(vec2 uv, float t) {
	
	float zoom = pow(2.718, lerp(log(start_zoom), log(end_zoom), t));
	float t_zoom = inv_lerp(zoom, start_zoom, end_zoom);
	vec2 pos = lerp(start_pos, end_pos, t_zoom);
	float rot = lerp(start_rot, end_rot, t);
	int iters = int(lerp(float(start_iters), float(end_iters), t_zoom));

	vec2 c = rotate(uv*zoom, rot) + pos;
	vec2 z = c;
	int i = 0;
	for(; i < iters; i ++) {
		z = square_complex(z) + c;
		if(length(z) > 2.0) {
			break;
		}
	}
	if(i == iters) {
		return 0.0;
	}
	if(i <= 1) {
		return 1.0;
	}
	return float(i)/float(iters);
	// return (float(i) - log(log(length(z))/log(2.0)))/float(i);
}

void main() {

	
	// float t = step(mod(time*2.0, 1.0), 0.5);//-pow(0.5, time * 0.1);
	float t = mod(time*0.02, 1.0);


	float man1 = get_mandelbrot(uv, t);
	vec3 rgb1 = hsv2rgb(vec3(pow(man1, 0.8), 1.0, step(0.000001, man1))).yzx;
	
	float man2 = get_mandelbrot(uv, t + 1.0);
	vec3 rgb2 = hsv2rgb(vec3(pow(man2, 0.8), 1.0, step(0.000001, man2))).yzx;

	vec3 rgb = mix(rgb1, rgb2, smoothstep(0.05, 0.0, t));
	// vec3 rgb = mix(rgb1, rgb2, 0.5);

	gl_FragColor = vec4(rgb*0.5,1.0);
}