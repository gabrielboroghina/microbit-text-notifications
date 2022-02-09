/* vim: set sw=2 expandtab tw=80: */

#include <stdio.h>
#include <stdlib.h>
#include <unistd.h>
#include <timer.h>
#include <string.h>
#include "tock.h"
#include "network.h"
#include "button.h"
#include "led_matrix_text.h"
#include <buzzer.h>
#include "../config.h"

static void print_formatted_text(char *text)
{
    printf("\n\nAPI response:");
    printf("\n------------------------------------------------------------\n");
    printf("%s", text);
    printf("\n------------------------------------------------------------\n");
}

static char *substract_notification_name(char **data)
{
    if (*data != NULL)
    {
        char *ptr = *data;

        ptr = strstr(ptr, "\"name\"");
        if (ptr == NULL)
        {
            return NULL;
        }

        ptr = strchr(ptr, ':');
        if (ptr == NULL)
        {
            return NULL;
        }

        ptr += 2;
        char *end_ptr = ptr;
        end_ptr = strchr(end_ptr, ',');
        end_ptr -= 2;
        int len = (int)(end_ptr - ptr) + 1;

        char *name = (char *)calloc(len + 1, sizeof(char));
        strncpy(name, ptr, len);
        return name;
    }

    return NULL;
}

static void notify()
{

    if (!buzzer_exists())
    {
        printf("There is no available buzzer\n");
        return;
    }

    // Notes in the form of (note_frequency, note_delay in musical terms)
    static int notification_bip[] = {
        NOTE_G4, 8, NOTE_C4, 8, NOTE_DS4, 16, NOTE_F4, 16, NOTE_G4, 8, NOTE_C4, 8};

    static int TEMPO = 95;

    int notes = sizeof(notification_bip) / sizeof(notification_bip[0]) / 2;
    int wholenote = (60000 * 4) / TEMPO;
    for (int note = 0; note < notes * 2; note = note + 2)
    {
        // calculates the duration of each note
        int divider = notification_bip[note + 1];
        int note_duration = 0;
        if (divider > 0)
        {
            // regular note, just proceed
            note_duration = (wholenote) / divider;
        }
        else if (divider < 0)
        {
            // dotted notes are represented with negative durations!!
            note_duration = (wholenote) / abs(divider);
            note_duration *= 1.5; // increases the duration in half for dotted notes
        }

        // we only play the note for 90% of the duration, leaving 10% as a pause
        tone_sync(notification_bip[note] * 3, note_duration * 0.9);
    }
}

static void get_notifications()
{
    do
    {
        int status;
        char *data = network_get(API_ENDPOINT "/api/notifications", &status);
        char *name = substract_notification_name(&data);

        if (name != NULL)
        {
            print_formatted_text(name);
            notify();
            display_text(name, single, 400);
            free(data);
        }
        else
        {
            printf("No new notification\n");
        }

        delay_ms(40000);
    } while (true);
}

int main(void)
{
    if (driver_exists(DRIVER_NUM_NETWORK) && driver_exists(DRIVER_NUM_LED_TEXT))
    {
        get_notifications();
    }
    else
    {
        printf("No network or led matrix driver\n");
    }
    return 0;
}