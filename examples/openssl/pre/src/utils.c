#include <openssl/bio.h>
#include <stdio.h>

void printd(unsigned char* src, int len)
{
    for (int i = 0; i < (len / 16) + (len % 16 != 0); i++) {
        for (int j = 0; j < 16; j++) {
            printf("%02x", src[i * 16 + j]);
        }
        printf("\n");
    }
}

void print_result(unsigned char* plaintext, unsigned char* enc_out, unsigned char* dec_out, int length)
{
    printf("plaintext:\n");
    BIO_dump_fp(stdout, (const char*)plaintext, length);
    printf("\nencrypted:\n");
    BIO_dump_fp(stdout, (const char*)enc_out, length);
    printf("\ndecrypted:\n");
    BIO_dump_fp(stdout, (const char*)dec_out, length);
    printf("\n");
}