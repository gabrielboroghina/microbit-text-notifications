#include <stdlib.h>
#include "tock.h"
#include "network.h"

typedef struct response {
    bool done;
    int status;
} response;

static bool processing = false; // Network processing lock

static void request_done(int status, int len, int unused, void* ud) {
    response* res = (response*) ud;
    res->done = true;
    res->status = status;
}

char* network_get(const char *url, int *status) {
    if (processing)
        return NULL;
    processing = true;

    *status = 0;
    response res = { false, 0 };

    // Allocate response buffer
    const size_t res_buf_size = 1024;
    char* data_buffer = (char*) calloc(res_buf_size, sizeof(char));
    if (data_buffer == NULL) {
        goto cleanup;
    }

    allow_rw_return_t ret_allow_buffer = allow_readwrite(DRIVER_NUM_NETWORK, 0, data_buffer, res_buf_size);
    if (ret_allow_buffer.status != TOCK_STATUSCODE_SUCCESS) {
        goto cleanup;
    }

    allow_ro_return_t ret_allow = allow_readonly(DRIVER_NUM_NETWORK, 0, url, strlen(url));
    if (ret_allow.status != TOCK_STATUSCODE_SUCCESS) {
        goto cleanup;
    }

    subscribe_return_t ret_subscribe = subscribe(DRIVER_NUM_NETWORK, 0, request_done, &res);
    if (ret_subscribe.status != TOCK_STATUSCODE_SUCCESS) {
        goto cleanup;
    }

    syscall_return_t sys = command(DRIVER_NUM_NETWORK, 1, 0, 0);
    if (sys.type == TOCK_SYSCALL_SUCCESS) {
        yield_for(&res.done);
        *status = res.status;
        if (*status != 0) {
            goto cleanup;
        }
    }
    goto end;

cleanup:
    free(data_buffer);
    data_buffer = NULL;

end:
    allow_readonly(DRIVER_NUM_NETWORK, 0, NULL, 0); // unallow address
    allow_readwrite(DRIVER_NUM_NETWORK, 0, NULL, 0); // unallow buffer

    processing = false;
    return data_buffer;
}

void network_post(const char* url, const char* payload) {
    if (processing)
        return;
    processing = true;

    response res = { false, 0 };

    // Allocate response buffer
    const size_t res_buf_size = 1024;
    char* data_buffer = (char*) calloc(res_buf_size, sizeof(char));
    if (!data_buffer) {
        goto cleanup;
    }

    allow_rw_return_t ret_allow_buffer = allow_readwrite(DRIVER_NUM_NETWORK, 0, data_buffer, res_buf_size);
    if (ret_allow_buffer.status != TOCK_STATUSCODE_SUCCESS) {
        goto cleanup;
    }

    allow_ro_return_t ret_allow_address = allow_readonly(DRIVER_NUM_NETWORK, 0, url, strlen(url));
    if (ret_allow_address.status != TOCK_STATUSCODE_SUCCESS) {
        goto cleanup;
    }
        
    allow_ro_return_t ret_allow_payload = allow_readonly(DRIVER_NUM_NETWORK, 1, payload, strlen(payload));
    if (ret_allow_payload.status != TOCK_STATUSCODE_SUCCESS) {
        goto cleanup;
    }

    subscribe_return_t ret_subscribe = subscribe(DRIVER_NUM_NETWORK, 0, request_done, &res);
    if (ret_subscribe.status != TOCK_STATUSCODE_SUCCESS) {
        goto cleanup;
    }

    syscall_return_t sys = command(DRIVER_NUM_NETWORK, 1, 0, 0);
    if (sys.type == TOCK_SYSCALL_SUCCESS) {
        yield_for(&res.done);
        goto cleanup;
    }

cleanup:
    allow_readonly(DRIVER_NUM_NETWORK, 1, NULL, 0); // unallow payload
    allow_readonly(DRIVER_NUM_NETWORK, 0, NULL, 0); // unallow address
    allow_readwrite(DRIVER_NUM_NETWORK, 0, NULL, 0); // unallow response buffer

    free(data_buffer);
    processing = false;
}