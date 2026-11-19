/* Allocation/string ABI only. Isogeny, jets, coverage and log-norm logic are Rust.
 * This exposes existing FLINT primitives without copying their ABI structs into
 * Rust. Library lifetime/context ownership is enforced by the Rust Rc wrapper. */
#include <flint/flint.h>
#include <flint/fmpz.h>
#include <flint/fmpz_mod.h>
#include <flint/fmpz_mod_poly.h>
#include <flint/fmpz_vec.h>

unsigned long crypto_flint_abi(void) { return 1; }
int crypto_flint_prime(const char *s) {
    fmpz_t n; fmpz_init(n);
    int ok = fmpz_set_str(n, s, 10) == 0 && fmpz_cmp_ui(n, 3) > 0;
    int result = ok ? fmpz_is_prime(n) : 0;
    fmpz_clear(n); return result;
}
void *crypto_flint_ctx_new(const char *s) {
    fmpz_t n; fmpz_init(n);
    if (fmpz_set_str(n, s, 10) != 0 || fmpz_cmp_ui(n, 3) <= 0) {
        fmpz_clear(n); return NULL;
    }
    fmpz_mod_ctx_struct *ctx = flint_malloc(sizeof(*ctx));
    fmpz_mod_ctx_init(ctx, n); fmpz_clear(n); return ctx;
}
void crypto_flint_ctx_free(void *v) {
    if (v) { fmpz_mod_ctx_clear(v); flint_free(v); }
}
void *crypto_flint_poly_new(const void *ctx) {
    fmpz_mod_poly_struct *p = flint_malloc(sizeof(*p));
    fmpz_mod_poly_init(p, ctx); return p;
}
void crypto_flint_poly_free(const void *ctx, void *p) {
    if (p) { fmpz_mod_poly_clear(p, ctx); flint_free(p); }
}
long crypto_flint_poly_length(const void *ctx, const void *p) {
    return fmpz_mod_poly_length(p, ctx);
}
int crypto_flint_poly_set_decimal(const void *ctx, void *p, long i, const char *s) {
    if (i < 0) return 0;
    fmpz_t n; fmpz_init(n);
    int ok = fmpz_set_str(n, s, 10) == 0;
    if (ok) fmpz_mod_poly_set_coeff_fmpz(p, i, n, ctx);
    fmpz_clear(n); return ok;
}
char *crypto_flint_poly_get_decimal(const void *ctx, const void *p, long i) {
    if (i < 0) return NULL;
    fmpz_t n; fmpz_init(n);
    fmpz_mod_poly_get_coeff_fmpz(n, p, i, ctx);
    char *s = fmpz_get_str(NULL, 10, n); fmpz_clear(n); return s;
}
int crypto_flint_poly_scale(const void *ctx, void *out, const void *p, const char *s) {
    fmpz_t n; fmpz_init(n);
    int ok = fmpz_set_str(n, s, 10) == 0;
    if (ok) fmpz_mod_poly_scalar_mul_fmpz(out, p, n, ctx);
    fmpz_clear(n); return ok;
}
void *crypto_flint_poly_roots(const void *ctx, const char *const *s, long n) {
    if (n < 0) return NULL;
    fmpz *roots = _fmpz_vec_init(n);
    for (long i = 0; i < n; i++) {
        if (fmpz_set_str(roots + i, s[i], 10) != 0) {
            _fmpz_vec_clear(roots, n); return NULL;
        }
    }
    fmpz_mod_poly_struct *p = crypto_flint_poly_new(ctx);
    fmpz_mod_poly_product_roots_fmpz_vec(p, roots, n, ctx);
    _fmpz_vec_clear(roots, n); return p;
}
char *crypto_flint_poly_dot(const void *ctx, const void *ap, const void *bp) {
    const fmpz_mod_poly_struct *a = ap, *b = bp;
    slong n = FLINT_MIN(a->length, b->length);
    fmpz_t sum; fmpz_init(sum);
    _fmpz_vec_dot(sum, a->coeffs, b->coeffs, n);
    fmpz_mod_set_fmpz(sum, sum, ctx);
    char *s = fmpz_get_str(NULL, 10, sum); fmpz_clear(sum); return s;
}
void crypto_flint_string_free(char *s) { flint_free(s); }
