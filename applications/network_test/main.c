/* vim: set sw=2 expandtab tw=80: */

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include "tock.h"
#include "network.h"
#include "button.h"

void print_formatted_text(char* text) {
  printf("\n\nAPI response:");
  printf("\n------------------------------------------------------------\n");
  printf("%s", text);
  printf("\n------------------------------------------------------------\n");
}

// Callback for button presses.
//   btn_num: The index of the button associated with the callback
//   val: 1 if pressed, 0 if depressed
static void button_callback(int btn_num, int val, int arg2, void *ud)
{
  if (val == 1)
  {
    if (btn_num == 0)
    {
      // Button A: perform a GET request
      char* data = network_get("http://www.google.com/");
      if (data != NULL)
      {
        print_formatted_text(data);
        free(data);
      }
      else
      {
        printf("No response\n");
      }
    }
    else
    {
      // Button B: perform a POST request
      char* body_buffer = (char*) calloc(1024, sizeof(char));
      strcpy(body_buffer, "test body");
      network_post("http://www.google.com/", body_buffer);
    }
  }
}

int main(void) {
  if (driver_exists(DRIVER_NUM_NETWORK)) {
    // Register actions on buttons
    button_subscribe(button_callback, NULL);
    button_enable_interrupt(0);
    button_enable_interrupt(1);
  }
  else
  {
    printf("No network driver\n");
  }
  return 0;
}