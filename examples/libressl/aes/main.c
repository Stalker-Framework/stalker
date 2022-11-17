#include <stdio.h>
#include <string.h>
#include <openssl/aes.h>

static unsigned char key[] = { 0x00, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99, 0xaa, 0xbb, 0xcc, 0xdd, 0xee, 0xff };

static unsigned char iv[] = { 0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f };

void raw_aes_128_single(unsigned char* key)
{
    unsigned char plaintext[16];
    unsigned char enc_out[16];
    unsigned char dec_out[16];

    memset(plaintext, 0xff, 16);
    for (int i = 0; i < 16; i++) {
        plaintext[i] = rand() % 256;
    }

    AES_KEY enc_key, dec_key;

    AES_set_encrypt_key(key, 128, &enc_key);
    AES_encrypt(plaintext, enc_out, &enc_key);

    AES_set_decrypt_key(key, 128, &dec_key);
    AES_decrypt(enc_out, dec_out, &dec_key);

    printd(plaintext, 16);
    printd(enc_out, 16);
    printd(dec_out, 16);
    printf("\n");
}

int main(int argc, char* argv[])
{
    for (int i = 0; i < 1000; i++) {
        raw_aes_128_single(key);
    }
    return 0;
}