attribute vec3 vin_position;
//attribute float vin_blue;
//attribute vec2 vin_color;

varying vec2 v_color;
varying float v_blue;

void main(void)
{
    //v_color = vin_color;
    //v_blue = vin_blue;
    v_color = vec2(1., 0.);
    v_blue = 0.0;

    gl_Position = vec4(vin_position, 1.0);
}
