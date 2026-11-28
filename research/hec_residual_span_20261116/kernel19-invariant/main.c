#include <stdio.h>
#include <assert.h>
#include <flint/flint.h>
#include <flint/acb.h>
#include <flint/acb_mat.h>
#include <flint/arb_mat.h>
#include <flint/acb_theta.h>
#include <flint/fmpz_mat.h>
#define BASE "research/hec_residual_span_20261116/"
static void symmetry_positive(acb_mat_t tau, slong prec){
 acb_t t; acb_init(t);
 acb_sub(t,acb_mat_entry(tau,0,1),acb_mat_entry(tau,1,0),prec); assert(acb_contains_zero(t));
 acb_add(t,acb_mat_entry(tau,0,1),acb_mat_entry(tau,1,0),prec); acb_mul_2exp_si(t,t,-1);
 acb_set(acb_mat_entry(tau,0,1),t);acb_set(acb_mat_entry(tau,1,0),t);
 arb_mat_t y,l;arb_mat_init(y,2,2);arb_mat_init(l,2,2);acb_mat_get_imag(y,tau);
 assert(arb_mat_cho(l,y,prec));for(int i=0;i<2;i++)assert(arb_is_positive(arb_mat_entry(l,i,i)));
 arb_mat_clear(y);arb_mat_clear(l);acb_clear(t);
}
static void exact_dump(FILE *f, acb_srcptr v, int n){
 for(int i=0;i<n;i++){assert(arb_dump_file(f,acb_realref(v+i))==0);fputc('\n',f);assert(arb_dump_file(f,acb_imagref(v+i))==0);fputc('\n',f);}
}
static void evaluate(const char *label,acb_ptr result){
 char path[512];snprintf(path,sizeof(path),BASE "kernel19-lattice-v2/%s.dat",label);
 FILE *f=fopen(path,"r");assert(f);unsigned long g,phi[2],den;
 assert(fscanf(f,"%lu %lu %lu %lu",&g,&phi[0],&phi[1],&den)==4);assert(g==2&&den>0);
 fmpz_mat_t c;fmpz_mat_init(c,4,4);char token[256];
 for(int i=0;i<4;i++)for(int j=0;j<4;j++){assert(fscanf(f,"%255s",token)==1);assert(fmpz_set_str(fmpz_mat_entry(c,i,j),token,10)==0);}fclose(f);
 acb_ptr previous=_acb_vec_init(23);
 for(slong prec=256;prec<=512;prec*=2){
 acb_t z,t,p,d;acb_init(z);acb_init(t);acb_init(p);acb_init(d);acb_unit_root(z,5,prec);
 acb_mat_t periods,a,b,tau;acb_mat_init(periods,2,4);acb_mat_init(a,2,2);acb_mat_init(b,2,2);acb_mat_init(tau,2,2);
 for(int i=0;i<2;i++)for(int j=0;j<4;j++)for(int k=0;k<4;k++){
 acb_pow_ui(p,z,phi[i]*k,prec);acb_mul_fmpz(t,p,fmpz_mat_entry(c,k,j),prec);acb_div_ui(t,t,den,prec);acb_add(acb_mat_entry(periods,i,j),acb_mat_entry(periods,i,j),t,prec);}
 for(int i=0;i<2;i++)for(int j=0;j<2;j++){acb_set(acb_mat_entry(a,i,j),acb_mat_entry(periods,i,2*j));acb_set(acb_mat_entry(b,i,j),acb_mat_entry(periods,i,2*j+1));}
 assert(acb_mat_solve(tau,a,b,prec));symmetry_positive(tau,prec);
 acb_ptr values=_acb_vec_init(23),th2=_acb_vec_init(16),zero=_acb_vec_init(2);
 acb_theta_all(values,zero,tau,0,prec);int even=0,odd=0;
 for(ulong ch=0;ch<16;ch++){assert(acb_is_finite(values+ch));acb_sqr(th2+ch,values+ch,prec);
 if(acb_theta_char_is_even(ch,2)){even++;assert(!acb_contains_zero(values+ch));}else{odd++;assert(acb_contains_zero(values+ch));}}
 assert(even==10&&odd==6);
 acb_theta_g2_even_weight(values+16,values+17,values+18,values+19,th2,prec);assert(!acb_contains_zero(values+18));
 /* Three weight-zero functions, denominator only chi10; not claimed to be separating. */
 const int numerator[3]={16,17,19};const ulong exponent[3]={2,3,6};
 for(int i=0;i<3;i++){acb_pow_ui(t,values+numerator[i],5,prec);acb_pow_ui(d,values+18,exponent[i],prec);acb_div(values+20+i,t,d,prec);}
 for(int i=0;i<23;i++){assert(acb_is_finite(values+i));if(prec==512)assert(acb_overlaps(values+i,previous+i));else acb_set(previous+i,values+i);}
 if(label[0]=='s'){assert(acb_contains_zero(values+16));assert(acb_contains_zero(values+17));assert(acb_contains_zero(values+19));}
 snprintf(path,sizeof(path),BASE "kernel19-invariant/%s-%ld.balls",label,prec);FILE *out=fopen(path,"wx");assert(out);exact_dump(out,values,23);assert(fclose(out)==0);
 /* Reload exact serialized balls and check containment equality in both directions. */
 FILE *in=fopen(path,"r");assert(in);acb_t loaded;acb_init(loaded);
 for(int i=0;i<23;i++){assert(arb_load_file(acb_realref(loaded),in)==0);assert(arb_load_file(acb_imagref(loaded),in)==0);assert(acb_equal(loaded,values+i));}fclose(in);acb_clear(loaded);
 printf("CONTROL label=%s prec=%ld even_nonzero=%d odd_zero_containing=%d positive_period=true chi10_nonzero=true exact_reload=true\n",label,prec,even,odd);
 for(int i=0;i<3;i++){printf("FUNCTION label=%s prec=%ld index=%d value=",label,prec,i);arb_printn(acb_realref(values+20+i),35,0);printf(" +i*");arb_printn(acb_imagref(values+20+i),35,0);printf("\n");if(prec==512)acb_set(result+i,values+20+i);}
 _acb_vec_clear(values,23);_acb_vec_clear(th2,16);_acb_vec_clear(zero,2);acb_clear(z);acb_clear(t);acb_clear(p);acb_clear(d);acb_mat_clear(periods);acb_mat_clear(a);acb_mat_clear(b);acb_mat_clear(tau);
 }
 _acb_vec_clear(previous,23);fmpz_mat_clear(c);
}
int main(void){
 const char *labels[5]={"source","q0","q1","q2","q3"};acb_ptr all=_acb_vec_init(15);
 printf("LIBRARY flint=%s\n",flint_version);for(int i=0;i<5;i++)evaluate(labels[i],all+3*i);
 int separated=0;for(int i=0;i<5;i++)for(int j=i+1;j<5;j++){int distinct=0;for(int k=0;k<3;k++)distinct|=!acb_overlaps(all+3*i+k,all+3*j+k);separated+=distinct;printf("PAIR left=%s right=%s certified_distinct=%d\n",labels[i],labels[j],distinct);}
 printf("SUMMARY separated_pairs=%d total_pairs=10\n",separated);_acb_vec_clear(all,15);flint_cleanup();fprintf(stderr,"PASS periods_theta_forms_precision_exact_reload=true algebraic_reconstruction=false\n");return 0;
}
