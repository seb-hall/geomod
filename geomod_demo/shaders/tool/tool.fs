#version 330
out vec4 FragColor;

uniform vec2 mousePos;
uniform vec2 gridOffset;
uniform float gridScale;


void main() {

    float gridSpacing = 50*gridScale; 


    vec2 pos = gl_FragCoord.xy;

    vec2 mouseOffset = mousePos - pos;


    if (length(mousePos) < 10.0) {
        FragColor = vec4(1.0, 0.0, 0.0, 0.5);
        return;
    }

    FragColor = vec4(0.0);
}
