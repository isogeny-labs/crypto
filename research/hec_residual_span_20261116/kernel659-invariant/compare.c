#include <assert.h>
#include <stdio.h>
#include <flint/acb.h>
static void read_value(acb_t out,FILE*fp){assert(arb_load_file(acb_realref(out),fp)==0);assert(arb_load_file(acb_imagref(out),fp)==0);}
int main(void){
 acb_t values[3][2][2];const char*labels[]={"source","q55_1","q12_555"};
 for(int i=0;i<3;i++)for(int p=0;p<2;p++){char path[512];snprintf(path,sizeof(path),"research/hec_residual_span_20261116/kernel659-invariant/%s.%d.invariant.balls",labels[i],p==0?256:512);FILE*fp=fopen(path,"r");assert(fp);for(int j=0;j<2;j++){acb_init(values[i][p][j]);read_value(values[i][p][j],fp);assert(acb_is_finite(values[i][p][j]));}fclose(fp);}
 for(int i=0;i<3;i++)for(int j=0;j<2;j++)assert(acb_overlaps(values[i][0][j],values[i][1][j]));
 for(int p=0;p<2;p++)for(int j=0;j<2;j++)assert(acb_overlaps(values[1][p][j],values[2][p][j]));
 for(int i=0;i<3;i++)for(int p=0;p<2;p++)for(int j=0;j<2;j++)acb_clear(values[i][p][j]);
 puts("PASS exact_ball_reload=true both_invariants_cross_precision_overlap=true isometric_partners_overlap=true algebraic_recognition_not_performed=true p421_valuation_not_computed=true");flint_cleanup();return 0;
}
