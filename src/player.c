#include "player.h"

#include <ccore.h>
#include <math.h>

static float _x = 0;
static float _y = 0;
static float _vx = 0;
static float _vy = 0;
static float _acceleration = 0;
static float _rotation = 0;

static int _up_button, _down_button, _left_button, _right_button;

void update_player(void)
{
	if(_up_button){
		_acceleration += PLAYER_ACCELERATION;
	}
	if(_down_button){
		_acceleration -= PLAYER_ACCELERATION;
	}
	if(_left_button){
		_rotation -= PLAYER_ROTATION;
	}
	if(_right_button){
		_rotation += PLAYER_ROTATION;
	}

	_acceleration *= PLAYER_ACCELERATION_DAMPING;

	_vx += sin(_rotation) * _acceleration;
	_vy += cos(_rotation) * _acceleration;

	_x += _vx;
	_y += _vy;
}

void player_handle_keyboard_press(int key_code)
{
	if(key_code == CC_KEY_UP){
		_up_button = 1;
	}else if(key_code == CC_KEY_DOWN){
		_down_button = 1;
	}else if(key_code == CC_KEY_LEFT){
		_left_button = 1;
	}else if(key_code == CC_KEY_RIGHT){
		_right_button = 1;
	}
}

void player_handle_keyboard_release(int key_code)
{
	if(key_code == CC_KEY_UP){
		_up_button = 0;
	}else if(key_code == CC_KEY_DOWN){
		_down_button = 0;
	}else if(key_code == CC_KEY_LEFT){
		_left_button = 0;
	}else if(key_code == CC_KEY_RIGHT){
		_right_button = 0;
	}
}

float get_player_x(void)
{
	return _x;
}

float get_player_y(void)
{
	return _y;
}

float get_player_rotation(void)
{
	return _rotation;
}
