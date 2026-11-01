/* SPDX-License-Identifier: GPL-2.0-or-later
 * Prime-field FLINT log-norm jets. Text conversion avoids relying on the
 * private ABI of Python-FLINT's polynomial objects. */
#include <flint/flint.h>
#include <flint/fmpz.h>
#include <flint/fmpz_poly.h>
#include <flint/fmpz_mod.h>
#include <flint/fmpz_mod_poly.h>
#include <stdio.h>
#include <string.h>

static int readpoly(fmpz_mod_poly_t out,const char *s,const fmpz_mod_ctx_t ctx) {
    fmpz_poly_t a; fmpz_poly_init(a);
    int error=fmpz_poly_set_str(a,s);
    if(!error) fmpz_mod_poly_set_fmpz_poly(out,a,ctx);
    fmpz_poly_clear(a); return error;
}

char *prime_log_norm_jet(const char *ps,const char *hs,const char *s0,
                        const char *s1,const char *s2,const char *s3) {
    fmpz_t p,two,three,i2,i3,z[3];
    fmpz_init(p);fmpz_init(two);fmpz_init(three);fmpz_init(i2);fmpz_init(i3);
    for(int j=0;j<3;j++)fmpz_init(z[j]);
    if(fmpz_set_str(p,ps,10)||!fmpz_is_probabprime(p)) {
        fmpz_clear(p);fmpz_clear(two);fmpz_clear(three);fmpz_clear(i2);fmpz_clear(i3);
        for(int j=0;j<3;j++)fmpz_clear(z[j]);
        return NULL;
    }
    fmpz_mod_ctx_t ctx;fmpz_mod_ctx_init(ctx,p);
    fmpz_mod_poly_t h,g[4],gi,u,v,w,u2,u3,uv,c2,c3,dh,tmp,rev,hi,weights;
    fmpz_mod_poly_init(rev,ctx);fmpz_mod_poly_init(hi,ctx);fmpz_mod_poly_init(weights,ctx);
    fmpz_mod_poly_init(h,ctx);fmpz_mod_poly_init(gi,ctx);
    for(int j=0;j<4;j++)fmpz_mod_poly_init(g[j],ctx);
    fmpz_mod_poly_init(u,ctx);fmpz_mod_poly_init(v,ctx);fmpz_mod_poly_init(w,ctx);
    fmpz_mod_poly_init(u2,ctx);fmpz_mod_poly_init(u3,ctx);fmpz_mod_poly_init(uv,ctx);
    fmpz_mod_poly_init(c2,ctx);fmpz_mod_poly_init(c3,ctx);fmpz_mod_poly_init(dh,ctx);fmpz_mod_poly_init(tmp,ctx);
    char *result=NULL;
    if(readpoly(h,hs,ctx)||readpoly(g[0],s0,ctx)||readpoly(g[1],s1,ctx)||
       readpoly(g[2],s2,ctx)||readpoly(g[3],s3,ctx))goto cleanup;
    if(fmpz_mod_poly_degree(h,ctx)<1)goto cleanup;
    slong n=fmpz_mod_poly_degree(h,ctx);
    fmpz_mod_poly_get_coeff_fmpz(z[0],h,n,ctx);
    if(!fmpz_is_one(z[0]))goto cleanup;
    fmpz_mod_poly_reverse(rev,h,n+1,ctx);
    fmpz_mod_poly_inv_series(hi,rev,n,ctx);
    if(!fmpz_mod_poly_invmod(gi,g[0],h,ctx))goto cleanup;
    fmpz_mod_poly_mulmod(u,g[1],gi,h,ctx);
    fmpz_mod_poly_mulmod(v,g[2],gi,h,ctx);
    fmpz_mod_poly_mulmod(w,g[3],gi,h,ctx);
    fmpz_mod_poly_mulmod_preinv(u2,u,u,h,hi,ctx);fmpz_mod_poly_mulmod_preinv(u3,u2,u,h,hi,ctx);
    fmpz_mod_poly_mulmod_preinv(uv,u,v,h,hi,ctx);
    fmpz_set_ui(two,2);fmpz_set_ui(three,3);
    if(!fmpz_invmod(i2,two,p)||!fmpz_invmod(i3,three,p))goto cleanup;
    fmpz_mod_poly_scalar_mul_fmpz(tmp,u2,i2,ctx);fmpz_mod_poly_sub(c2,v,tmp,ctx);
    fmpz_mod_poly_scalar_mul_fmpz(tmp,u3,i3,ctx);fmpz_mod_poly_sub(c3,w,uv,ctx);
    fmpz_mod_poly_add(c3,c3,tmp,ctx);fmpz_mod_poly_derivative(dh,h,ctx);
    /* Tr(X^i) are coefficients of rev(h')/rev(h). Compute once,
       then share the trace functional across all three log coefficients.
       This identity requires monic h, as did the original coefficient rule. */
    fmpz_mod_poly_reverse(rev,dh,n,ctx);
    fmpz_mod_poly_mullow(weights,rev,hi,n,ctx);
    const fmpz_mod_poly_struct *polys[3]={u,c2,c3};
    for(int j=0;j<3;j++) {
        fmpz_zero(z[j]);
        for(slong i=0;i<FLINT_MIN(fmpz_mod_poly_length(polys[j],ctx),fmpz_mod_poly_length(weights,ctx));i++)
            fmpz_addmul(z[j],polys[j]->coeffs+i,weights->coeffs+i);
        fmpz_mod(z[j],z[j],p);
    }
    char *r0=fmpz_get_str(NULL,10,z[0]),*r1=fmpz_get_str(NULL,10,z[1]),*r2=fmpz_get_str(NULL,10,z[2]);
    size_t len=strlen(r0)+strlen(r1)+strlen(r2)+4;
    result=flint_malloc(len);snprintf(result,len,"%s %s %s",r0,r1,r2);
    flint_free(r0);flint_free(r1);flint_free(r2);
cleanup:
    fmpz_mod_poly_clear(rev,ctx);fmpz_mod_poly_clear(hi,ctx);fmpz_mod_poly_clear(weights,ctx);
    fmpz_mod_poly_clear(h,ctx);fmpz_mod_poly_clear(gi,ctx);
    for(int j=0;j<4;j++)fmpz_mod_poly_clear(g[j],ctx);
    fmpz_mod_poly_clear(u,ctx);fmpz_mod_poly_clear(v,ctx);fmpz_mod_poly_clear(w,ctx);
    fmpz_mod_poly_clear(u2,ctx);fmpz_mod_poly_clear(u3,ctx);fmpz_mod_poly_clear(uv,ctx);
    fmpz_mod_poly_clear(c2,ctx);fmpz_mod_poly_clear(c3,ctx);fmpz_mod_poly_clear(dh,ctx);fmpz_mod_poly_clear(tmp,ctx);
    fmpz_mod_ctx_clear(ctx);
    fmpz_clear(p);fmpz_clear(two);fmpz_clear(three);fmpz_clear(i2);fmpz_clear(i3);
    for(int j=0;j<3;j++)fmpz_clear(z[j]);
    return result;
}
void prime_log_norm_free(void *p){flint_free(p);}
