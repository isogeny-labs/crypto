/* Fixed public, known-scalar non-kernel contexts for development checks. */
#define main historical_search80_main
#include "search80.c"
#undef main
#include <string.h>
int main(int argc,char **argv){
 if(argc!=2&&!(argc==3&&!strcmp(argv[2],"giants")))return 2;
 mpz_init_set_str(p,"4794168233664648458988367110514316682522236917173341621535751",10);curve_b=784;
 point G,Q;init(&G);init(&Q);G.infinity=0;
 mpz_set_str(G.x,"603209182794240622510892080705207481693427956682909708617837",10);
 mpz_set_str(G.y,"1682738350748939452891832813322046124324576766734042502997852",10);
 if(argc==3){
   mpz_set_str(G.x,"4515130868018265073699728324389842779619221778507237352874816",10);
   mpz_set_str(G.y,"1852242499917145718370025456583037063631410696703406330482828",10);
 }
 check(oncurve(&G),"source generator");
 FILE *f=fopen(argv[1],"wbx");if(!f)return 2;
 mpz_t n,B;mpz_inits(n,B,NULL);
 mpz_set_str(B,"729845429276318409687041",10);mpz_sub_ui(B,B,1);mpz_fdiv_q_2exp(B,B,2);mpz_sqrt(B,B);
 for(unsigned i=1;i<=32;i++){
   if(argc==3)mpz_mul_ui(n,B,4*(i-1)+2);else mpz_set_ui(n,i);
   mul(&Q,&G,n);check(!Q.infinity&&oncurve(&Q),"point context");
   unsigned char raw[32]={0},tmp[32];size_t used;mpz_export(tmp,&used,1,1,1,0,Q.x);memcpy(raw+32-used,tmp,used);
   if(fwrite(raw,1,32,f)!=32)return 2;
 }
 if(fclose(f))return 2;
 mpz_clears(n,B,NULL);clear(&G);clear(&Q);mpz_clear(p);return 0;
}
