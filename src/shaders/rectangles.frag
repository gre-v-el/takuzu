#version 100
precision highp float;

varying vec2 uv;
uniform float time;
uniform vec2 resolution;

// https://www.shadertoy.com/view/dtXGWj
// CC0: Trippy rectangles
//  Continuation of last night trippy effects

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
float hash(vec2 co) {
  return fract(sin(dot(co.xy ,vec2(12.9898,58.233))) * 13758.5453);
}

// License: Unknown, author: Unknown, found: don't remember
float tanh_approx(float x) {
  //  Found this somewhere on the interwebs
  //  return tanh(x);
  float x2 = x*x;
  return clamp(x*(27.0 + x2)/(27.0+9.0*x2), -1.0, 1.0);
}

// License: MIT OR CC-BY-NC-4.0, author: mercury, found: https://mercury.sexy/hg_sdf/
float mod1(inout float p, float size) {
  float halfsize = size*0.5;
  float c = floor((p + halfsize)/size);
  p = mod(p + halfsize, size) - halfsize;
  return c;
}

// License: MIT, author: Inigo Quilez, found: https://iquilezles.org/www/articles/distfunctions2d/distfunctions2d.htm
float box(vec2 p, vec2 b) {
  vec2 d = abs(p)-b;
  return length(max(d,0.0)) + min(max(d.x,d.y),0.0);
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

vec2 df(vec2 p, float aa) {
  float tm = TIME;
  float m = fract(tm);
  float f = floor(tm);
  float z = forward(m);
  
  vec2 p0 = p;
  p0 /= z;
  vec2 sp0 = sign(p0);
  p0 = abs(p0);

  float l0x = p0.x;
  float n0x = ceil(reverse(l0x));
  vec2 c0x  = cell(n0x); 


  float l0y = p0.y;
  float n0y = ceil(reverse(l0y));
  vec2 c0y  = cell(n0y); 


  vec2 p1 = vec2(p0.x, p0.y);
  vec2 o1 = vec2(c0x.x, c0y.x);
  vec2 c1 = vec2(c0x.y, c0y.y);
  p1 -= o1;
  
  float r1 = 0.5*aa/z;

  vec2 p2 = p1;
  vec2 c2 = c1;
  float n2 = 0.0; 
  
  if (c1.x < c1.y) {
    float f2 = floor(c1.y/c1.x);
    c2 = vec2(c1.x, c1.y/f2);
    if (fract(0.5*f2) < 0.5) {
      p2.y -= -c2.y;
    }
    
    n2 = mod1(p2.y, 2.0*c2.y);
  } else if (c1.x > c1.y){
    float f2 = floor(c1.x/c1.y);
    c2 = vec2(c1.x/f2, c1.y);
    if (fract(0.5*f2) < 0.5) {
      p2.x -= -c2.x;
    }

    n2 = mod1(p2.x, 2.0*c2.x);
  }
  float h0 = hash(n2+vec2(n0x, n0y)-vec2(f)+vec2(sp0.x, sp0.y));
  
  float d2 = box(p2, c2-2.0*r1)-r1;
  
  float d = d2;
  d *= z;

  return vec2(d, h0);
}

vec4 effect(vec2 p, float hue) {
  float aa = 2.0/RESOLUTION.y;
  vec2 d2 = df(p, aa);

  vec3 col = vec3(0.0);
  vec3 bcol = hsv2rgb(vec3(fract(hue+0.3*d2.y), 0.85, 1.0));
  return vec4(bcol, smoothstep(aa, -aa, d2.x));
}

void main() {
  vec2 q = gl_FragCoord.xy/RESOLUTION.xy;
  vec2 p = -1. + 2. * q;
  vec2 ppp = p;
  p.x *= RESOLUTION.x/RESOLUTION.y;
  float hue = -length(p)+0.1*TIME; 

  vec2 pp = p;
  vec3 col = vec3(0.0);
  vec4 col0 = effect(pp, hue);
  col = mix(col, col0.xyz, col0.w);

  col += hsv2rgb(vec3(hue, 0.66, 4.0))*mix(1.0, 0.0, tanh_approx(2.0*sqrt(length(p))));
  col *= smoothstep(1.5, 0.5, length(ppp));
  col = sqrt(col);
  gl_FragColor = vec4(col*0.2, 1.0);
}

