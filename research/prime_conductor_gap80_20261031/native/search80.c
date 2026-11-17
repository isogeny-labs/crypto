/* Synthetic CM(-7) parameter search and native arithmetic verification.
 * Does not construct the quotient or evaluate an isogeny. */
#include <stdio.h>
#include <stdlib.h>
#include <time.h>
#include <sys/resource.h>
#include <gmp.h>
#include <flint/fmpz.h>
typedef struct {mpz_t x,y;int infinity;} point;
static mpz_t p;
static long curve_b=-784;
static void init(point *a){mpz_inits(a->x,a->y,NULL);a->infinity=1;}
static void clear(point *a){mpz_clears(a->x,a->y,NULL);}
static void copy(point *a,const point *b){mpz_set(a->x,b->x);mpz_set(a->y,b->y);a->infinity=b->infinity;}
static void add(point *r,const point *a,const point *b){
 if(a->infinity){copy(r,b);return;}if(b->infinity){copy(r,a);return;}
 mpz_t s,d,x,y;mpz_inits(s,d,x,y,NULL);
 if(mpz_cmp(a->x,b->x)==0){
   if(mpz_cmp(a->y,b->y)||mpz_sgn(a->y)==0){r->infinity=1;goto done;}
   mpz_mul(s,a->x,a->x);mpz_mul_ui(s,s,3);mpz_sub_ui(s,s,140);mpz_mul_ui(d,a->y,2);
 }else{mpz_sub(s,b->y,a->y);mpz_sub(d,b->x,a->x);}
 if(!mpz_invert(d,d,p)){fprintf(stderr,"noninvertible denominator\n");exit(2);}
 mpz_mul(s,s,d);mpz_mod(s,s,p);mpz_mul(x,s,s);mpz_sub(x,x,a->x);mpz_sub(x,x,b->x);mpz_mod(x,x,p);
 mpz_sub(y,a->x,x);mpz_mul(y,s,y);mpz_sub(y,y,a->y);mpz_mod(y,y,p);
 mpz_set(r->x,x);mpz_set(r->y,y);r->infinity=0;
done:mpz_clears(s,d,x,y,NULL);
}
static void mul(point *r,const point *a,const mpz_t n){
 point t;init(&t);r->infinity=1;
 for(size_t i=mpz_sizeinbase(n,2);i-->0;){add(&t,r,r);copy(r,&t);if(mpz_tstbit(n,i)){add(&t,r,a);copy(r,&t);}}
 clear(&t);
}
static int oncurve(const point *a){
 if(a->infinity)return 1;
 mpz_t x,y,t;mpz_inits(x,y,t,NULL);mpz_mul(x,a->x,a->x);mpz_mul(x,x,a->x);
 mpz_mul_ui(t,a->x,140);mpz_sub(x,x,t);
 if(curve_b<0)mpz_sub_ui(x,x,-curve_b);else mpz_add_ui(x,x,curve_b);
 mpz_mul(y,a->y,a->y);mpz_sub(x,x,y);mpz_mod(x,x,p);
 int ok=mpz_sgn(x)==0;mpz_clears(x,y,t,NULL);return ok;
}
static void check(int ok,const char *s){if(!ok){fprintf(stderr,"FAIL %s\n",s);exit(3);}}
static void printpoint(const char *name,const point *a){gmp_printf("\"%s\":[\"%Zd\",\"%Zd\"],\n",name,a->x,a->y);}
int main(void){
 struct timespec start,end;timespec_get(&start,TIME_UTC);clock_t cpu=clock();
 mpz_t ell,k,q,t,n,z,exp,rhs,co,w;mpz_inits(p,ell,k,q,t,n,z,exp,rhs,co,w,NULL);
 mpz_set_ui(k,3000031);mpz_mul(q,k,k);mpz_add_ui(q,q,7);mpz_divexact_ui(q,q,8);
 fmpz_t f;fmpz_init(f);fmpz_set_mpz(f,q);check(fmpz_is_prime(f),"prime q");
 unsigned long c;unsigned trials=0;
 for(c=32769;c<200000;c+=2){
   mpz_set_ui(ell,c);mpz_mul_2exp(ell,ell,64);mpz_add_ui(ell,ell,1);
   if(!mpz_probab_prime_p(ell,30))continue;
   mpz_set_si(z,-7);if(mpz_kronecker(z,ell)!=-1)continue;
   mpz_mul(z,k,ell);mpz_add_ui(z,z,1);mpz_mul(p,z,z);mpz_mul(z,ell,ell);mpz_mul_ui(z,z,7);mpz_add(p,p,z);
   trials++;if(!mpz_probab_prime_p(p,30))continue;
   fmpz_set_mpz(f,ell);if(!fmpz_is_prime(f))continue;
   fmpz_set_mpz(f,p);if(fmpz_is_prime(f))break;
 }
 check(c<200000,"search budget");
 mpz_mul(t,k,ell);mpz_add_ui(t,t,1);mpz_mul_ui(t,t,2);
 mpz_add_ui(n,p,1);mpz_sub(n,n,t);
 mpz_mul(z,ell,ell);mpz_mul(z,z,q);mpz_mul_ui(z,z,8);check(mpz_cmp(n,z)==0,"order identity");
 /* Pocklington certificate for ell: F=2^64 > sqrt(ell). */
 mpz_sub_ui(exp,ell,1);
 unsigned long witness;
 for(witness=2;witness<100;witness++){
   mpz_set_ui(w,witness);mpz_powm(z,w,exp,ell);if(mpz_cmp_ui(z,1))continue;
   mpz_fdiv_q_2exp(co,exp,1);mpz_powm(z,w,co,ell);mpz_sub_ui(z,z,1);mpz_gcd(z,z,ell);if(mpz_cmp_ui(z,1)==0)break;
 }
 check(witness<100,"Pocklington witness");
 point P,K,G,R;init(&P);init(&K);init(&G);init(&R);
 check(mpz_fdiv_ui(p,4)==3,"square root shortcut");mpz_add_ui(exp,p,1);mpz_fdiv_q_2exp(exp,exp,2);
 int found=0;
 for(int twist=0;twist<2&&!found;twist++){
 curve_b=twist?784:-784;
 for(unsigned long x=1;x<100&&!found;x++){
   mpz_set_ui(P.x,x);mpz_mul(rhs,P.x,P.x);mpz_mul(rhs,rhs,P.x);mpz_mul_ui(z,P.x,140);mpz_sub(rhs,rhs,z);
   if(curve_b<0)mpz_sub_ui(rhs,rhs,-curve_b);else mpz_add_ui(rhs,rhs,curve_b);
   mpz_mod(rhs,rhs,p);
   if(mpz_legendre(rhs,p)!=1)continue;
   mpz_powm(P.y,rhs,exp,p);P.infinity=0;check(oncurve(&P),"point");
   mul(&R,&P,n);if(!R.infinity)continue;
   mpz_mul_ui(co,t,2);mul(&R,&P,co);if(R.infinity)continue;
   mpz_mul_ui(co,q,8);mul(&K,&P,co);if(K.infinity)continue;mul(&R,&K,ell);if(!R.infinity)continue;
   mpz_divexact(co,n,q);mul(&G,&P,co);if(G.infinity)continue;mul(&R,&G,q);if(!R.infinity)continue;
   found=1;
 }
 }
 check(found,"nonzero kernel and subgroup");check(oncurve(&K)&&oncurve(&G),"output curves");
 printf("{\n\"schema\":\"gap80-native-source-candidate-v1\",\n");
 gmp_printf("\"ell\":\"%Zd\",\n\"ell_bits\":%zu,\n\"p\":\"%Zd\",\n\"p_bits\":%zu,\n\"k\":\"%Zd\",\n\"q\":\"%Zd\",\n\"trace\":\"%Zd\",\n\"ambient_order\":\"%Zd\",\n",ell,mpz_sizeinbase(ell,2),p,mpz_sizeinbase(p,2),k,q,t,n);
 printf("\"source_coefficients\":[\"0\",\"0\",\"0\",\"-140\",\"%ld\"],\n\"source_conductor\":1,\n\"fundamental_discriminant\":-7,\n",curve_b);
 printpoint("trace_witness",&P);printpoint("kernel",&K);printpoint("generator",&G);
 printf("\"ell_pocklington\":{\"known_factor\":\"18446744073709551616\",\"prime_divisor\":2,\"witness\":%lu},\n\"prime_verifier\":\"FLINT fmpz_is_prime for p, ell, q\",\n\"prime_field_candidates_tested\":%u,\n\"status\":\"source_and_kernel_verified_native; explicit quotient and map NOT computed\"\n}\n",witness,trials);
 clear(&P);clear(&K);clear(&G);clear(&R);fmpz_clear(f);mpz_clears(p,ell,k,q,t,n,z,exp,rhs,co,w,NULL);flint_cleanup();
 timespec_get(&end,TIME_UTC);struct rusage ru;getrusage(RUSAGE_SELF,&ru);
 fprintf(stderr,"{\"wall_seconds\":%.6f,\"cpu_seconds\":%.6f,\"peak_rss_kib\":%ld,\"flint_version\":\"%s\"}\n",(end.tv_sec-start.tv_sec)+(end.tv_nsec-start.tv_nsec)*1e-9,(double)(clock()-cpu)/CLOCKS_PER_SEC,ru.ru_maxrss,FLINT_VERSION);
 return 0;
}
