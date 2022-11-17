#include <gcrypt.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include "print.h"

void error(const char* what, gcry_error_t err)
{
    fprintf(stderr, "%s failed: %s\n", what, gcry_strerror(err));
    exit(1);
}

typedef struct {
    const void* key;
    const void* nonce;
    size_t messageSize;
    int authTagLength;
} Config;

void prepare(Config config, gcry_cipher_hd_t* hd)
{
    gcry_error_t err = gcry_cipher_open(hd, GCRY_CIPHER_AES128, GCRY_CIPHER_MODE_ECB, 0);
    if (err) {
        error("gcry_cipher_open", err);
    }

    err = gcry_cipher_setkey(*hd, config.key, strlen(config.key));
    if (err) {
        error("gcry_cipher_setkey", err);
    }

    if (strlen(config.nonce) > 0) {
        err = gcry_cipher_setiv(*hd, config.nonce, strlen(config.nonce));
        if (err) {
            error("gcry_cipher_setiv", err);
        }
    }

    uint64_t params[3];
    params[0] = config.messageSize;
    params[1] = 0;
    params[2] = config.authTagLength;
    if (params[2] > 0) {
        err = gcry_cipher_ctl(*hd, GCRYCTL_SET_CCM_LENGTHS, params, sizeof(params));
        if (err) {
            error("gcry_cipher_ctl", err);
        }
    }
}

void* encrypt(Config config, const void* plainText, void** tag)
{
    gcry_cipher_hd_t hdEncrypt;
    prepare(config, &hdEncrypt);

    void* cipherText = malloc(config.messageSize);
    gcry_error_t err = gcry_cipher_encrypt(hdEncrypt, cipherText, config.messageSize, plainText, config.messageSize);
    if (err) {
        error("gcry_cipher_encrypt", err);
    }

    if (config.authTagLength > 0) {
        *tag = malloc(config.authTagLength);
        err = gcry_cipher_gettag(hdEncrypt, *tag, config.authTagLength);
        if (err) {
            error("gcry_cipher_encrypt", err);
        }
    }

    gcry_cipher_close(hdEncrypt);

    return cipherText;
}

void* decrypt(Config config, void* cipherText, void* tag)
{
    gcry_cipher_hd_t hdDecrypt;
    prepare(config, &hdDecrypt);

    void* recoveredText = malloc(config.messageSize);
    gcry_error_t err = gcry_cipher_decrypt(hdDecrypt, recoveredText, config.messageSize, cipherText, config.messageSize);
    if (err) {
        error("gcry_cipher_decrypt", err);
    }

    if (config.authTagLength > 0) {
        err = gcry_cipher_checktag(hdDecrypt, tag, config.authTagLength);
        if (gpg_err_code(err) == GPG_ERR_CHECKSUM) {
            error("Authentication", err);
        }
    }

    gcry_cipher_close(hdDecrypt);
    return recoveredText;
}

void raw_aes_128_single(Config config)
{
    unsigned char plaintext[16];

    memset(plaintext, 0xff, 16);
    for (int i = 0; i < 16; i++) {
        plaintext[i] = rand() % 256;
    }

    void* tag;
    void* enc_out = encrypt(config, plaintext, &tag);
    void* dec_out = decrypt(config, enc_out, tag);

    printd(plaintext, 16);
    printd(enc_out, 16);
    printd(dec_out, 16);
    printf("\n");

    free(enc_out);
    free(dec_out);
}

int main()
{
    const char* plainText = "0123456789abcdef";

    Config config;
    config.key = "1234567890123456";
    config.nonce = "";
    config.messageSize = strlen(plainText);
    config.authTagLength = 0;

    for (int i = 0; i < 1000; i++) {
        raw_aes_128_single(config);
    }
    return 0;
}
