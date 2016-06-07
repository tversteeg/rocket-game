#include <ccore.h>

#include <stdio.h>
#include <stdlib.h>

#include "render.h"
#include "player.h"

void error_handler(const char *message)
{
	fprintf(stderr, "Error: \"%s\"\n", message);
	exit(EXIT_FAILURE);
}

int main(int argc, char** argv)
{
	struct cc_event event;
	sprite_t spaceship;
	unsigned long last_time, current_time;
	char fps_title[256];
	int frames;

	cc_set_error_handler(error_handler);

	cc_new_window(0);

	cc_set_window_size(800, 600);
	cc_set_window_title("Small Game");

	init_opengl("assets/fragment.shader", "assets/vertex.shader", 1024);

	spaceship = spawn_sprite(load_vector_from_dfield("assets/spaceship.dfield"), 0, 0, 0, 255);

	scale_sprite(spaceship, 0.1f, 0.1f);

	frames = 0;
	last_time = cc_get_time_nano_seconds();

	while(cc_poll_window()){
		while(cc_pop_event(&event)){
			if(event.type == CC_EVENT_DRAW){
				move_sprite(spaceship, get_player_x(), get_player_y());
				rotate_sprite(spaceship, get_player_rotation());
				render();
			}else if(event.type == CC_EVENT_PRESS_KEY){
				player_handle_keyboard_press(event.data.key_code);
			}else if(event.type == CC_EVENT_RELEASE_KEY){
				player_handle_keyboard_release(event.data.key_code);
			}
		}

		update_player();

		current_time = cc_get_time_nano_seconds();
		frames++;
		if(current_time - last_time >= (unsigned long)1.0e9){
			sprintf(fps_title, "Small Game: %d fps", frames);
			cc_set_window_title(fps_title);

			frames = 0;
			last_time += (unsigned long)1.0e09;
		}

		cc_sleep_micro_seconds(5000);
	}

	cc_destroy_opengl_context();
	cc_destroy_window();

	return EXIT_SUCCESS;
}
