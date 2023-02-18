#version 100
precision highp float;

varying vec2 uv;
uniform float time;
uniform float alpha;
uniform vec2 resolution;

// https://www.shadertoy.com/view/mlf3Rl
// CC0: Infinite Arcs II
//  Rethinking a bit on how do the infinite "zoom". 
//  If I expotential zoom then it becomes scale invariant
//  which ended up quite useful
//  Potentially useful for future raymarchers.

#define TIME        time*0.3
#define RESOLUTION  resolution
#define PI          3.141592654
#define TAU         (2.0*PI)
#define ROT(a)      mat2(cos(a), sin(a), -sin(a), cos(a))

const float ExpBy = log2(1.2);

// License: WTFPL, author: sam hocevar, found: https://stackoverflow.com/a/17897228/418488
const vec4 hsv2rgb_K = vec4(1.0, 2.0 / 3.0, 1.0 / 3.0, 3.0);
vec3 hsv2rgb(vec3 c) {
  vec3 p = abs(fract(c.xxx + hsv2rgb_K.xyz) * 6.0 - hsv2rgb_K.www);
  return c.z * mix(hsv2rgb_K.xxx, clamp(p - hsv2rgb_K.xxx, 0.0, 1.0), c.y);
}
// License: WTFPL, author: sam hocevar, found: https://stackoverflow.com/a/17897228/418488
//  Macro version of above to enable compile-time constants
#define HSV2RGB(c)  (c.z * mix(hsv2rgb_K.xxx, clamp(abs(fract(c.xxx + hsv2rgb_K.xyz) * 6.0 - hsv2rgb_K.www) - hsv2rgb_K.xxx, 0.0, 1.0), c.y))

// License: Unknown, author: Unknown, found: don't remember
float hash(float co) {
  return fract(sin(co*12.9898) * 13758.5453);
}

vec2 sca(float a) {
  return vec2(sin(a), cos(a)); 
}

// License: MIT, author: Inigo Quilez, found: https://iquilezles.org/www/articles/distfunctions2d/distfunctions2d.htm
float arc(vec2 p, vec2 sc, float ra, float rb) {
  // sc is the sin/cos of the arc's aperture
  p.x = abs(p.x);
  return ((sc.y*p.x>sc.x*p.y) ? length(p-sc*ra) : 
                                  abs(length(p)-ra)) - rb;
}

float forward(float n) {
  return exp2(ExpBy*n);
}

float reverse(float n) {
  return log2(n)/ExpBy;
}

vec2 cell(float n) {
  float n2  = forward(n);
  float pn2 = forward(n-1.0);
  float m   = (n2+pn2)*0.5;
  float w   = (n2-pn2)*0.5;
  return vec2(m, w);
}

vec2 df(vec2 p) {
  const float w = 2.0/3.0;
  
  float tm = TIME;
  float m = fract(tm);
  float f = floor(tm);
  float z = forward(m);
  
  vec2 p0 = p;
  p0 /= z;

  float l0 = length(p0);
  float n0 = ceil(reverse(l0));
  vec2 c0 = cell(n0); 
  
  float h0 = hash(n0-f);
  float h1 = fract(3677.0*h0);
  float h2 = fract(8677.0*h0);
  float sh2 = (h2-0.5)*2.0;

  float a = TAU*h2+sqrt(abs(sh2))*sign(sh2)*TIME*TAU/20.0;
  p0 *= ROT(a);
  float d0 = arc(p0, sca(PI/4.0+0.5*PI*h1), c0.x, c0.y*w);
  d0 *= z;
  return vec2(d0, h0);
}

void main() {
  float aa = 2.0/RESOLUTION.y;
  vec2 q = gl_FragCoord.xy/RESOLUTION.xy;
  vec2 p = -1. + 2. * q;
  p.x *= RESOLUTION.x/RESOLUTION.y;
  vec2 d2 = df(p);

  vec3 col = vec3(0.0);
  vec3 bcol = hsv2rgb(vec3(d2.y, 0.9, smoothstep(10.0*aa, 20.0*aa, length(p))));
  col = mix(col, bcol, smoothstep(aa, -aa, d2.x));
  col = sqrt(col);
  gl_FragColor = vec4(col * 0.2, alpha);
}
