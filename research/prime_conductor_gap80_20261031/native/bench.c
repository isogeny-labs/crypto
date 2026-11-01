#define _POSIX_C_SOURCE 200809L
#include <flint/flint.h>
#include <flint/fmpz.h>
#include <flint/fmpz_mod.h>
#include <flint/fmpz_mod_poly.h>
#include <flint/fmpz_poly.h>
#include <time.h>
#include <stdio.h>
#include <string.h>
#include <stdlib.h>
char *baseline(const char*,const char*,const char*,const char*,const char*,const char*);
char *optimized(const char*,const char*,const char*,const char*,const char*,const char*);
static double now(void){struct timespec t;clock_gettime(CLOCK_MONOTONIC,&t);return t.tv_sec+t.tv_nsec*1e-9;}
int main(int argc,char**argv){
 slong n=argc>1?atol(argv[1]):1024;int reps=argc>2?atoi(argv[2]):5;
 const char *ps="2990824833105251809312850774942586406637913479771";
 fmpz_t p;fmpz_init(p);fmpz_set_str(p,ps,10);
 fmpz_mod_ctx_t ctx;fmpz_mod_ctx_init(ctx,p);
 flint_rand_t rng;flint_rand_init(rng);flint_rand_set_seed(rng,20261031,80);
 fmpz_mod_poly_t a;fmpz_mod_poly_init(a,ctx);fmpz_poly_t b;fmpz_poly_init(b);
 char *s[5];
 for(int i=0;i<5;i++){
   fmpz_mod_poly_randtest(a,rng,n,ctx);
   if(i==0)fmpz_mod_poly_set_coeff_ui(a,n,1,ctx);
   fmpz_mod_poly_get_fmpz_poly(b,a,ctx);s[i]=fmpz_poly_get_str(b);
 }
 for(int r=0;r<reps;r++){
   char *answer[2]={NULL,NULL};double elapsed[2];
   for(int k=0;k<2;k++){
     int v=(r+k)%2;double t=now();
     answer[v]=(v?optimized:baseline)(ps,s[0],s[1],s[2],s[3],s[4]);elapsed[v]=now()-t;
   }
   if(!answer[0]||!answer[1]||strcmp(answer[0],answer[1])){fprintf(stderr,"mismatch n=%ld r=%d\n",n,r);return 1;}
   printf("{\"degree\":%ld,\"repeat\":%d,\"baseline_seconds\":%.9f,\"optimized_seconds\":%.9f,\"equal\":true}\n",n,r,elapsed[0],elapsed[1]);fflush(stdout);
   flint_free(answer[0]);flint_free(answer[1]);
 }
 for(int i=0;i<5;i++)flint_free(s[i]);
 fmpz_poly_clear(b);fmpz_mod_poly_clear(a,ctx);flint_rand_clear(rng);fmpz_mod_ctx_clear(ctx);fmpz_clear(p);flint_cleanup();return 0;
}
