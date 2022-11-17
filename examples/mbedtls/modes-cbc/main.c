#include "mbedtls/aes.h"
#include "print.h"

#define LENGTH 48

void raw_aes_ctr_mode(mbedtls_aes_context* enc_ctx, mbedtls_aes_context* dec_ctx, unsigned char* iv)
{
    unsigned char plaintext[LENGTH];
    unsigned char enc_out[LENGTH];
    unsigned char dec_out[LENGTH];

    memset(plaintext, 0xff, LENGTH);
    for (int i = 0; i < LENGTH; i++) {
        plaintext[i] = rand() % 256;
    }

    size_t nc_off_enc = 0;
    size_t nc_off_dec = 0;
    unsigned char iv_enc[16];
    unsigned char iv_dec[16];
    unsigned char stream_block_enc[16];
    unsigned char stream_block_dec[16];
    memcpy(iv_enc, iv, 16);
    memcpy(iv_dec, iv, 16);
    memset(stream_block_enc, 0, 16);
    memset(stream_block_dec, 0, 16);

    mbedtls_aes_crypt_ctr(enc_ctx, LENGTH, &nc_off_enc, iv_enc, stream_block_enc, plaintext, enc_out);
    mbedtls_aes_crypt_ctr(enc_ctx, LENGTH, &nc_off_dec, iv_dec, stream_block_dec, enc_out, dec_out);

    printd(plaintext, LENGTH);
    printd(enc_out, LENGTH);
    printd(dec_out, LENGTH);
    printf("\n");
}

void raw_aes_cbc_mode(mbedtls_aes_context* enc_ctx, mbedtls_aes_context* dec_ctx, unsigned char* iv)
{
    unsigned char plaintext[LENGTH];
    unsigned char enc_out[LENGTH];
    unsigned char dec_out[LENGTH];
    unsigned char iv_enc[16];
    unsigned char iv_dec[16];
    memcpy(iv_enc, iv, 16);
    memcpy(iv_dec, iv, 16);

    memset(plaintext, 0xff, LENGTH);
    for (int i = 0; i < LENGTH; i++) {
        plaintext[i] = rand() % 256;
    }

    mbedtls_aes_crypt_cbc(enc_ctx, MBEDTLS_AES_ENCRYPT, LENGTH, iv_enc, plaintext, enc_out);
    mbedtls_aes_crypt_cbc(dec_ctx, MBEDTLS_AES_DECRYPT, LENGTH, iv_dec, enc_out, dec_out);

    printd(plaintext, LENGTH);
    printd(enc_out, LENGTH);
    printd(dec_out, LENGTH);
    printf("\n");
}

int main()
{
    mbedtls_aes_context enc_ctx;
    mbedtls_aes_context dec_ctx;
    unsigned char key[] = { 0x00, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99, 0xaa, 0xbb, 0xcc, 0xdd, 0xee, 0xff };
    unsigned char iv[] = { 0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f };

    mbedtls_aes_setkey_enc(&enc_ctx, key, 128);
    mbedtls_aes_setkey_dec(&dec_ctx, key, 128);

    for (int i = 0; i < 1000; i++) {
        raw_aes_cbc_mode(&enc_ctx, &dec_ctx, iv);
        // raw_aes_ctr_mode(&enc_ctx, &enc_ctx, iv);
    }
}
