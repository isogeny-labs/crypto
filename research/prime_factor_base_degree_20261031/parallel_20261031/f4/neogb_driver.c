/* Thin GPLv2+ msolve-neogb adapter. No upstream algorithm source is modified.
 * API sequence follows upstream export_f4 at pinned source revision.
 * Build instructions/provenance: README.md and build-neogb.sh. */
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <sys/resource.h>
#include "libneogb.h"

static void fail(const char *message) { fprintf(stderr,"adapter: %s\n",message); exit(2); }
static int read_int(FILE *f) { int x; if(fscanf(f,"%d",&x)!=1) fail("invalid integer input"); return x; }
int main(int argc,char **argv) {
    if(argc!=2) fail("usage: neogb-driver INPUT.tokens");
    FILE *f=fopen(argv[1],"r");if(!f)fail("cannot open input");
    int p=read_int(f),nv=read_int(f),ng=read_int(f);
    if(p<5||p>65521||nv<1||nv>3||ng<1||ng>256)fail("input outside frozen limits");
    int32_t *lens=calloc(ng,sizeof(int32_t));
    int32_t *exps=calloc(8192*nv,sizeof(int32_t));
    int32_t *cfs=calloc(8192,sizeof(int32_t));
    if(!lens||!exps||!cfs)fail("allocation");
    int nt=0;
    for(int i=0;i<ng;i++) {
        lens[i]=read_int(f);if(lens[i]<0||nt+lens[i]>8192)fail("term limit");
        for(int j=0;j<lens[i];j++,nt++) {
            cfs[nt]=read_int(f);if(cfs[nt]<1||cfs[nt]>=p)fail("noncanonical coefficient");
            for(int k=0;k<nv;k++){int e=read_int(f);if(e<0||e>128)fail("exponent limit");exps[nt*nv+k]=e;}
        }
    }
    char tail;if(fscanf(f," %c",&tail)==1)fail("trailing input");fclose(f);
    /* Mode 2 selects exact sparse elimination, not probabilistic pivots.
       Pin libc PRNG defensively as well; the full CLI's clock seed is not linked. */
    srand(315);
    bs_t *bs=NULL;ht_t *ht=NULL;md_t *md=NULL;
    const double cpu_start=cputime(),real_start=realtime();
    int initialized=initialize_gba_input_data(&bs,&ht,&md,lens,exps,cfs,p,
        0,0,nv,ng,0,12,1,0,0,2,0,1,0,0,2);
    if(initialized!=1)fail("upstream initialization rejected input");
    int err=0;bs=core_f4(bs,md,&err,p);if(err)fail("upstream F4 failed");
    int32_t load=0,*lengths=NULL,*basis_exp=NULL;void *basis_cf=NULL;
    int64_t nterms=export_results_from_f4(&load,&lengths,&basis_exp,&basis_cf,malloc,&bs,&ht,&md);
    md->f4_ctime=cputime()-cpu_start;md->f4_rtime=realtime()-real_start;
    /* core_f4 already printed its local-context statistics. The export context
       does not retain these counters; never report its zeros as F4 workload. */
    struct rusage usage;if(getrusage(RUSAGE_SELF,&usage))fail("getrusage");
    printf("{\"schema\":\"msolve-neogb-adapter-result/v1\",\"algorithm\":\"stock_msolve_neogb_f4\",\"source_revision\":\"3672113ee2c696a2ac9c637636e0f7214f500ba1\",\"p\":%d,\"variables\":%d,\"la_option\":2,\"linear_algebra_mode\":\"exact_sparse\",\"random_seed\":315,\"threads\":1,\"reduced_basis\":[",p,nv);
    int64_t pos=0;
    for(int i=0;i<load;i++){
        if(i)putchar(',');putchar('[');
        for(int j=0;j<lengths[i];j++,pos++){
            if(j)putchar(',');printf("{\"coefficient\":%d,\"exponents\":[",((int32_t*)basis_cf)[pos]);
            for(int k=0;k<nv;k++){if(k)putchar(',');printf("%d",basis_exp[pos*nv+k]);}
            printf("]}");
        }putchar(']');
    }
    printf("],\"basis_terms\":%lld,\"basis_polynomials\":%d,\"pairs_reduced\":null,\"rows_reduced\":null,\"zero_reductions\":null,\"largest_area_matrix_rows\":null,\"largest_area_matrix_columns\":null,\"counter_source\":\"core_stderr_parsed_by_native_runner\",\"algorithm_elapsed_seconds\":%.9f,\"algorithm_cpu_seconds\":%.9f,\"process_peak_rss_kib\":%ld,\"process_input_blocks\":%ld,\"process_output_blocks\":%ld,\"field_multiplications\":null,\"field_additions\":null,\"timing_qualification\":\"L0_shared_host_diagnostic_only\"}\n",
        (long long)nterms,load,md->f4_rtime,md->f4_ctime,usage.ru_maxrss,usage.ru_inblock,usage.ru_oublock);
    free_f4_julia_result_data(free,&lengths,&basis_exp,&basis_cf,load,p);
    free_shared_hash_data(ht);if(bs)free_basis(&bs);free(md);free(lens);free(exps);free(cfs);
    return 0;
}
