/* Independently check every stored root with PARI elliptic multiplication. */
#include <pari/pari.h>
#include <stdio.h>
int main(int argc,char **argv){
 if(argc!=2)return 2;
 FILE *f=fopen(argv[1],"rb");if(!f)return 2;
 pari_init(64000000,1000000);
 GEN p=strtoi("4794168233664648458988367110514316682522236917173341621535751");
 GEN E=ellinit(mkvec5(gen_0,gen_0,gen_0,stoi(-140),stoi(784)),p,DEFAULTPREC);
 GEN K=mkvec2(strtoi("4515130868018265073699728324389842779619221778507237352874816"),strtoi("1852242499917145718370025456583037063631410696703406330482828"));
 pari_sp keep=avma;unsigned char raw[32];unsigned long i=0;size_t n;
 while((n=fread(raw,1,32,f))==32){
   GEN x=gen_0;for(unsigned j=0;j<32;j++)x=addis(shifti(x,8),raw[j]);
   GEN Q=ellmul(E,K,utoi(2*i+1));
   if(lg(Q)!=3||!equalii(lift(gel(Q,1)),x)){fprintf(stderr,"root mismatch %lu\n",i);return 1;}
   avma=keep;i++;
 }
 if(n||ferror(f)||i!=4096){fprintf(stderr,"wrong fixture length\n");return 1;}
 fclose(f);printf("{\"status\":\"pass\",\"independent_backend\":\"PARI\",\"roots_verified\":%lu,\"first_ordinal\":0,\"scope\":\"fixed 4096-root storage fixture; no isogeny evaluation\"}\n",i);
 pari_close();return 0;
}
