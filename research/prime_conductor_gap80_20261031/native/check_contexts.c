/* Independent PARI check of the 32 fixed non-kernel scalar contexts. */
#include <pari/pari.h>
#include <stdio.h>
#include <string.h>
int main(int argc,char **argv){
 if(argc!=2&&!(argc==3&&!strcmp(argv[2],"giants")))return 2;
 FILE *f=fopen(argv[1],"rb");if(!f)return 2;pari_init(64000000,1000000);
 GEN p=strtoi("4794168233664648458988367110514316682522236917173341621535751");
 GEN E=ellinit(mkvec5(gen_0,gen_0,gen_0,stoi(-140),stoi(784)),p,DEFAULTPREC);
 GEN G=mkvec2(strtoi("603209182794240622510892080705207481693427956682909708617837"),strtoi("1682738350748939452891832813322046124324576766734042502997852"));
 GEN ell=strtoi("729845429276318409687041");
 GEN B=sqrti(shifti(subis(ell,1),-2));
 if(argc==3){
   G=mkvec2(strtoi("4515130868018265073699728324389842779619221778507237352874816"),strtoi("1852242499917145718370025456583037063631410696703406330482828"));
   if(lg(ellmul(E,G,ell))!=2)return 1;
 }else if(lg(ellmul(E,G,strtoi("1125023250121")))!=2||lg(ellmul(E,G,ell))!=3)return 1;
 pari_sp keep=avma;
 for(unsigned i=1;i<=32;i++){
   unsigned char b[32];if(fread(b,1,32,f)!=32)return 1;
   GEN x=gen_0;for(int j=0;j<32;j++)x=addis(shifti(x,8),b[j]);
   GEN scalar=argc==3?mului(4*(i-1)+2,B):utoi(i);
   GEN Q=ellmul(E,G,scalar);if(lg(Q)!=3||!equalii(lift(gel(Q,1)),x))return 1;
   avma=keep;
 }
 if(fgetc(f)!=EOF||ferror(f))return 1;
 fclose(f);pari_close();
 if(argc==3)puts("{\"status\":\"pass\",\"backend\":\"PARI\",\"actual_giant_contexts_verified\":32,\"index_rule\":\"(4*i+2)*B on K; i=0..31\"}");
 else puts("{\"status\":\"pass\",\"backend\":\"PARI\",\"known_scalar_contexts_verified\":32,\"generator_q_order_checked\":true,\"generator_not_in_ell_kernel_checked\":true}");
 return 0;
}
