#version 100
precision highp float;

varying vec2 uv;
uniform float time;
uniform vec2 resolution;

// CC0: Colorful bubbles underwater
// https://www.shadertoy.com/view/dtBSRV
//  Twitter art again that I attempted to recreate with a bit of a twist

#define RAINBOW_COLORS

#define TIME        time*0.2
#define RESOLUTION  resolution
#define PI          3.141592654
#define TAU         (2.0*PI)
const float MaxIter = 12.0;

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
// License: Unknown, author: Unknown, found: don't remember
float hash(vec2 co) {
  return fract(sin(dot(co.xy ,vec2(12.9898,58.233))) * 13758.5453);
}

// License: MIT OR CC-BY-NC-4.0, author: mercury, found: https://mercury.sexy/hg_sdf/
vec2 mod2(inout vec2 p, vec2 size) {
  vec2 c = floor((p + size*0.5)/size);
  p = mod(p + size*0.5,size) - size*0.5;
  return c;
}

vec4 plane(vec2 p, float i, float zf, float z, vec3 bgcol) {
  float sz = 0.5*zf;
  vec2 cp = p;
  vec2 cn = mod2(cp, vec2(2.0*sz, sz));
  float h0 = hash(cn+i+123.4);
  float h1 = fract(4483.0*h0);
  float h2 = fract(8677.0*h0);
  float h3 = fract(9677.0*h0);
  float h4 = fract(7877.0*h0);
  float h5 = fract(9967.0*h0);
  if (h4 < 0.5) {
    return vec4(0.0);
  }
  float fi = exp(-0.25*max(z-1.0, 0.0));
  float aa = mix(6.0, 1.0, fi)*2.0/RESOLUTION.y; 
  float r  = sz*mix(0.1, 0.475, h0*h0);
  float amp = mix(0.25, 0.5, h3)*r;
  cp.x -= amp*sin(mix(3.0, 0.25, h0)*TIME+TAU*h2);
  cp.x += 0.95*(sz-r-amp)*sign(h3-0.5)*h3;
  cp.y += 0.475*(sz-2.0*r)*sign(h5-0.5)*h5;
  float d = length(cp)-r;
#if defined(RAINBOW_COLORS)
  vec3 hsv = vec3(h1, 0.75, 1.5);
  vec3 ocol = hsv2rgb(hsv);
  vec3 icol = hsv2rgb(hsv*vec3(1.0, 0.5, 1.25));
#else
  vec3 ocol = (0.5+0.5*sin(vec3(0.0, 1.0, 2.0)+h1*TAU));
  vec3 icol = sqrt(ocol);
  ocol *= 1.5;
  icol *= 2.0;
#endif
  vec3 col = mix(icol, ocol, smoothstep(r, 0.0, -d))*mix(0.8, 1.0, h0);
  col = mix(bgcol, col, fi);
  float t = smoothstep(aa, -aa, d);
  return vec4(col, t);
}

// License: Unknown, author: Claude Brezinski, found: https://mathr.co.uk/blog/2017-09-06_approximating_hyperbolic_tangent.html
float tanh_approx(float x) {
  //  Found this somewhere on the interwebs
  //  return tanh(x);
  float x2 = x*x;
  return clamp(x*(27.0 + x2)/(27.0+9.0*x2), -1.0, 1.0);
}

vec3 effect(vec2 p, vec2 pp) {
  const vec3 bgcol0 = HSV2RGB(vec3(0.66, 0.85, 0.1)); 
  const vec3 bgcol1 = HSV2RGB(vec3(0.55, 0.66, 0.6));
  vec3 bgcol = mix(bgcol1, bgcol0, tanh_approx(1.5*length(p)));
  vec3 col = bgcol;
  for (float i = 0.0; i < MaxIter; ++i) {
    const float Near = 4.0;
    float z = MaxIter - i;
    float zf = Near/(Near + MaxIter - i);
    vec2 sp = p;
    float h = hash(i+1234.5); 
    sp.y += -mix(0.2, 0.3, h*h)*TIME*zf;
    sp += h;
    vec4 pcol = plane(sp, i, zf, z, bgcol);
    col = mix(col, pcol.xyz, pcol.w);
  }  
  col *= smoothstep(1.4, 0.5, length(pp));
  col = sqrt(col);
  return col;
}

void main() {
	vec2 q = gl_FragCoord.xy/RESOLUTION.xy;
	vec2 p = -1. + 2. * q;
	vec2 pp = p;
	p.x *= RESOLUTION.x/RESOLUTION.y;
	vec3 col = effect(p, pp);
	gl_FragColor = vec4(col * 0.5, 1.0);
}