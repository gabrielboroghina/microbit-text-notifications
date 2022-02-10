/* vim: set sw=2 expandtab tw=80: */

#include <stdio.h>
#include <stdlib.h>
#include <timer.h>
#include "tock.h"
#include "button.h"
#include "network.h"
#include "led_matrix_text.h"
#include "../config.h"

static int numerical_value = 0;
static int intervals_index = 0;
static char intervals[] = {'s', 'm', 'h', 'd', '\0'};
// The user will be able to cycle through values to choose the desired time interval.
// Firstly, he will commit the numerical value. Secondly, he'll commit the interval value(seconds, minutes, hours, days).
// After committing his choice for these values he will commit the final choice, making a post request to the server.
static int commit_number = 0;

// Button callback processing lock
static bool processing = false;

// Buffer used for displaying data
static char pData[40]; // 33 for numerical value + paylod for additional string

static void reset()
{
    numerical_value = 0;
    intervals_index = 0;
    commit_number = 0;
    processing = false;
    display_text("", none, 0);
}

// Callback for button presses.
//   btn_num: The index of the button associated with the callback
//   val: 1 if pressed, 0 if depressed
static void button_callback(int btn_num, int val, int arg2, void *ud)
{
    if (processing)
    {
        return;
    }
    processing = true;

    if (val == 1)
    {

        if (btn_num == 0 /* A button*/)
        {
            printf("A Button pressed\n");
            if (commit_number == 0)
            {
                numerical_value++;
            }
            else if (commit_number == 1)
            {
                if (intervals_index < 3)
                {
                    intervals_index++;
                }
            }
        }

        if (btn_num == 1 /* B button*/)
        {
            printf("B Button pressed\n");
            if (commit_number == 0)
            {
                numerical_value--;
            }
            else if (commit_number == 1)
            {
                if (intervals_index > 0)
                {
                    intervals_index--;
                }
            }
        }

        // convert numerical value to string [pData]
        // compose displayed text
        sprintf(pData, "%d%c", numerical_value, intervals[intervals_index]);
        printf("%s\n", pData);

        display_text(pData, repeat, 700);

        if (btn_num == 2 /* Touch button*/)
        {
            printf("Touch Button pressed\n");
            if (commit_number == 1)
            {
                // snooze interval uses seconds
                int snooze_value = numerical_value;
                if (intervals[intervals_index] == 'm')
                {
                    snooze_value *= 60;
                }
                else if (intervals[intervals_index] == 'h')
                {
                    snooze_value *= 3600;
                }

                // compose request body
                char *body = (char *)calloc(33 + 15, sizeof(char));
                sprintf(body, "{\"snooze\":%d}", snooze_value);

                network_post(API_ENDPOINT "/api/snooze", body);
                reset();
                free(body);
            }
            else
            {
                commit_number++;
            }
        }
    }

    delay_ms(500);
    processing = false;
}

int main(void)
{
    if (driver_exists(DRIVER_NUM_NETWORK) && driver_exists(DRIVER_NUM_LED_TEXT))
    {
        // Register actions on buttons
        button_subscribe(button_callback, NULL);
        button_enable_interrupt(0);
        button_enable_interrupt(1);
        button_enable_interrupt(2);
    }
    else
    {
        printf("No led matrix driver or network driver\n");
    }

    return 0;
}
