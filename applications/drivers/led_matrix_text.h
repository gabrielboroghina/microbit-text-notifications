#pragma once

#include "tock.h"

#define DRIVER_NUM_LED_TEXT 0xa0000

enum text_mode {
    single,
    repeat,
    none,
};

void display_text(const char* text, int mode, int char_delay);
