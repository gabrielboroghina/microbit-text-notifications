/* vim: set sw=2 expandtab tw=80: */

#include <stdio.h>
#include <stdlib.h>
#include "tock.h"
#include "button.h"
#include "network.h"
#include "led_matrix_text.h"

static int digits_number(int n) {
    int count = 0;
    do {
        n /= 10;
        ++count;
    } while (n != 0);

    return count;
}

int numerical_value = 0;
int intervals_index = 0;
char intervals[] = {'s', 'm', 'h', 'd', '\0'};
// The user will be able to cycle through values to choose the desired time interval.
// Firstly, he will commit the numerical value. Secondly, he'll commit the interval value(seconds, minutes, hours, days).
// After committing his choice for these values he will commit the final choice, making a post request to the server.
int commit_number = 0;

static void reset() {
    numerical_value = 0;
    intervals_index = 0;
    commit_number = 0;
    display_text("", none, 0);
}

// Callback for button presses.
//   btn_num: The index of the button associated with the callback
//   val: 1 if pressed, 0 if depressed
static void button_callback(int btn_num, int val, int arg2, void *ud)
{
    if (val == 1) {
        if (btn_num == 0 /* A button*/) {
            printf("A Button pressed\n");

            if (commit_number == 0) {
                numerical_value++;
            } else if (commit_number == 1) {
                if (intervals_index < 3) {
                    intervals_index++;
                }
            } 
        }

        if (btn_num == 1 /* B button*/) {
             printf("B Button pressed\n");

            if (commit_number == 0) {
                numerical_value--;
            } else if (commit_number == 1) {
                if (intervals_index > 0) {
                    intervals_index--;
                }
            }
        }

        // convert numerical value to string [buf]
        char snum[33];
        itoa(numerical_value, snum, 10);
        int len = digits_number(numerical_value);

        // construct final message (numerical value + interval + '^)
        char* pData = (char*)calloc(len + 3, sizeof(char));
        strncpy(pData, snum, len);
        pData[len] = intervals[intervals_index];
        pData[len+1] = '^';
        printf("%s\n", pData);

        display_text(pData, repeat, 900);

        if (btn_num == 2 /* Touch button*/) {
            printf("Touch Button pressed\n");
            if (commit_number == 2) {
                printf("Submitted to server\n");
                network_post("http://www.google.com/", pData);
                reset();
            } else {
                commit_number++;
            }
        }
    }
}

int main(void) {
    if (driver_exists(DRIVER_NUM_NETWORK) && driver_exists(DRIVER_NUM_LED_TEXT)) {
        // Register actions on buttons
        button_subscribe(button_callback, NULL);
        button_enable_interrupt(0);
        button_enable_interrupt(1);
        button_enable_interrupt(2);
    } else {
        printf("No led matrix driver or network driver\n");
    }
    
    return 0;
}
