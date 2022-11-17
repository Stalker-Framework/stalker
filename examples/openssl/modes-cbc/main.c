#include <string.h>
#include <openssl/conf.h>
#include <openssl/err.h>
#include <openssl/evp.h>
#include <openssl/aes.h>

static unsigned char key[] = { 0x00, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99, 0xaa, 0xbb, 0xcc, 0xdd, 0xee, 0xff };

static unsigned char iv[] = { 0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f };

/*
 * Mainly adopted from <https://wiki.openssl.org/index.php/EVP_Symmetric_Encryption_and_Decryption>.
 */

static const unsigned char skey[] = { 0x00, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99, 0xaa, 0xbb, 0xcc, 0xdd, 0xee, 0xff };

static const unsigned char siv[] = { 0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f };

#define COUNT 48

#define use_evp(cipher)                                                               \
    void evp_##cipher(unsigned char* key, unsigned char* iv)                          \
    {                                                                                 \
        /* Message to be encrypted */                                                 \
        unsigned char plaintext[128] = "The quick brown fox jumps over the lazy dog"; \
        for (int i = 0; i < 48; i++) {                                                \
            plaintext[i] = rand() % 256;                                              \
        }                                                                             \
                                                                                      \
        /*                                                                            \
         * Buffer for ciphertext. Ensure the buffer is long enough for the            \
         * ciphertext which may be longer than the plaintext, depending on the        \
         * algorithm and mode.                                                        \
         */                                                                           \
        unsigned char enc_out[128];                                                   \
                                                                                      \
        /* Buffer for the decrypted text */                                           \
        unsigned char dec_out[128];                                                   \
                                                                                      \
        int dec_len, enc_len;                                                         \
                                                                                      \
        /* Encrypt the plaintext */                                                   \
        enc_len = encrypt_##cipher(plaintext, COUNT, key, iv,                         \
            enc_out);                                                                 \
                                                                                      \
        /* Decrypt the ciphertext */                                                  \
        dec_len = decrypt_##cipher(enc_out, enc_len, key, iv,                         \
            dec_out);                                                                 \
                                                                                      \
        print_result(plaintext, enc_out, dec_out, COUNT);                             \
                                                                                      \
        return;                                                                       \
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

void raw_aes_128_cbc(unsigned char* key, unsigned char* iv)
{
    unsigned char siv[16];
    unsigned char plaintext[COUNT];
    unsigned char enc_out[COUNT];
    unsigned char dec_out[COUNT];

    for (int i = 0; i < COUNT; i++) {
        plaintext[i] = rand() % 256;
    }

    AES_KEY enc_key, dec_key;

    AES_set_encrypt_key(key, 128, &enc_key);
    AES_encrypt(plaintext, enc_out, &enc_key);
    memcpy(siv, iv, 16);
    AES_cbc_encrypt(plaintext, enc_out, COUNT, &enc_key, siv, 1);

    AES_set_decrypt_key(key, 128, &dec_key);
    memcpy(siv, iv, 16);
    AES_cbc_encrypt(enc_out, dec_out, COUNT, &dec_key, siv, 0);

    printd(plaintext, COUNT);
    printd(enc_out, COUNT);
    printd(dec_out, COUNT);
    printf("\n");
}

int main(int argc, char* argv[])
{
    for (int i = 0; i < 1000; i++) {
        // evp_aes_128_cbc(key, iv);
        raw_aes_128_cbc(key, iv);
        // evp_aes_128_ctr(key, iv);
    }
    return 0;
}
