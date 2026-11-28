/* EXP12b: cost of one modular add and one modular mul (mod a random odd l of NL limbs), GMP mpn layer.
   Used as the unit cost of sparse linear algebra (Lanczos/Wiedemann) over Z/l. */
#include <gmp.h>
#include <stdio.h>
#include <time.h>
#include <stdlib.h>
static double now(){struct timespec t;clock_gettime(CLOCK_MONOTONIC,&t);return t.tv_sec+1e-9*t.tv_nsec;}
int main(int argc,char**argv){
  int NL=atoi(argv[1]); long R=20000000; mp_limb_t l[8],a[8],b[8],p[16],q[16],r[8]; volatile mp_limb_t sink=0;
  for(int i=0;i<NL;i++){l[i]=((mp_limb_t)rand()<<33)^rand()^((mp_limb_t)rand()<<17);a[i]=rand();b[i]=rand();} l[NL-1]|=(mp_limb_t)1<<63; l[0]|=1;
  double t0=now();
  for(long k=0;k<R;k++){ mp_limb_t c=mpn_add_n(a,a,b,NL); if(c||mpn_cmp(a,l,NL)>=0) mpn_sub_n(a,a,l,NL); b[0]^=k; }
  double t1=now();
  for(long k=0;k<R;k++){ mpn_mul_n(p,a,b,NL); mpn_tdiv_qr(q,r,0,p,2*NL,l,NL); a[0]=r[0]^k; sink^=r[1]; }
  double t2=now();
  printf("limbs=%d  modadd %.2f ns  modmul(mul+tdiv) %.2f ns\n",NL,(t1-t0)/R*1e9,(t2-t1)/R*1e9);
  return 0;}
