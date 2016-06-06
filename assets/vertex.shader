#version 330
layout (location = 0) in vec2 position;
layout (location = 1) in vec2 scale;
layout (location = 2) in float rotation;
layout (location = 3) in vec4 color;

out vec2 texture_coord;
out vec4 fragment_color;

const vec2 pos[] = vec2[4](
	vec2(-1.0f, -1.0f),
	vec2( 1.0f, -1.0f),
	vec2(-1.0f,  1.0f),
	vec2( 1.0f,  1.0f)
);

void main () {
	vec2 offset = pos[gl_VertexID];
	mat4 rotation_matrix = mat4(cos(rotation), -sin(rotation), 0.0, 0.0,
	                            sin(rotation),  cos(rotation), 0.0, 0.0,
	                            0.0, 0.0, 1.0, 0.0,
	                            0.0, 0.0, 0.0, 1.0);

	vec4 offset_rotated = rotation_matrix * vec4(offset * scale, 1.0, 1.0);
	vec4 position_result = vec4(offset_rotated.xy + position.xy, 1.0, 1.0);
	gl_Position = position_result;

	texture_coord = (offset + vec2(1.0)) / vec2(2.0);
	texture_coord.y = 1.0 - texture_coord.y;

	fragment_color = color;
}
