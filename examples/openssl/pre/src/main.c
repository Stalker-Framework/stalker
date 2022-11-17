#include "evp_exps.h"
#include "raw_exps.h"
#include "utils.h"
#include <string.h>

static unsigned char key[] = { 0x00, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99, 0xaa, 0xbb, 0xcc, 0xdd, 0xee, 0xff };

static unsigned char iv[] = { 0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f };

int main(int argc, char* argv[])
{
    if (argc != 2) {
        return 1;
    }

    if (!strcmp(argv[1], "ev0.1")) {
        evp_aes_128_cbc(key, iv);
    }
    if (!strcmp(argv[1], "ev0.3.1")) {
        cbc_mode(key, iv);
        cbc_mode(key, key); // Change IV
        // evp_aes_128_cbc(key, iv); // panic!
    }
    if (!strcmp(argv[1], "ev0.3.1-decrypt")) {
        cbc_mode(key, iv);
        // evp_aes_128_cbc(key, iv); // panic!
    }
    if (!strcmp(argv[1], "ev0.3.2") || !strcmp(argv[1], "ev0.3.3")) {
        evp_aes_128_ctr(key, iv);
    }

    if (!strcmp(argv[1], "batch")) {
        evp_aes_128_ctr(key, iv);
    }

    return 0;
}