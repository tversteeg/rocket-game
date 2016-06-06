#pragma once

#ifdef USE_EPOXY
#include <epoxy/gl.h>
#elif defined USE_GLEW
#include <GL/glew.h>
#else
#define GL_GLEXT_PROTOTYPES
#include <GL/gl.h>
#include <GL/glext.h>
#endif

typedef unsigned long sprite_t;
typedef GLuint texture_t;

void init_opengl(const char *fragment_file, const char *vertex_file, int max_sprites);
void render(void);

texture_t load_vector_from_dfield(const char *file);

sprite_t spawn_sprite(texture_t texture, unsigned char r, unsigned char g, unsigned char b, unsigned char a);
void move_sprite(sprite_t sprite, float x, float y);
void rotate_sprite(sprite_t sprite, float rotation);
void scale_sprite(sprite_t sprite, float width, float height);
