/* vim: set sw=2 expandtab tw=80: */

#include <stdio.h>
#include <stdlib.h>
#include "tock.h"
#include "led_matrix_text.h"

int main(void) {
  if (driver_exists(DRIVER_NUM_LED_TEXT)) {
    display_text("ABCDEFGHIJKLMNOPQRSTUVWXYZ", 300);
  }
  else
  {
    printf("No network driver\n");
  }
  return 0;
}