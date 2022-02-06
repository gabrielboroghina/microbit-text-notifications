#pragma once

#define DRIVER_NUM_NETWORK 0xa0001

char * network_get(const char *url, int *status);
void network_post(const char *url, const char *payload);