#include "render.h"

#include <ccore.h>
#include <stdlib.h>
#include <stdio.h>

static int _sprites_max, _sprites_total, _need_update;
static GLuint _program;
static float *_data;
static unsigned char *_color_data;
static GLint _texture_location;
static GLuint _texture, _vertex_array, _data_buffer, _color_buffer;

static void compile_and_check_shader(GLuint shader, const char *shader_file)
{
	int shader_status, shader_log_length;
	char *shader_source, *shader_log;
	FILE *fd;
	size_t bytes;

	fd = fopen(shader_file, "rb");
	if(!fd){
		cc_set_error("Could not open file %s", shader_file);
	}
	fseek(fd, 0, SEEK_END);
  bytes = ftell(fd);
  rewind(fd);

	shader_source = (char*)malloc(bytes + 1);
	if(bytes != fread(shader_source, sizeof(char), bytes, fd)){
		cc_set_error("Error reading file %s", shader_file);
	}
	shader_source[bytes] = '\0';
	fclose(fd);

	glShaderSource(shader, 1, (const char **)&shader_source, NULL);
	glCompileShader(shader);
	glGetShaderiv(shader, GL_COMPILE_STATUS, &shader_status);
  if(!shader_status) {
		glGetShaderiv(shader, GL_INFO_LOG_LENGTH, &shader_log_length);
		shader_log = (char*)malloc(shader_log_length + 1);
		glGetShaderInfoLog(shader, shader_log_length, NULL, shader_log);

		fprintf(stderr, "Compilation error in OpenGL shader:\n%s\nSource:\n%s\n", shader_log, shader_source);
		cc_set_error("OpenGL couldn't compile");
	}

	free(shader_source);
}

static void update_buffers(void)
{
	glBindVertexArray(_vertex_array);

	glBindBuffer(GL_ARRAY_BUFFER, _data_buffer);
	glBufferSubData(GL_ARRAY_BUFFER, 0, _sprites_total * 5 * sizeof(GLfloat), _data);

	glBindBuffer(GL_ARRAY_BUFFER, _color_buffer);
	glBufferSubData(GL_ARRAY_BUFFER, 0, _sprites_total * 4 * sizeof(GLubyte), _color_data);

	glEnableVertexAttribArray(0);
	glEnableVertexAttribArray(1);
	glEnableVertexAttribArray(2);
	glBindBuffer(GL_ARRAY_BUFFER, _data_buffer);
	glVertexAttribPointer(0, 2, GL_FLOAT, GL_FALSE, 5 * sizeof(GLfloat), (GLvoid*)0);
	glVertexAttribPointer(1, 2, GL_FLOAT, GL_FALSE, 5 * sizeof(GLfloat), (GLvoid*)(2 * sizeof(float)));
	glVertexAttribPointer(2, 1, GL_FLOAT, GL_FALSE, 5 * sizeof(GLfloat), (GLvoid*)(4 * sizeof(float)));

	glEnableVertexAttribArray(3);
	glBindBuffer(GL_ARRAY_BUFFER, _color_buffer);
	glVertexAttribPointer(3, 4, GL_UNSIGNED_BYTE, GL_FALSE, 4 * sizeof(GLubyte), (GLvoid*)0);

	glVertexAttribDivisor(0, 1);
	glVertexAttribDivisor(1, 1);
	glVertexAttribDivisor(2, 1);
	glVertexAttribDivisor(3, 1);
}

void init_opengl(const char *fragment_file, const char *vertex_file, int max_sprites)
{
	GLuint vertex_shader, fragment_shader;

	_sprites_max = max_sprites;
	_sprites_total = 0;

	_data = (float*)malloc(_sprites_max * 5 * sizeof(float));
	_color_data = (unsigned char*)malloc(_sprites_max * 4 * sizeof(unsigned char));
	if(!_data || !_color_data){
		cc_out_of_memory_error();
		return;
	}

	cc_bind_opengl_context();

#ifdef USE_GLEW
	glewInit();
#endif

	vertex_shader = glCreateShader(GL_VERTEX_SHADER);
	compile_and_check_shader(vertex_shader, vertex_file);
	fragment_shader = glCreateShader(GL_FRAGMENT_SHADER);
	compile_and_check_shader(fragment_shader, fragment_file);

	_program = glCreateProgram();
	glAttachShader(_program, fragment_shader);
	glAttachShader(_program, vertex_shader);
	glLinkProgram(_program);

	_texture_location = glGetUniformLocation(_program, "texture_unit");
	if(_texture_location < 0){
		cc_set_error("Could not get OpenGL uniform location");
		return;
	}

	glGenVertexArrays(1, &_vertex_array);
	glBindVertexArray(_vertex_array);

	glGenBuffers(1, &_data_buffer);
	glBindBuffer(GL_ARRAY_BUFFER, _data_buffer);
	glBufferData(GL_ARRAY_BUFFER, _sprites_max * 5 * sizeof(GLfloat), NULL, GL_STREAM_DRAW);

	glGenBuffers(1, &_color_buffer);
	glBindBuffer(GL_ARRAY_BUFFER, _color_buffer);
	glBufferData(GL_ARRAY_BUFFER, _sprites_max * 4 * sizeof(GLubyte), NULL, GL_STREAM_DRAW);

	glEnable(GL_BLEND);
	glBlendFunc(GL_SRC_ALPHA, GL_ONE_MINUS_SRC_ALPHA);

	_need_update = 1;
}

void render(void)
{
	if(_need_update){
		update_buffers();
		_need_update = 0;
	}

	glViewport(0, 0, cc_get_window_width(), cc_get_window_height());

	glClear(GL_COLOR_BUFFER_BIT);
	glClearColor(1.0, 1.0, 1.0, 1.0);

	glBindVertexArray(_vertex_array);
	glUseProgram(_program);

	glActiveTexture(GL_TEXTURE0);
	glBindTexture(GL_TEXTURE_2D, _texture);
	glUniform1i(_texture_location, 0);

	glDrawArraysInstanced(GL_TRIANGLE_STRIP, 0, 4, _sprites_total);

	cc_swap_opengl_buffers();
}

texture_t load_vector_from_dfield(const char *file)
{
	unsigned char texture_width, texture_height;
	char *texture_raw;
	FILE *fd;
	size_t bytes;

	/* Open texture */
	fd = fopen(file, "rb");
	if(!fd){
		cc_set_error("Could not open %s", file);
	}
	fseek(fd, 0, SEEK_END);
  bytes = ftell(fd);
  rewind(fd);

	fread(&texture_width, sizeof(unsigned char), 1, fd);
	fread(&texture_height, sizeof(unsigned char), 1, fd);

	if(bytes != (unsigned long)(texture_width * texture_height) + 2){
		cc_set_error("File size not correct");
	}

	texture_raw = (char*)malloc(bytes - 2);
	if(!texture_raw){
		cc_out_of_memory_error();
	}

	bytes = fread(texture_raw, sizeof(char), bytes - 2, fd);
	if(bytes != (unsigned long)(texture_width * texture_height)){
		cc_set_error("Error reading bytes from file, could only read %lu bytes", bytes);
	}
	fclose(fd);

	glGenTextures(1, &_texture);
	glBindTexture(GL_TEXTURE_2D, _texture);
	glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_MIN_FILTER, GL_LINEAR);
	glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_MAG_FILTER, GL_LINEAR);
	glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_WRAP_S, GL_CLAMP_TO_EDGE);
	glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_WRAP_T, GL_CLAMP_TO_EDGE);
	glTexImage2D(GL_TEXTURE_2D, 0, GL_RED, texture_width, texture_height, 0, GL_RED, GL_UNSIGNED_BYTE, texture_raw);
	free(texture_raw);
	glBindTexture(GL_TEXTURE_2D, 0);

	return _texture;
}

sprite_t spawn_sprite(texture_t texture, unsigned char r, unsigned char g, unsigned char b, unsigned char a)
{
	sprite_t current;
	int color_index;

	current = _sprites_total;
	_sprites_total++;

	move_sprite(current, 0.0f, 0.0f);
	scale_sprite(current, 1.0f, 1.0f);
	rotate_sprite(current, 0.0f);

	color_index = current << 2;

	_color_data[color_index + 0] = r;
	_color_data[color_index + 1] = g;
	_color_data[color_index + 2] = b;
	_color_data[color_index + 3] = a;

	return current;
}

void move_sprite(sprite_t sprite, float x, float y)
{
	const int data_index = sprite * 5;

	_data[data_index + 0] = x;
	_data[data_index + 1] = y;

	_need_update = 1;
}

void scale_sprite(sprite_t sprite, float width, float height)
{
	const int data_index = sprite * 5;

	_data[data_index + 2] = width;
	_data[data_index + 3] = height;

	_need_update = 1;
}

void rotate_sprite(sprite_t sprite, float rotation)
{
	const int data_index = sprite * 5;

	_data[data_index + 4] = rotation;

	_need_update = 1;
}
