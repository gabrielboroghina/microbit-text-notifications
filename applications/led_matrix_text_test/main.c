/* vim: set sw=2 expandtab tw=80: */

#include <stdio.h>
#include <stdlib.h>
#include "tock.h"
#include "button.h"
#include "led_matrix_text.h"

// Callback for button presses.
//   btn_num: The index of the button associated with the callback
//   val: 1 if pressed, 0 if depressed
static void button_callback(int btn_num, int val, int arg2, void *ud)
{
  if (val == 1)
  {
    if (btn_num == 0)
    {
      display_text("", none, 0);
      // TODO decrease snooze value
    }
    else 
    {
      display_text("", none, 0);
      // TODO increase snooze value
    }
  }
}

int main(void) {
  if (driver_exists(DRIVER_NUM_LED_TEXT)) {
    // Register actions on buttons
    button_subscribe(button_callback, NULL);
    button_enable_interrupt(0);
    button_enable_interrupt(1);

    printf("Start display\n");
    display_text("1s^", repeat, 500);
    printf("Returned in userspace\n");
  }
  else
  {
    printf("No network driver\n");
  }
  return 0;
}