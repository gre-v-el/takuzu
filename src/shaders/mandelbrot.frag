#version 100
precision highp float;

varying vec2 uv;
uniform float time;

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

float get_mandelbrot(vec2 uv, vec2 pos, float zoom, float rot, int iters) {
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
	return float(i)/float(iters);
	// return (float(i) - log(log(length(z))/log(2.0)))/float(i);
}

void main() {

	vec2 start_pos = vec2(-1.0, 0.0);
	float start_rot = 0.0;
	float start_zoom = 3.5;
	int start_iters = 20;

	vec2 end_pos = vec2(-1.254802703, -0.382753676);
	float end_rot = 2.16;
	float end_zoom = 0.01;
	int end_iters = 180;
	
	// float t = step(mod(time*2.0, 1.0), 0.5);//-pow(0.5, time * 0.1);
	float t_linear = mod(time*0.5, 1.0);

	float zoom = pow(2.718, lerp(log(start_zoom), log(end_zoom), t_linear));
	float t_zoom = inv_lerp(zoom, start_zoom, end_zoom);
	vec2 pos = lerp(start_pos, end_pos, t_zoom);
	float rot = lerp(start_rot, end_rot, t_linear);
	int iters = int(lerp(float(start_iters), float(end_iters), t_linear));

	float col = get_mandelbrot(uv, pos, zoom, rot, iters);
	col = pow(col, 0.5);

	gl_FragColor = vec4(vec3(col)*0.5,1.0);
}