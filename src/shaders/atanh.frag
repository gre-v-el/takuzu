#version 100
precision highp float;

varying vec2 uv;
uniform float time;
uniform vec2 resolution;

// https://www.shadertoy.com/view/sddfR4
// License CC0: More Complex Atanh
//  Inspired by: Complex Atanh  - https://www.shadertoy.com/view/tsBXRW
//  I always thought Complex Atanh by mla was very cool
//  I tinkered a bit with it on saturday morning and got something 
//  I think is different enough to share

#define RESOLUTION  resolution
#define TIME        time*0.1
#define PI          3.141592654
#define TAU         (2.0*PI)
#define ROT(a)      mat2(cos(a), sin(a), -sin(a), cos(a))

// License: WTFPL, author: sam hocevar, found: https://stackoverflow.com/a/17897228/418488
const vec4 hsv2rgb_K = vec4(1.0, 2.0 / 3.0, 1.0 / 3.0, 3.0);
vec3 hsv2rgb(vec3 c) {
  vec3 p = abs(fract(c.xxx + hsv2rgb_K.xyz) * 6.0 - hsv2rgb_K.www);
  return c.z * mix(hsv2rgb_K.xxx, clamp(p - hsv2rgb_K.xxx, 0.0, 1.0), c.y);
}
// License: WTFPL, author: sam hocevar, found: https://stackoverflow.com/a/17897228/418488
//  Macro version of above to enable compile-time constants
#define HSV2RGB(c)  (c.z * mix(hsv2rgb_K.xxx, clamp(abs(fract(c.xxx + hsv2rgb_K.xyz) * 6.0 - hsv2rgb_K.www) - hsv2rgb_K.xxx, 0.0, 1.0), c.y))

// License: MIT OR CC-BY-NC-4.0, author: mercury, found: https://mercury.sexy/hg_sdf/
vec2 mod2(inout vec2 p, vec2 size) {
  vec2 c = floor((p + size*0.5)/size);
  p = mod(p + size*0.5,size) - size*0.5;
  return c;
}

float circle8(vec2 p, float r) {
  p *= p;
  p *= p;
  return pow(dot(p, p),1.0/8.0)-r;
}

// License: Unknown, author: Unknown, found: don't remember
float tanh_approx(float x) {
  //  Found this somewhere on the interwebs
  //  return tanh(x);
  float x2 = x*x;
  return clamp(x*(27.0 + x2)/(27.0+9.0*x2), -1.0, 1.0);
}

// Complex trig functions found at: Complex Atanh  - https://www.shadertoy.com/view/sl3XW7
//  A very cool shader
vec2 cmul(vec2 z, vec2 w) {
  return mat2(z,-z.y,z.x)*w;
}

vec2 cinv(vec2 z) {
  float t = dot(z,z);
  return vec2(z.x,-z.y)/t;
}

vec2 cdiv(vec2 z, vec2 w) {
  return cmul(z,cinv(w));
}

vec2 clog(vec2 z) {
  float r = length(z);
  return vec2(log(r),atan(z.y,z.x));
}

vec2 catanh(vec2 z) {
  return 0.5*clog(cdiv(vec2(1,0)+z,vec2(1,0)-z));
}

// My own attempt at a ctanh
vec2 cexp(vec2 z) {
  float r = exp(z.x);
  return r*vec2(cos(z.y), sin(z.y));
}

vec2 ctanh(vec2 z) {
  z = cexp(2.0*z);
  return cdiv(vec2(1,0)-z,vec2(1,0)+z);
}

vec2 transform(vec2 p) {
  float a = 0.5*TIME;
  p *= mix(2.0, 0.5, smoothstep(-0.85, 0.85, cos(0.5*a)));
  p = ctanh(p);
  p *= ROT(0.2*a);
  p += 1.5*vec2(cos(0.3*a), sin(0.4*a));
  p = catanh(p);
  p.x -= 0.2*a;
  return p;
}

vec3 effect(vec3 col, vec2 p_) {
  const float scale = 1.0/PI;
  const float cellw = 0.05;
  p_ *= ROT(0.05*TIME);

  float aaa = 2.0/RESOLUTION.y;
  vec2 np_ = p_+aaa;
  vec2 p   = transform(p_);
  vec2 np  = transform(np_);
  float aa = distance(p, np)*sqrt(0.5);

  p *= scale;
  aa *= scale;

  vec2 n = floor(p/cellw);
  p = mod(p, cellw);
  p -= 0.5*cellw;
  float fo = tanh_approx(aaa/(aa));
  float d = circle8(p, 0.45*cellw);
  col = mix(col, hsv2rgb(vec3(fract(0.1*n.y+0.05*n.x+0.05*TIME), mix(0., 0.95, fo), mix(0.9, 0.85, fo*fo))), smoothstep(aa, -aa, d)*step(aa, 0.7));

  return col;
}

void main() {
  vec2 q = gl_FragCoord.xy/resolution.xy;
  vec2 p  = -1. + 2. * q;
  p.x *= RESOLUTION.x/RESOLUTION.y;
  
  vec3 col = vec3(1.0);
  col = effect(col, p);
  col = clamp(col, 0.0, 1.0);
  col = sqrt(col);
  
  gl_FragColor = vec4(col*0.2, 1.0);
}
