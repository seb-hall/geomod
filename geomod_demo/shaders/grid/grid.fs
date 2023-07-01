#version 330
out vec4 FragColor;

uniform vec2 gridOffset;
uniform float gridScale;

void main() {

    float gridSpacing = 50 + gridScale; 
    vec2 pos = gl_FragCoord.xy + gridOffset;
    vec2 offset = mod(pos, gridSpacing);
    if (offset.x < 1 || offset.y < 1) {
        FragColor = vec4(1.0, 1.0, 1.0, 0.125);
    } else {
        FragColor = vec4(0.0);
    }

    
}
