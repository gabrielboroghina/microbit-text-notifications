#include "tock.h"
#include "led_matrix_text.h"

static void request_done(int status, int len, int unused, void* ud) {
    bool* done = (bool*) ud;
    *done = true;
}

/**
 * Display a string on the LED matrix letter by letter.
 * 
 * @param text String to be displayed
 * @param char_delay Delay between each two consecutive letters
 * @param mode Display mode: 
 *     0 - show the text a single time and stop the LEDs
 *     1 - continuously repeat the text
 */
void display_text(const char* text, int mode, int char_delay)
{
    bool done = false;

    allow_ro_return_t ret_allow = allow_readonly(DRIVER_NUM_LED_TEXT, 0, text, strlen(text));
    if (ret_allow.status == TOCK_STATUSCODE_SUCCESS) 
    {
        subscribe_return_t ret_subscribe = subscribe(DRIVER_NUM_LED_TEXT, 0, request_done, &done);
        if (ret_subscribe.status == TOCK_STATUSCODE_SUCCESS) 
        {
            syscall_return_t sys = command(DRIVER_NUM_LED_TEXT, mode == none ? 2 : 1, char_delay, mode);
            if (mode != repeat && sys.type == TOCK_SYSCALL_SUCCESS)
            {
                yield_for(&done);
            }
        }
    }

    if (mode != repeat) {
        // unallow address
        allow_readonly(DRIVER_NUM_LED_TEXT, 0, NULL, 0);
    }
}