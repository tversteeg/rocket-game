#version 330
uniform sampler2D texture_unit;

in vec2 texture_coord;
in vec4 fragment_color;

out vec4 color;

void main () {
	float alpha_mask = texture(texture_unit, texture_coord).r;
	alpha_mask = smoothstep(0.85, 0.9, alpha_mask);
	color = vec4(fragment_color.rgb, alpha_mask);
}
