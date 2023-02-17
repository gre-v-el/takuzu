#version 100
precision highp float;
varying vec2 uv;

uniform float time;


// Interstellar
// https://www.shadertoy.com/view/Xdl3D2
// Hazel Quantock
// This code is licensed under the CC0 license http://creativecommons.org/publicdomain/zero/1.0/


// Gamma correction
#define GAMMA 2.2

vec3 to_gamma(vec3 col) {
	return pow(col, vec3(1.0/GAMMA));
}

float random(vec2 v) {
	v = fract(v/128.)*128. + vec2(-64.340622, -72.465622);
    return fract(dot(v.xyx * v.xyy, vec3(20.390625, 60.703125, 2.4281209)));
}


void main() {
	vec3 ray = vec3(uv, 1.0);

	float offset = time*0.1;	
	float speed2 = 0.2;
	float speed = 0.3;
	
	vec3 col = vec3(0);
	
	vec3 stp = ray/max(abs(ray.x),abs(ray.y));
	
	vec3 pos = 2.0*stp+.5;
	for ( int i=0; i < 20; i++ )
	{
		float z = random(floor(pos.xy));
		z = fract(z-offset);
		float d = 50.0*z-pos.z;
		float w = pow(max(0.0,1.0-8.0*length(fract(pos.xy)-.5)),2.0);
		vec3 c = max(vec3(0),vec3(1.0-abs(d+speed2*.5)/speed,1.0-abs(d)/speed,1.0-abs(d-speed2*.5)/speed));
		col += 1.5*(1.0-z)*c*w;
		pos += stp;
	}
	
	gl_FragColor = vec4(to_gamma(col)*0.5,1.0);
}