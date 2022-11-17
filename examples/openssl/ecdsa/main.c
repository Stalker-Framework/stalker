#include <openssl/ec.h>
#include <openssl/obj_mac.h>
#include <openssl/ecdsa.h>
#include <openssl/bn.h>
#include <stdio.h>

int ecdsa_verify(EC_KEY* eckey, unsigned char* hash, ECDSA_SIG* signature)
{
    return ECDSA_do_verify(hash, strlen(hash), signature, eckey);
}

ECDSA_SIG* ecdsa_sign(EC_KEY* eckey, unsigned char* hash)
{
    return ECDSA_do_sign(hash, strlen(hash), eckey);
}

void printsig(ECDSA_SIG* sig)
{
    char* r_str = BN_bn2hex(ECDSA_SIG_get0_r(sig));
    char* s_str = BN_bn2hex(ECDSA_SIG_get0_s(sig));

    printf("%s\n%s\n", r_str, s_str);
}

void ecdsa_exp(EC_KEY* eckey)
{
    unsigned char hash[16];
    for (int i = 0; i < 16; i++) {
        hash[i] = rand();
    }
    ECDSA_SIG* sig = ecdsa_sign(eckey, hash);
    int status = ecdsa_verify(eckey, hash, sig);
    printsig(sig);
    printf("%d\n", status);
    printf("\n");
}

int main(int argc, char* argv[])
{
    EC_KEY* eckey = EC_KEY_new();
    EC_GROUP* ecgroup = EC_GROUP_new_by_curve_name(NID_secp192k1);
    EC_KEY_set_group(eckey, ecgroup);
    EC_KEY_generate_key(eckey);

    for (int i = 0; i < 1000; i++) {
        ecdsa_exp(eckey);
    }

    EC_GROUP_free(ecgroup);
    EC_KEY_free(eckey);
    return (0);
}