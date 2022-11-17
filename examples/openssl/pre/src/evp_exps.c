#include "evp_exps.h"
#include "utils.h"
#include <openssl/conf.h>
#include <openssl/err.h>
#include <openssl/evp.h>
#include <string.h>

/* 
* Mainly adopted from <https://wiki.openssl.org/index.php/EVP_Symmetric_Encryption_and_Decryption>.
*/

static const unsigned char skey[] = { 0x00, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99, 0xaa, 0xbb, 0xcc, 0xdd, 0xee, 0xff };

static const unsigned char siv[] = { 0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f };

#define use_evp(cipher)                                                            \
    void evp_##cipher(unsigned char* key, unsigned char* iv)                       \
    {                                                                              \
        /* Message to be encrypted */                                              \
        unsigned char plaintext[] = "The quick brown fox jumps over the lazy dog"; \
                                                                                   \
        /*                                                                         \
         * Buffer for ciphertext. Ensure the buffer is long enough for the         \
         * ciphertext which may be longer than the plaintext, depending on the     \
         * algorithm and mode.                                                     \
         */                                                                        \
        unsigned char enc_out[128];                                                \
                                                                                   \
        /* Buffer for the decrypted text */                                        \
        unsigned char dec_out[128];                                                \
                                                                                   \
        int dec_len, enc_len;                                                      \
                                                                                   \
        /* Encrypt the plaintext */                                                \
        enc_len = encrypt_##cipher(plaintext, strlen((char*)plaintext), key, iv,   \
            enc_out);                                                              \
                                                                                   \
        /* Decrypt the ciphertext */                                               \
        dec_len = decrypt_##cipher(enc_out, enc_len, key, iv,                      \
            dec_out);                                                              \
                                                                                   \
        print_result(plaintext, enc_out, dec_out, enc_len);                        \
                                                                                   \
        return;                                                                    \
    }

#define decrypt_evp(cipher)                                                          \
    int decrypt_##cipher(unsigned char* ciphertext, int enc_len, unsigned char* key, \
        unsigned char* iv, unsigned char* plaintext)                                 \
    {                                                                                \
        EVP_CIPHER_CTX* ctx;                                                         \
                                                                                     \
        int len;                                                                     \
                                                                                     \
        int plaintext_len;                                                           \
                                                                                     \
        /* Create and initialise the context */                                      \
        if (!(ctx = EVP_CIPHER_CTX_new()))                                           \
            handleErrors();                                                          \
                                                                                     \
        /*                                                                           \
         * Initialise the decryption operation. IMPORTANT - ensure you use a key     \
         * and IV size appropriate for your cipher                                   \
         * In this example we are using 128 bit AES (i.e. a 128 bit key). The        \
         * IV size for *most* modes is the same as the block size. For AES this      \
         * is 128 bits                                                               \
         */                                                                          \
        if (1 != EVP_DecryptInit_ex(ctx, EVP_##cipher(), NULL, key, iv))             \
            handleErrors();                                                          \
                                                                                     \
        /*                                                                           \
         * Provide the message to be decrypted, and obtain the plaintext output.     \
         * EVP_DecryptUpdate can be called multiple times if necessary.              \
         */                                                                          \
        if (1 != EVP_DecryptUpdate(ctx, plaintext, &len, ciphertext, enc_len))       \
            handleErrors();                                                          \
        plaintext_len = len;                                                         \
                                                                                     \
        /*                                                                           \
         * Finalise the decryption. Further plaintext bytes may be written at        \
         * this stage.                                                               \
         */                                                                          \
        if (1 != EVP_DecryptFinal_ex(ctx, plaintext + len, &len))                    \
            handleErrors();                                                          \
        plaintext_len += len;                                                        \
                                                                                     \
        /* Clean up */                                                               \
        EVP_CIPHER_CTX_free(ctx);                                                    \
                                                                                     \
        return plaintext_len;                                                        \
    }

#define encrypt_evp(cipher)                                                               \
    int encrypt_##cipher(unsigned char* plaintext, int plaintext_len, unsigned char* key, \
        unsigned char* iv, unsigned char* ciphertext)                                     \
    {                                                                                     \
        EVP_CIPHER_CTX* ctx;                                                              \
                                                                                          \
        int len;                                                                          \
                                                                                          \
        int enc_len;                                                                      \
                                                                                          \
        /* Create and initialise the context */                                           \
        if (!(ctx = EVP_CIPHER_CTX_new()))                                                \
            handleErrors();                                                               \
                                                                                          \
        /*                                                                                \
         * Initialise the encryption operation. IMPORTANT - ensure you use a key          \
         * and IV size appropriate for your cipher                                        \
         * In this example we are using 128 bit AES (i.e. a 128 bit key). The             \
         * IV size for *most* modes is the same as the block size. For AES this           \
         * is 128 bits                                                                    \
         */                                                                               \
        if (1 != EVP_EncryptInit_ex(ctx, EVP_##cipher(), NULL, key, iv))                  \
            handleErrors();                                                               \
                                                                                          \
        /*                                                                                \
         * Provide the message to be encrypted, and obtain the encrypted output.          \
         * EVP_EncryptUpdate can be called multiple times if necessary                    \
         */                                                                               \
        if (1 != EVP_EncryptUpdate(ctx, ciphertext, &len, plaintext, plaintext_len))      \
            handleErrors();                                                               \
        enc_len = len;                                                                    \
                                                                                          \
        /*                                                                                \
         * Finalise the encryption. Further ciphertext bytes may be written at            \
         * this stage.                                                                    \
         */                                                                               \
        if (1 != EVP_EncryptFinal_ex(ctx, ciphertext + len, &len))                        \
            handleErrors();                                                               \
        enc_len += len;                                                                   \
                                                                                          \
        /* Clean up */                                                                    \
        EVP_CIPHER_CTX_free(ctx);                                                         \
                                                                                          \
        return enc_len;                                                                   \
    }

#define impl_evp(cipher) \
    use_evp(cipher);     \
    encrypt_evp(cipher); \
    decrypt_evp(cipher);

impl_evp(aes_128_cbc);
impl_evp(aes_128_ctr);

void handleErrors(void)
{
    ERR_print_errors_fp(stderr);
    abort();
}