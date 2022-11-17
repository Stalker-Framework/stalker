#include <gcrypt.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

void xerr(const char* msg)
{
    fprintf(stderr, "%s\n", msg);
    exit(1);
}

void gcrypt_init()
{
    gcry_error_t err = 0;

    /* We don't want to see any warnings, e.g. because we have not yet
       parsed program options which might be used to suppress such
       warnings. */
    err = gcry_control(GCRYCTL_SUSPEND_SECMEM_WARN);

    /* ... If required, other initialization goes here.  Note that the
       process might still be running with increased privileges and that
       the secure memory has not been intialized.  */

    /* Allocate a pool of 16k secure memory.  This make the secure memory
       available and also drops privileges where needed.  */
    err |= gcry_control(GCRYCTL_INIT_SECMEM, 16384, 0);

    /* It is now okay to let Libgcrypt complain when there was/is
       a problem with the secure memory. */
    err |= gcry_control(GCRYCTL_RESUME_SECMEM_WARN);

    /* ... If required, other initialization goes here.  */

    /* Tell Libgcrypt that initialization has completed. */
    err |= gcry_control(GCRYCTL_INITIALIZATION_FINISHED, 0);

    if (err) {
        xerr("gcrypt: failed initialization");
    }
}

int dump_sig(gcry_sexp_t sig)
{
    gcry_sexp_t dsa = gcry_sexp_find_token(sig, "ecdsa", 0);
    gcry_mpi_t r = gcry_sexp_nth_mpi(gcry_sexp_find_token(dsa, "r", 0), 1, GCRYMPI_FMT_USG);
    gcry_mpi_t s = gcry_sexp_nth_mpi(gcry_sexp_find_token(dsa, "s", 0), 1, GCRYMPI_FMT_USG);

    size_t nw;
    unsigned char* buffer;
    gcry_mpi_aprint(GCRYMPI_FMT_HEX, &buffer, &nw, r);
    printf("%s\n", buffer);
    gcry_mpi_aprint(GCRYMPI_FMT_HEX, &buffer, &nw, s);
    printf("%s\n", buffer);
    free(buffer);
}

int do_sign_verify(gcry_sexp_t ecc_keypair)
{
    // Hash that need to be signed.
    gcry_mpi_t m = gcry_mpi_new(128);
    gcry_mpi_randomize(m, 128, GCRY_WEAK_RANDOM);
    gcry_sexp_t data;
    gcry_sexp_build(&data, NULL, "(data (flags raw) (value %m))", m);

    gcry_sexp_t sig;
    gcry_err_code_t err = 0;
    err = gcry_pk_sign(&sig, data, gcry_sexp_find_token(ecc_keypair, "private-key", 0));

    if (err) {
        xerr("Failed to sign.");
    }
    dump_sig(sig);
    err = gcry_pk_verify(sig, data, gcry_sexp_find_token(ecc_keypair, "public-key", 0));
    printf("%d\n", !err);
    printf("\n");
}

int main(int argc, char** argv)
{
    gcrypt_init();

    gcry_error_t err = 0;
    gcry_sexp_t ecc_parms;
    gcry_sexp_t ecc_keypair;

    // Build sexps and generate the key pair
    err |= gcry_sexp_build(&ecc_parms, NULL, "(genkey (ecc (curve secp192r1)))");
    err |= gcry_pk_genkey(&ecc_keypair, ecc_parms);

    for (int i = 0; i < 1000; i++) {
        do_sign_verify(ecc_keypair);
    }

    // Release contexts.
    gcry_sexp_release(ecc_keypair);
    gcry_sexp_release(ecc_parms);

    return 0;
}