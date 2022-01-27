#pragma once

#include "tock.h"

#define DRIVER_NUM_LED_TEXT 0xa0000

void display_text(const char* text, int char_delay);
