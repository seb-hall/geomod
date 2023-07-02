#version 330
out vec4 FragColor;

uniform vec2 gridOffset;
uniform float gridScale;

void main() {

    float gridSpacing = 50*gridScale; 


    vec2 pos = gl_FragCoord.xy + gridOffset;

    vec2 majorOffset = mod(pos, gridSpacing);
    if (majorOffset.x < 1 || majorOffset.y < 1) {
        FragColor = vec4(1.0, 1.0, 1.0, 0.125);
        return;
    }

    vec2 minorOffset = mod(pos, gridSpacing / 10.0);
    if (minorOffset.x < 1 || minorOffset.y < 1) {
        FragColor = vec4(1.0, 1.0, 1.0, 0.05);
        return;
    }

    

    FragColor = vec4(0.0);
}
