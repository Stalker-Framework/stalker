#include <gcrypt.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define MSG_SIZE 48

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
    gcry_error_t err = gcry_cipher_open(hd, GCRY_CIPHER_AES128, GCRY_CIPHER_MODE_CTR, 0);
    if (err) {
        error("gcry_cipher_open", err);
    }

    err = gcry_cipher_setkey(*hd, config.key, 16);
    if (err) {
        error("gcry_cipher_setkey", err);
    }

    err = gcry_cipher_setiv(*hd, config.nonce, 16);
    if (err) {
        error("gcry_cipher_setiv", err);
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

void raw_aes_128(Config config)
{
    unsigned char plaintext[MSG_SIZE];

    memset(plaintext, 0xff, MSG_SIZE);
    for (int i = 0; i < MSG_SIZE; i++) {
        plaintext[i] = rand() % 256;
    }

    void* tag;
    void* enc_out = encrypt(config, plaintext, &tag);
    void* dec_out = decrypt(config, enc_out, tag);

    printd(plaintext, MSG_SIZE);
    printd(enc_out, MSG_SIZE);
    printd(dec_out, MSG_SIZE);
    printf("\n");

    free(enc_out);
    free(dec_out);
}

int main()
{
    unsigned char key[] = { 0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f };
    unsigned char nonce[] = { 0x00, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99, 0xaa, 0xbb, 0xcc, 0xdd, 0xee, 0xff };

    Config config;
    config.nonce = nonce;
    config.key = key;
    config.messageSize = MSG_SIZE;
    config.authTagLength = 0;

    for (int i = 0; i < 1000; i++) {
        raw_aes_128(config);
    }
    return 0;
}
