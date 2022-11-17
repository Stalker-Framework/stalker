#include "evp_exps.h"
#include "utils.h"
#include <openssl/aes.h>
#include <stdio.h>
#include <string.h>

static const unsigned char skey[] = { 0x00, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99, 0xaa, 0xbb, 0xcc, 0xdd, 0xee, 0xff };

static const unsigned char siv[] = { 0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f };

void raw_aes(unsigned char* key)
{
    unsigned char plaintext[16];
    unsigned char enc_out[16];
    unsigned char dec_out[16];

    memset(plaintext, 0xff, 16);

    AES_KEY enc_key, dec_key;

    AES_set_encrypt_key(key, 128, &enc_key);
    AES_encrypt(plaintext, enc_out, &enc_key);

    AES_set_decrypt_key(key, 128, &dec_key);
    AES_decrypt(enc_out, dec_out, &dec_key);

    print_result(plaintext, enc_out, dec_out, 16);
}

void cbc_mode(unsigned char* key, unsigned char* iv)
{
    unsigned char plaintext[64] = { 0 };
    unsigned char enc_out[64];
    unsigned char dec_out[64];

    unsigned char text[] = "The quick brown fox jumps over the lazy dog";
    strcpy(plaintext, text);
    memset(enc_out, 0x00, 64);
    memset(dec_out, 0x00, 64);

    AES_KEY enc_key, dec_key;

    AES_set_encrypt_key(key, 128, &enc_key);
    AES_cbc_encrypt(plaintext, enc_out, 64, &enc_key, iv, 1);

    memcpy(iv, siv, 16);
    AES_set_decrypt_key(key, 128, &dec_key);
    AES_cbc_encrypt(enc_out, dec_out, 64, &dec_key, iv, 0);

    print_result(plaintext, enc_out, dec_out, 64);
}
