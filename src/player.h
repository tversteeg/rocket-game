#pragma once

#define PLAYER_ACCELERATION 0.0001f
#define PLAYER_ACCELERATION_DAMPING 0.7f
#define PLAYER_ROTATION 0.05f

void update_player(void);
void player_handle_keyboard_press(int key_code);
void player_handle_keyboard_release(int key_code);

float get_player_x(void);
float get_player_y(void);
float get_player_rotation(void);
