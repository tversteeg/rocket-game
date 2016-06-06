#include <ccore.h>

#include <stdio.h>
#include <stdlib.h>

#include "render.h"

void error_handler(const char *message)
{
	fprintf(stderr, "Error: \"%s\"\n", message);
	exit(EXIT_FAILURE);
}

int main(int argc, char** argv)
{
	struct cc_event event;
	sprite_t spaceship;
	float rotation;

	cc_set_error_handler(error_handler);

	cc_new_window(0);

	cc_set_window_size(800, 600);
	cc_set_window_title("Small Game");

	init_opengl("assets/fragment.shader", "assets/vertex.shader", 1024);

	spaceship = spawn_sprite(load_vector_from_dfield("assets/spaceship.dfield"), 0, 0, 0, 255);

	scale_sprite(spaceship, 0.1f, 0.1f);
	move_sprite(spaceship, 0.5f, 0.1f);

	rotation = 0;

	while(cc_poll_window()){
		event = cc_pop_event();
		if(event.type == CC_EVENT_DRAW){
			rotation += 0.1f;
			rotate_sprite(spaceship, rotation);
			render();
		}
	}

	cc_destroy_opengl_context();
	cc_destroy_window();

	return EXIT_SUCCESS;
}
