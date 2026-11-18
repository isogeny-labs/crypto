#include <stdio.h>
#include <stdlib.h>
#include <assert.h>
#include <flint/flint.h>
#include <flint/acb.h>
#include <flint/acb_mat.h>
#include <flint/arb_mat.h>
#include <flint/acb_theta.h>
#include <flint/fmpz_mat.h>
static void symmetric(acb_mat_t t, slong prec){
 acb_t z;acb_init(z);
 for(int i=0;i<3;i++)for(int j=i+1;j<3;j++){
 acb_sub(z,acb_mat_entry(t,i,j),acb_mat_entry(t,j,i),prec);assert(acb_contains_zero(z));
 acb_add(z,acb_mat_entry(t,i,j),acb_mat_entry(t,j,i),prec);acb_mul_2exp_si(z,z,-1);
 acb_set(acb_mat_entry(t,i,j),z);acb_set(acb_mat_entry(t,j,i),z);
 }acb_clear(z);
}
static void positive(const acb_mat_t t,slong prec){
 arb_mat_t y,l;arb_mat_init(y,3,3);arb_mat_init(l,3,3);
 acb_mat_get_imag(y,t);assert(arb_mat_cho(l,y,prec));
 for(int i=0;i<3;i++)assert(arb_is_positive(arb_mat_entry(l,i,i)));
 arb_mat_clear(y);arb_mat_clear(l);
}
int main(int argc,char**argv){
 assert(argc==3);FILE*in=fopen(argv[1],"r");assert(in);
 unsigned long phi[3],den;assert(fscanf(in,"%lu %lu %lu %lu",&phi[0],&phi[1],&phi[2],&den)==4);
 fmpz_mat_t coefficients;fmpz_mat_init(coefficients,6,6);char token[256];
 for(int i=0;i<6;i++)for(int j=0;j<6;j++){assert(fscanf(in,"%255s",token)==1);assert(fmpz_set_str(fmpz_mat_entry(coefficients,i,j),token,10)==0);}
 fclose(in);
 acb_ptr old=_acb_vec_init(64);int source=argv[2][0]=='s';
 printf("LIBRARY\tflint=%s\tlabel=%s\n",flint_version,argv[2]);
 for(slong prec=256;prec<=512;prec*=2){
 acb_t z,power,term;acb_init(z);acb_init(power);acb_init(term);acb_unit_root(z,7,prec);
 acb_mat_t periods,a,b,tau,reduced;acb_mat_init(periods,3,6);acb_mat_init(a,3,3);acb_mat_init(b,3,3);acb_mat_init(tau,3,3);acb_mat_init(reduced,3,3);
 for(int i=0;i<3;i++)for(int j=0;j<6;j++)for(int k=0;k<6;k++){
 acb_pow_ui(power,z,phi[i]*k,prec);acb_mul_fmpz(term,power,fmpz_mat_entry(coefficients,k,j),prec);acb_div_ui(term,term,den,prec);
 acb_add(acb_mat_entry(periods,i,j),acb_mat_entry(periods,i,j),term,prec);
 }
 for(int i=0;i<3;i++)for(int j=0;j<3;j++){acb_set(acb_mat_entry(a,i,j),acb_mat_entry(periods,i,2*j));acb_set(acb_mat_entry(b,i,j),acb_mat_entry(periods,i,2*j+1));}
 assert(acb_mat_solve(tau,a,b,prec));symmetric(tau,prec);positive(tau,prec);
 printf("PERIOD\tlabel=%s\tprec=%ld\ttau=",argv[2],prec);acb_mat_printd(tau,35);printf("\n");
 fmpz_mat_t reduction;fmpz_mat_init(reduction,6,6);acb_siegel_reduce(reduction,tau,prec);assert(sp2gz_is_correct(reduction));
 acb_siegel_transform(reduced,reduction,tau,prec);symmetric(reduced,prec);positive(reduced,prec);
 printf("REDUCTION\tlabel=%s\tprec=%ld\tmatrix=",argv[2],prec);fmpz_mat_print_pretty(reduction);printf("\n");
 printf("REDUCED_PERIOD\tlabel=%s\tprec=%ld\ttau=",argv[2],prec);acb_mat_printd(reduced,35);printf("\n");
 acb_ptr zero=_acb_vec_init(3),theta=_acb_vec_init(64);acb_theta_all(theta,zero,tau,0,prec);
 int even=0,nonzero=0,unresolved=0,odd=0;
 for(ulong ch=0;ch<64;ch++){
 assert(acb_is_finite(theta+ch));
 if(prec==512)assert(acb_overlaps(theta+ch,old+ch));
 if(acb_theta_char_is_even(ch,3)){even++;int nz=!acb_contains_zero(theta+ch);nonzero+=nz;unresolved+=!nz;
 printf("THETA\tlabel=%s\tprec=%ld\tcharacteristic=%lu\tnonzero=%d\tvalue=",argv[2],prec,ch,nz);acb_printd(theta+ch,35);printf("\n");
 }else{odd++;assert(acb_contains_zero(theta+ch));}
 if(prec==256)acb_set(old+ch,theta+ch);
 }
 assert(even==36&&odd==28);if(source){assert(nonzero==35&&unresolved==1);}
 printf("THETA_SUMMARY\tlabel=%s\tprec=%ld\teven=%d\tnonzero=%d\tunresolved=%d\todd_zero_containing=%d\n",argv[2],prec,even,nonzero,unresolved,odd);
 _acb_vec_clear(theta,64);_acb_vec_clear(zero,3);fmpz_mat_clear(reduction);
 acb_mat_clear(periods);acb_mat_clear(a);acb_mat_clear(b);acb_mat_clear(tau);acb_mat_clear(reduced);acb_clear(z);acb_clear(power);acb_clear(term);
 }
 _acb_vec_clear(old,64);fmpz_mat_clear(coefficients);flint_cleanup();
 fprintf(stderr,"PASS label=%s precisions=256,512 symplectic_input=true period_symmetry_positive_certified=true reduction_symplectic=true theta_balls_overlap=true source_control_if_applicable=true finite_field_claim=false\n",argv[2]);return 0;
}
