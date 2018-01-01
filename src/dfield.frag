#version 330

uniform sampler2D t_Texture;
in vec2 v_Uv;
in vec4 v_Color;
out vec4 Target0;

layout (std140) uniform Globals {
	mat4 u_MVP;
};

layout (std140) uniform VectorShaderConsts {
	float ignore;
};

void main() {
	float dfield_offset = texture(t_Texture, v_Uv).r;
	float alpha_mask = clamp((dfield_offset - 0.22) * 200, 0.0, 1.0);

	Target0 = vec4(dfield_offset, dfield_offset, dfield_offset, alpha_mask);
}
