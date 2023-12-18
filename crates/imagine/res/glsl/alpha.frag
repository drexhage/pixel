precision highp float;
varying vec2 texCoords;

uniform sampler2D baseSampler;
uniform vec2 basePosition;
uniform float baseAlpha;
uniform vec2 baseSize;

uniform sampler2D activeSampler;
uniform vec2 activePosition;
uniform float activeAlpha;
uniform vec2 activeSize;

uniform vec2 windowPosition;
uniform vec2 windowSize;

void main() {
  vec2 relative = vec2(texCoords.x, 1.0 - texCoords.y);

  vec2 basePos = relative * (windowSize / baseSize) - (basePosition - windowPosition) / baseSize;
  vec2 activePos = relative * (windowSize / activeSize) - (activePosition - windowPosition) / activeSize;
  bool basePosValid = basePos.x >= 0. && basePos.x <= 1. && basePos.y >= 0. && basePos.y <= 1.;
  bool activePosValid = activePos.x >= 0. && activePos.x <= 1. && activePos.y >= 0. && activePos.y <= 1.;

  if (basePosValid && activePosValid) {
    vec4 base = texture2D(baseSampler, basePos);
    float alpha_b = baseAlpha * base.a;

    vec4 active = texture2D(activeSampler, activePos);
    float alpha_a = activeAlpha * active.a;

    float alpha = alpha_a + alpha_b * (1.0 - alpha_a);
    vec4 c = (active * alpha_a + base * alpha_b * (1.0 - alpha_a)) / alpha;
    gl_FragColor = vec4(c.rgb, alpha);
  } else if (basePosValid) {
    gl_FragColor = texture2D(baseSampler, basePos);
  } else if (activePosValid) {
    gl_FragColor = texture2D(activeSampler, activePos);
  } else {
    gl_FragColor = vec4(0.0, 0.0, 0.0, 0.0);
  }
}