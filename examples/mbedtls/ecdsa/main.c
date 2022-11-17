#include "mbedtls/asn1write.h"
#include "mbedtls/ctr_drbg.h"
#include "mbedtls/ecdsa.h"
#include "mbedtls/entropy.h"

int main()
{
    const char* pers = "ecdsa";
    mbedtls_entropy_context entropy;
    mbedtls_ctr_drbg_context ctr_drbg;
    int ret;

    mbedtls_ecdsa_context ecdsa;
    const mbedtls_ecp_curve_info* curve_info = mbedtls_ecp_curve_info_from_grp_id(MBEDTLS_ECP_DP_SECP192K1);

    mbedtls_ecdsa_init(&ecdsa);
    mbedtls_ctr_drbg_init(&ctr_drbg);

    mbedtls_ecdsa_init(&ecdsa);
    fflush(stdout);

    mbedtls_entropy_init(&entropy);
    if ((ret = mbedtls_ctr_drbg_seed(&ctr_drbg, mbedtls_entropy_func, &entropy,
             (const unsigned char*)pers,
             strlen(pers)))
        != 0) {
        exit(1);
    }
    fflush(stdout);

    mbedtls_ecdsa_genkey(&ecdsa, curve_info->grp_id, mbedtls_ctr_drbg_random, &ctr_drbg);

    for (int i = 0; i < 1000; i++) {
        unsigned char buf[128];
        unsigned char tmp[200];
        size_t sig_len;

        memset(buf, 0x2A, sizeof(buf));

        for (int j = 0; j < 16; j++) {
            buf[j] = rand() % 256;
        }

        int ret_write_sign = mbedtls_ecdsa_write_signature(&ecdsa, MBEDTLS_MD_SHA256, buf, curve_info->bit_size,
            tmp, sizeof(tmp), &sig_len, mbedtls_ctr_drbg_random, &ctr_drbg);

        mbedtls_mpi r, s;
        mbedtls_mpi_init(&r);
        mbedtls_mpi_init(&s);
        size_t len;
        unsigned char* p = (unsigned char*)tmp;
        const unsigned char* end = tmp + sig_len;
        mbedtls_asn1_get_tag(&p, end, &len,
            MBEDTLS_ASN1_CONSTRUCTED | MBEDTLS_ASN1_SEQUENCE);
        mbedtls_asn1_get_mpi(&p, end, &r);
        mbedtls_asn1_get_mpi(&p, end, &s);

        char r_str[64];
        char s_str[64];
        size_t r_len;
        size_t s_len;

        mbedtls_mpi_write_string(&r, 16, r_str, 64, &r_len);
        mbedtls_mpi_write_string(&s, 16, s_str, 64, &s_len);

        printf("%s\n", r_str);
        printf("%s\n", s_str);

        int ret_verify = mbedtls_ecdsa_read_signature(&ecdsa, buf, curve_info->bit_size, tmp, sig_len);
        printf("%d\n\n", !ret_verify);
    }
}