#include "mbedtls/aes.h"
#include "print.h"

void raw_aes_128_single(mbedtls_aes_context *ctx)
{
    unsigned char plaintext[16];
    unsigned char enc_out[16];
    unsigned char dec_out[16];

    memset(plaintext, 0xff, 16);
    for (int i = 0; i < 16; i++)
    {
        plaintext[i] = rand() % 256;
    }

    mbedtls_aes_crypt_ecb(ctx, MBEDTLS_AES_ENCRYPT, plaintext, enc_out);
    mbedtls_aes_crypt_ecb(ctx, MBEDTLS_AES_DECRYPT, enc_out, dec_out);

    printd(plaintext, 16);
    printd(enc_out, 16);
    printd(dec_out, 16);
    printf("\n");
}

int main()
{
    mbedtls_aes_context ctx;
    const char *input = "0123456789abcdef";
    const char *key = "0123456789abcdef";
    mbedtls_aes_setkey_enc(&ctx, key, 128);

    for (int i = 0; i < 1000; i++)
    {
        raw_aes_128_single(&ctx);
    }
}