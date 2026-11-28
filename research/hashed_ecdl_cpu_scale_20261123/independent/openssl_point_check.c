/* Independent post-run point audit, using OpenSSL point operations only.
 * Public fixtures/targets, no private input and no scalar reduction by an
 * asserted order. The simple bit loop lets r*P be checked before supplying
 * any group-order metadata to OpenSSL. This is not a solver or timing arm.
 */
#include <openssl/bn.h>
#include <openssl/ec.h>
#include <openssl/err.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

static const uint64_t fixtures[6][6] = {
    {1048571,1048568,44,1048189,2,317355},
    {16777199,16777196,31,16782299,2,5412647},
    {268435399,268435396,3,268407199,1,1},
    {4294967291ULL,4294967288ULL,31,4294959973ULL,2,2120051858ULL},
    {68719476731ULL,68719476728ULL,33,68719022539ULL,2,51741851039ULL},
    {1099511627563ULL,1099511627560ULL,56,1099512551701ULL,1,15267301451ULL}
};
_Static_assert(sizeof(unsigned long) >= 8, "64-bit BN_ULONG build required");

static int mul_unreduced(const EC_GROUP *g, EC_POINT *out,
                         const EC_POINT *p, const BIGNUM *k, BN_CTX *ctx) {
    if (!EC_POINT_set_to_infinity(g,out)) return 0;
    for (int bit=BN_num_bits(k)-1;bit>=0;--bit) {
        if (!EC_POINT_dbl(g,out,out,ctx)) return 0;
        if (BN_is_bit_set(k,bit) && !EC_POINT_add(g,out,out,p,ctx)) return 0;
    }
    return 1;
}
#define VERIFY(c,msg) do { if (!(c)) { fprintf(stderr,"VERIFY %s\n",msg); rc=1; goto done; } } while(0)
#define LIB(c,msg) do { if (!(c)) { fprintf(stderr,"LIBRARY %s\n",msg); rc=2; goto done; } } while(0)

int main(int argc,char **argv) {
    if(argc!=12) { fprintf(stderr,"CASE SEED p a b r Gx Gy Qx Qy scalar\n"); return 64; }
    for(int j=1;j<argc;++j) {
        if(!argv[j][0] || strspn(argv[j],"0123456789")!=strlen(argv[j]) || strlen(argv[j])>20) return 64;
    }
    unsigned long which=strtoul(argv[1],NULL,10),seed=strtoul(argv[2],NULL,10);
    if(which>5 || (seed!=101 && seed!=202)) return 64;
    int rc=2;
    BN_CTX *ctx=BN_CTX_new(); BIGNUM *v[9]={0}; BIGNUM *x=BN_new();
    EC_GROUP *g=NULL; EC_POINT *P=NULL,*Q=NULL,*expected=NULL,*result=NULL;
    LIB(ctx && x,"allocation");
    for(int j=0;j<9;++j) LIB(BN_dec2bn(&v[j],argv[j+3])>0,"decimal parse");
    for(int j=0;j<6;++j) VERIFY(BN_num_bits(v[j])<=64 && BN_get_word(v[j])==fixtures[which][j],"BINDING fixture");
    VERIFY(BN_cmp(v[8],v[3])<0,"SCALAR canonical range");
    VERIFY(BN_cmp(v[6],v[0])<0 && BN_cmp(v[7],v[0])<0,"BINDING coordinate range");
    g=EC_GROUP_new_curve_GFp(v[0],v[1],v[2],ctx); LIB(g,"curve creation");
    LIB(EC_GROUP_check_discriminant(g,ctx)==1,"discriminant");
    P=EC_POINT_new(g); Q=EC_POINT_new(g); expected=EC_POINT_new(g); result=EC_POINT_new(g);
    LIB(P && Q && expected && result,"point allocation");
    LIB(EC_POINT_set_affine_coordinates(g,P,v[4],v[5],ctx)==1,"generator coordinates");
    VERIFY(EC_POINT_set_affine_coordinates(g,Q,v[6],v[7],ctx)==1,"BINDING target coordinates");
    VERIFY(EC_POINT_is_at_infinity(g,P)==0 && EC_POINT_is_at_infinity(g,Q)==0,"BINDING nonidentity");
    uint64_t xx=(uint64_t)seed*1009 % fixtures[which][0];
    int found=0;
    for(unsigned int tries=0;tries<256;++tries) {
        LIB(BN_set_word(x,(unsigned long)xx)==1,"target x");
        if(EC_POINT_set_compressed_coordinates(g,expected,x,0,ctx)==1) { found=1; break; }
        ERR_clear_error(); xx=(xx+1)%fixtures[which][0];
    }
    LIB(found,"public target search bound");
    int comparison=EC_POINT_cmp(g,expected,Q,ctx);
    LIB(comparison>=0,"target comparison");
    VERIFY(comparison==0,"BINDING public target");
    LIB(mul_unreduced(g,result,P,v[3],ctx),"order multiplication");
    VERIFY(EC_POINT_is_at_infinity(g,result)==1,"ORDER rP");
    LIB(mul_unreduced(g,result,P,v[8],ctx),"scalar multiplication");
    comparison=EC_POINT_cmp(g,result,Q,ctx);
    LIB(comparison>=0,"scalar comparison");
    VERIFY(comparison==0,"SCALAR kP=Q");
    puts("OPENSSL VERIFIED public-target binding, nonidentity rP=O and kP=Q"); rc=0;
done:
    EC_POINT_free(result); EC_POINT_free(expected); EC_POINT_free(Q); EC_POINT_free(P);
    EC_GROUP_free(g); BN_free(x); for(int j=0;j<9;++j) BN_free(v[j]); BN_CTX_free(ctx);
    return rc;
}
