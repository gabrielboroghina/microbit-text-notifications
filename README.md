# MicroBit ​Text notifications system​

## Team Members:

- Boroghina Gabriel
- Catrina Mihaela-Florentina
- Smărăndoiu Andrei

## Introduction

The idea of this project refers to a system that allows scheduling various notifications at a specific timepoint in the future​, similar to a calendar service​.
The user will receive notifications and will be able to snooze the notifications. Each received notification will be signaled using the buzzer. Also, the text of the notification will be displayed on the LED matrix.
Notifications are stored in a DB and can be managed through a web dashboard​.

## Drivers

The drivers specifically designed for this project:

### HTTP Driver

- kernel/drivers/src/network.rs

Sends GET/POST requests from Microbit using a serial proxy​ written in python.

### LED matrix text display Driver

- kernel/drivers/src/led_matrix_text.rs

Writes a string on the LEDs, one letter at a time with a small delay​.
It has two modes:

1. write the text one time
2. write the text repeatedly

## Applications

User space apps designed for this project:

### Notifications alerts​

User is notified through the MicroBit board​.
Notifications have a text description that will be displayed to the user on the LED matrix (letter-by-letter)​.
The notification of the user starts with an audio alert​.

### Notifications snooze​

The user can reschedule (snooze) a notification right after it was displayed​
Snooze time is selected using the MicroBit’s buttons & LED display​

## Server

TechStack​:

- ExpressJS REST API​
- MongoDB for CRUD Database​
- Bootstrap for UI​

Routes​:

- GET /notifications​
- POST /notifications {ID, body, timestamp}​
- PUT /notifications {ID, newBody, newTimestamp}​
- DEL /notifications {ID}​

API​ used by user-space apps:

- GET /api/notifications​ - used to receive all the notifications expiring in the current minute
- POST/api/snooze {snoozeInterval}​ - used to snooze the current notification with a snoozeInterval

## Architecture

![alt text](docs/img/Architecture.png?raw=true "Architecture")
![alt text](docs/img/Server.png?raw=true "Server")
