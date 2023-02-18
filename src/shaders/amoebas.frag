#version 100
precision highp float;

varying vec2 uv;
uniform float time;
uniform vec2 resolution;

// https://www.shadertoy.com/view/7d23DR
// License CC0: 2D Amoebas
//  While messing around I stumbled on a simple "amoeba" lika effect.
//  Nothing complicated but nice IMHO so I shared

#define RESOLUTION resolution
#define TIME       time*0.3

float circle(vec2 p, float r) {
  return length(p) - r;
}

// IQ's polynominal min
float pmin(float a, float b, float k) {
    float h = clamp( 0.5+0.5*(b-a)/k, 0.0, 1.0 );
    return mix( b, a, h ) - k*h*(1.0-h);
}

// http://mercury.sexy/hg_sdf/
vec2 mod2(inout vec2 p, vec2 size) {
  vec2 c = floor((p + size*0.5)/size);
  p = mod(p + size*0.5,size) - size*0.5;
  return c;
}


float df(vec2 p) {
  // Generates a grid of dots
  vec2 dp = p;
  vec2 dn = mod2(dp, vec2(0.25));
  float ddots = length(dp);
  
  // Blobs
  float dblobs = 1E6;
  for (int i = 0; i < 5; ++i) {
    float dd = circle(p-1.0*vec2(sin(TIME+float(i)), sin(float(i*i)+TIME*sqrt(0.5))), 0.1);
    dblobs = pmin(dblobs, dd, 0.35);
  }

  float d = 1E6;
  d = min(d, ddots);
  // Smooth min between blobs and dots makes it look somewhat amoeba like
  d = pmin(d, dblobs, 0.35);
  return d;
}

vec3 postProcess(vec3 col, vec2 q)  {
  col=pow(clamp(col,0.0,1.0),vec3(1.0/2.2)); 
  col=col*0.6+0.4*col*col*(3.0-2.0*col);  // contrast
  col=mix(col, vec3(dot(col, vec3(0.33))), -0.4);  // satuation
  col*=0.5+0.5*pow(19.0*q.x*q.y*(1.0-q.x)*(1.0-q.y),0.7);  // vigneting
  return col;
}

void main() {
  vec2 q = gl_FragCoord.xy/RESOLUTION.xy;
  vec2 p = -1. + 2. * q;
  p.y *= RESOLUTION.y/RESOLUTION.x;
  float aa = 2.0/RESOLUTION.y;
 
  const float z = 1.4;
  float d = df(p/z)*z; 
  
  vec3 col = vec3(0.33);
  col = mix(col, vec3(.0), smoothstep(-aa, aa, -d));

  col = postProcess(col, q);
  gl_FragColor = vec4(col*0.4, 1.0);
}

