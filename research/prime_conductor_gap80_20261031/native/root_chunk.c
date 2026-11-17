/* Bounded native root preparation. Reuse checked GMP point arithmetic;
 * does not construct the kernel polynomial, quotient or point map. */
#define main historical_search80_main
#include "search80.c"
#undef main
#include <errno.h>
#include <string.h>

int main(int argc,char **argv) {
 if(argc!=8){fprintf(stderr,"usage: root_chunk P ELL KX KY FIRST_ORDINAL COUNT OUTPUT\n");return 2;}
 mpz_t ell,first,index,end,B,z;mpz_inits(p,ell,first,index,end,B,z,NULL);
 if(mpz_set_str(p,argv[1],10)||mpz_set_str(ell,argv[2],10)||
    mpz_set_str(first,argv[5],10)||mpz_sgn(first)<0||mpz_cmp_ui(ell,3)<0||mpz_even_p(ell))return 2;
 char *tail;errno=0;unsigned long count=strtoul(argv[6],&tail,10);
 if(errno||*tail||!count||count>65536)return 2;
 mpz_sub_ui(B,ell,1);mpz_fdiv_q_2exp(B,B,2);mpz_sqrt(B,B);if(mpz_sgn(B)==0)mpz_set_ui(B,1);
 mpz_add_ui(end,first,count);if(mpz_cmp(end,B)>0){fprintf(stderr,"chunk exceeds baby-root window\n");return 2;}
 if(mpz_sizeinbase(p,2)>256||mpz_cmp_ui(p,7)<=0)return 2;
 point K,Q,step,R;init(&K);init(&Q);init(&step);init(&R);curve_b=784;
 if(mpz_set_str(K.x,argv[3],10)||mpz_set_str(K.y,argv[4],10))return 2;
 K.infinity=0;check(oncurve(&K),"kernel membership");mul(&R,&K,ell);check(R.infinity,"kernel order divides ell");
 mpz_mul_2exp(index,first,1);mpz_add_ui(index,index,1);mul(&Q,&K,index);
 mpz_set_ui(z,2);mul(&step,&K,z);
 FILE *out=fopen(argv[7],"wbx");if(!out){perror("exclusive output");return 2;}
 for(unsigned long i=0;i<count;i++){
   check(!Q.infinity&&oncurve(&Q),"root point");
   unsigned char raw[32]={0};size_t used=0;
   unsigned char tmp[32];mpz_export(tmp,&used,1,1,1,0,Q.x);
   check(used<=32,"root encoding");memcpy(raw+32-used,tmp,used);
   if(fwrite(raw,1,32,out)!=32){perror("write root");return 2;}
   add(&R,&Q,&step);copy(&Q,&R);
 }
 if(fclose(out)){perror("close root chunk");return 2;}
 gmp_printf("{\"status\":\"chunk_written\",\"ell\":\"%Zd\",\"first_ordinal\":\"%Zd\",\"count\":%lu,\"bytes\":%lu,\"record_format\":\"32-byte unsigned big-endian x-coordinate\",\"kernel_index\":\"2*ordinal+1\",\"scope\":\"bounded root preparation; no quotient or map\"}\n",ell,first,count,32*count);
 clear(&K);clear(&Q);clear(&step);clear(&R);mpz_clears(p,ell,first,index,end,B,z,NULL);return 0;
}
