/* Independent direct-root oracle for log Norm jets, not another trace algorithm. */
#include <flint/flint.h>
#include <flint/fmpz.h>
#include <flint/fmpz_mod.h>
#include <flint/fmpz_mod_poly.h>
#include <flint/fmpz_poly.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
char *optimized(const char*,const char*,const char*,const char*,const char*,const char*);
static void fail(const char *s){fprintf(stderr,"FAIL %s\n",s);exit(1);}
static char *encode(const fmpz_mod_poly_t a,const fmpz_mod_ctx_t ctx){fmpz_poly_t b;fmpz_poly_init(b);fmpz_mod_poly_get_fmpz_poly(b,a,ctx);char *s=fmpz_poly_get_str(b);fmpz_poly_clear(b);return s;}
int main(void){
 const char *primes[]={"101","2990824833105251809312850774942586406637913479771","4794168233664648458988367110514316682522236917173341621535751"};
 flint_rand_t rng;flint_rand_init(rng);flint_rand_set_seed(rng,20261101,80);int count=0;
 for(int pi=0;pi<3;pi++)for(slong n=1;n<=32;n++){
  fmpz_t p,x,gi,u,v,w,a,b,inv2,inv3,sum[3];
  fmpz_init(p);fmpz_set_str(p,primes[pi],10);fmpz_mod_ctx_t ctx;fmpz_mod_ctx_init(ctx,p);
  fmpz_init(x);fmpz_init(gi);fmpz_init(u);fmpz_init(v);fmpz_init(w);fmpz_init(a);fmpz_init(b);fmpz_init(inv2);fmpz_init(inv3);
  for(int j=0;j<3;j++)fmpz_init(sum[j]);
  fmpz_set_ui(a,2);fmpz_invmod(inv2,a,p);fmpz_set_ui(a,3);fmpz_invmod(inv3,a,p);
  fmpz_mod_poly_t h,linear,g[4];fmpz_mod_poly_init(h,ctx);fmpz_mod_poly_init(linear,ctx);
  fmpz_mod_poly_one(h,ctx);fmpz_mod_poly_set_coeff_ui(linear,1,1,ctx);
  for(slong i=1;i<=n;i++){fmpz_set_ui(x,i);fmpz_neg(a,x);fmpz_mod_poly_set_coeff_fmpz(linear,0,a,ctx);fmpz_mod_poly_mul(h,h,linear,ctx);}
  for(int j=0;j<4;j++){fmpz_mod_poly_init(g[j],ctx);fmpz_mod_poly_randtest(g[j],rng,2*n+2,ctx);}
  /* g0=X+1 guarantees no zero at our roots for all three fields. */
  fmpz_mod_poly_zero(g[0],ctx);fmpz_mod_poly_set_coeff_ui(g[0],0,1,ctx);fmpz_mod_poly_set_coeff_ui(g[0],1,1,ctx);
  for(slong i=1;i<=n;i++){
   fmpz_set_ui(x,i);fmpz_add_ui(a,x,1);if(!fmpz_invmod(gi,a,p))fail("oracle inverse");
   fmpz_mod_poly_evaluate_fmpz(u,g[1],x,ctx);fmpz_mul(u,u,gi);fmpz_mod(u,u,p);
   fmpz_mod_poly_evaluate_fmpz(v,g[2],x,ctx);fmpz_mul(v,v,gi);fmpz_mod(v,v,p);
   fmpz_mod_poly_evaluate_fmpz(w,g[3],x,ctx);fmpz_mul(w,w,gi);fmpz_mod(w,w,p);
   fmpz_add(sum[0],sum[0],u);
   fmpz_mul(a,u,u);fmpz_mul(b,a,inv2);fmpz_sub(b,v,b);fmpz_add(sum[1],sum[1],b);
   fmpz_mul(a,a,u);fmpz_mul(a,a,inv3);fmpz_mul(b,u,v);fmpz_sub(b,w,b);fmpz_add(b,b,a);fmpz_add(sum[2],sum[2],b);
  }
  char *hs=encode(h,ctx),*ss[4];for(int j=0;j<4;j++)ss[j]=encode(g[j],ctx);
  char *out=optimized(primes[pi],hs,ss[0],ss[1],ss[2],ss[3]);if(!out)fail("null result");
  char *tok=strtok(out," ");for(int j=0;j<3;j++){if(!tok||fmpz_set_str(a,tok,10))fail("parse");fmpz_mod(sum[j],sum[j],p);if(!fmpz_equal(a,sum[j]))fail("direct-root mismatch");tok=strtok(NULL," ");}if(tok)fail("extra token");
  flint_free(out);flint_free(hs);for(int j=0;j<4;j++){flint_free(ss[j]);fmpz_mod_poly_clear(g[j],ctx);}
  /* h shares a factor with g0=h: inversion must fail. */
  hs=encode(h,ctx);out=optimized(primes[pi],hs,hs,"1  1","0","0");if(out)fail("noninvertible accepted");flint_free(hs);
  fmpz_mod_poly_clear(h,ctx);fmpz_mod_poly_clear(linear,ctx);fmpz_mod_ctx_clear(ctx);
  fmpz_clear(p);fmpz_clear(x);fmpz_clear(gi);fmpz_clear(u);fmpz_clear(v);fmpz_clear(w);fmpz_clear(a);fmpz_clear(b);fmpz_clear(inv2);fmpz_clear(inv3);for(int j=0;j<3;j++)fmpz_clear(sum[j]);count++;
 }
 printf("{\"status\":\"pass\",\"direct_root_cases\":%d,\"noninvertible_controls\":%d,\"fields\":3,\"includes_80bit_candidate_field\":true}\n",count,count);
 flint_rand_clear(rng);flint_cleanup();return 0;
}
