#ifndef EVP_EXPS_H
#define EVP_EXPS_H

#define declare_evp(cipher)                                                                 \
    void evp_##cipher(unsigned char* key, unsigned char* iv);                                                                \
    int decrypt_##cipher(unsigned char* ciphertext, int ciphertext_len, unsigned char* key, \
        unsigned char* iv, unsigned char* plaintext);                                       \
    int encrypt_##cipher(unsigned char* plaintext, int plaintext_len, unsigned char* key,   \
        unsigned char* iv, unsigned char* ciphertext);

declare_evp(aes_128_cbc);
declare_evp(aes_128_ctr);

void handleErrors(void);

#endif // !EVP_EXPS_H