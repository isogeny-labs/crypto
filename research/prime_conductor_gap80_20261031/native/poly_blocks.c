/* Native bounded block products and additive reciprocal-power sums.
 * Fixed frozen gap80 field. No global polynomial or isogeny is constructed. */
#include <gmp.h>
#include <flint/flint.h>
#include <flint/fmpz.h>
#include <flint/fmpz_mod.h>
#include <flint/fmpz_mod_poly.h>
#include <flint/fmpz_vec.h>
#include <stdio.h>
#include <stdlib.h>
#include <stdint.h>
#include <string.h>
#define ROOT_CAP 65536
#define POINT_CAP 64
static fmpz_t prime;
static fmpz_mod_ctx_t ctx;
static void die(const char *s){fprintf(stderr,"poly_blocks: %s\n",s);exit(2);}
static int read_value(FILE *f,fmpz_t z){
 unsigned char raw[32];size_t n=fread(raw,1,32,f);if(!n&&!ferror(f))return 0;
 if(n!=32)die("truncated coordinate/coefficient");
 mpz_t t;mpz_init(t);mpz_import(t,32,1,1,1,0,raw);fmpz_set_mpz(z,t);mpz_clear(t);
 if(fmpz_cmp(z,prime)>=0)die("noncanonical field value");
 return 1;
}
static void write_value(FILE *f,const fmpz_t z){
 unsigned char raw[32]={0},tmp[32];size_t n=0;mpz_t t;mpz_init(t);fmpz_get_mpz(t,z);
 mpz_export(tmp,&n,1,1,1,0,t);mpz_clear(t);if(n>32)die("coefficient overflow");
 memcpy(raw+32-n,tmp,n);if(fwrite(raw,1,32,f)!=32)die("write failed");
}
static fmpz *values(const char *name,slong cap,slong *len){
 FILE *f=fopen(name,"rb");if(!f)die("cannot open values");
 fmpz *v=_fmpz_vec_init(cap);fmpz_t tmp;fmpz_init(tmp);*len=0;
 while(read_value(f,tmp)){if(*len==cap)die("value count exceeds bound");fmpz_set(v+(*len)++,tmp);}
 fclose(f);fmpz_clear(tmp);if(!*len)die("empty values");return v;
}
static void build(const char *roots,const char *out){
 slong n;fmpz *r=values(roots,ROOT_CAP,&n);fmpz_mod_poly_t h;fmpz_mod_poly_init(h,ctx);
 fmpz_mod_poly_product_roots_fmpz_vec(h,r,n,ctx);
 FILE *f=fopen(out,"wbx");if(!f)die("output exists or cannot be created");
 unsigned char degree[4]={(unsigned char)(n>>24),(unsigned char)(n>>16),(unsigned char)(n>>8),(unsigned char)n};
 if(fwrite("G80POLY1",1,8,f)!=8||fwrite(degree,1,4,f)!=4)die("header write failed");
 write_value(f,prime);fmpz_t c;fmpz_init(c);
 for(slong i=0;i<=n;i++){fmpz_mod_poly_get_coeff_fmpz(c,h,i,ctx);write_value(f,c);}
 if(fclose(f))die("close failed");
 fmpz_clear(c);fmpz_mod_poly_clear(h,ctx);_fmpz_vec_clear(r,ROOT_CAP);
 printf("{\"status\":\"block_written\",\"degree\":%ld,\"bytes\":%ld}\n",n,44+32*(n+1));
}
static slong load_poly(const char *name,fmpz_mod_poly_t h){
 FILE *f=fopen(name,"rb");if(!f)die("cannot open polynomial block");
 unsigned char magic[8],d[4];if(fread(magic,1,8,f)!=8||memcmp(magic,"G80POLY1",8)||fread(d,1,4,f)!=4)die("invalid block header");
 uint32_t n=((uint32_t)d[0]<<24)|((uint32_t)d[1]<<16)|((uint32_t)d[2]<<8)|d[3];
 if(!n||n>ROOT_CAP)die("block degree exceeds bound");
 unsigned char field[32];if(fread(field,1,32,f)!=32)die("truncated field header");
 mpz_t t;mpz_init(t);mpz_import(t,32,1,1,1,0,field);fmpz_t c;fmpz_init(c);fmpz_set_mpz(c,t);mpz_clear(t);
 if(!fmpz_equal(c,prime))die("block field mismatch");
 for(uint32_t i=0;i<=n;i++){if(!read_value(f,c))die("truncated polynomial");fmpz_mod_poly_set_coeff_fmpz(h,i,c,ctx);}
 if(!fmpz_is_one(c)||fgetc(f)!=EOF||ferror(f))die("nonmonic or extra polynomial data");
 fclose(f);fmpz_clear(c);return n;
}
static void accumulate(fmpz *sums,const fmpz *xs,slong np,const char *name,uint64_t *total){
 fmpz_mod_poly_t h,d;fmpz_mod_poly_init(h,ctx);fmpz_mod_poly_init(d,ctx);slong n=load_poly(name,h);
 if(UINT64_MAX-*total<(uint64_t)n)die("root-count overflow");
 *total+=n;
 fmpz *ev[7];for(int k=0;k<7;k++){
   ev[k]=_fmpz_vec_init(np);fmpz_mod_poly_evaluate_fmpz_vec_fast(ev[k],h,xs,np,ctx);
   fmpz_mod_poly_derivative(d,h,ctx);fmpz_mod_poly_swap(d,h,ctx);
 }
 fmpz_t fact,invfact[7],inv,hv[7],L[6],z,tmp;fmpz_init(fact);fmpz_one(fact);fmpz_init(inv);fmpz_init(z);fmpz_init(tmp);
 for(int k=0;k<7;k++){fmpz_init(invfact[k]);fmpz_init(hv[k]);if(k)fmpz_mul_ui(fact,fact,k);if(!fmpz_invmod(invfact[k],fact,prime))die("factorial inverse");}
 for(int k=0;k<6;k++)fmpz_init(L[k]);
 for(slong i=0;i<np;i++){
   if(!fmpz_invmod(inv,ev[0]+i,prime))die("evaluation point collides with a root");
   fmpz_one(hv[0]);
   for(int k=1;k<7;k++){fmpz_mul(z,ev[k]+i,invfact[k]);fmpz_mul(z,z,inv);fmpz_mod(hv[k],z,prime);}
   for(int k=0;k<6;k++){
     fmpz_mul_ui(z,hv[k+1],k+1);
     for(int j=1;j<=k;j++){fmpz_mul(tmp,hv[j],L[k-j]);fmpz_sub(z,z,tmp);}
     fmpz_mod(L[k],z,prime);if(k%2)fmpz_neg(z,L[k]);else fmpz_set(z,L[k]);
     fmpz_add(z,z,sums+6*i+k);fmpz_mod(sums+6*i+k,z,prime);
   }
 }
 for(int k=0;k<7;k++){_fmpz_vec_clear(ev[k],np);fmpz_clear(invfact[k]);fmpz_clear(hv[k]);}
 for(int k=0;k<6;k++)fmpz_clear(L[k]);
 fmpz_clear(fact);fmpz_clear(inv);fmpz_clear(z);fmpz_clear(tmp);fmpz_mod_poly_clear(h,ctx);fmpz_mod_poly_clear(d,ctx);
}
static void oracle(fmpz *sums,const fmpz *xs,slong np,const char *name,uint64_t *total){
 slong nr;fmpz *roots=values(name,ROOT_CAP,&nr);*total=nr;fmpz_t z,inv,power;fmpz_init(z);fmpz_init(inv);fmpz_init(power);
 for(slong i=0;i<np;i++)for(slong j=0;j<nr;j++){
   fmpz_sub(z,xs+i,roots+j);if(!fmpz_invmod(inv,z,prime))die("oracle point collides with root");fmpz_one(power);
   for(int k=0;k<6;k++){fmpz_mul(z,power,inv);fmpz_mod(power,z,prime);fmpz_add(z,sums+6*i+k,power);fmpz_mod(sums+6*i+k,z,prime);}
 }
 _fmpz_vec_clear(roots,ROOT_CAP);fmpz_clear(z);fmpz_clear(inv);fmpz_clear(power);
}
int main(int argc,char **argv){
 if(argc<4){fprintf(stderr,"usage: poly_blocks build ROOTS OUT | eval POINTS POLY... | oracle POINTS ROOTS\n");return 2;}
 fmpz_init(prime);fmpz_set_str(prime,"4794168233664648458988367110514316682522236917173341621535751",10);fmpz_mod_ctx_init(ctx,prime);
 if(!strcmp(argv[1],"build")&&argc==4)build(argv[2],argv[3]);
 else if(!strcmp(argv[1],"eval")||(!strcmp(argv[1],"oracle")&&argc==4)){
   slong np;fmpz *xs=values(argv[2],POINT_CAP,&np),*sums=_fmpz_vec_init(6*np);uint64_t total=0;
   if(!strcmp(argv[1],"oracle"))oracle(sums,xs,np,argv[3],&total);
   else for(int i=3;i<argc;i++)accumulate(sums,xs,np,argv[i],&total);
   printf("{\"root_count\":%llu,\"point_count\":%ld,\"reciprocal_sums\":[",(unsigned long long)total,np);
   for(slong i=0;i<np;i++){printf("%s[",i?",":"");for(int k=0;k<6;k++){printf("%s\"",k?",":"");fmpz_print(sums+6*i+k);printf("\"");}printf("]");}printf("]}\n");
   _fmpz_vec_clear(xs,POINT_CAP);_fmpz_vec_clear(sums,6*np);
 }else die("invalid command");
 fmpz_mod_ctx_clear(ctx);fmpz_clear(prime);flint_cleanup();return 0;
}
