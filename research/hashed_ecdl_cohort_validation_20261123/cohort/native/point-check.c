/* Prospective cohort checker. Reuse the frozen bounded admission implementation,
 * but never invoke its original census/main. This caller keeps state.out NULL.
 * Target construction uses OpenSSL Euler/square-root operations independently
 * of the Rust decoder. Scalar replay uses explicit addition/doubling. */
#define main admission_original_main
#include "../../native/admission.c"
#undef main

static const Input fixture_domains[] = {
    {.p=1048571,.a=1048568,.b=44,.r=1048189,.gx=2,.gy=317355},
    {.p=16777199,.a=16777196,.b=31,.r=16782299,.gx=2,.gy=5412647},
    {.p=268435399,.a=268435396,.b=3,.r=268407199,.gx=1,.gy=1},
    {.p=4294967291,.a=4294967288,.b=31,.r=4294959973,.gx=2,.gy=2120051858},
    {.p=68719476731,.a=68719476728,.b=33,.r=68719022539,.gx=2,.gy=51741851039},
    {.p=1099511627563,.a=1099511627560,.b=56,.r=1099512551701,.gx=1,.gy=15267301451}
};

static void reject(const char *cause) {
    fprintf(stderr, "REJECTED %s\n", cause);
    exit(1);
}

static bool recognized_seed(uint64_t seed) {
    static const uint64_t seeds[] = {101,202,303,404,505,606,707,808};
    for (unsigned k=0; k<8; ++k) if (seeds[k]==seed) return true;
    return false;
}

static Input public_fixture(uint64_t domain, uint64_t seed) {
    if (domain>=6 || !recognized_seed(seed)) reject("unrecognized fixture");
    Input in=fixture_domains[domain];
    in.domain=(int)domain; in.seed=seed;
    (void)snprintf(in.id,sizeof(in.id),"case%" PRIu64 "-seed%" PRIu64,domain,seed);
    in.qx=in.gx; in.qy=in.gy;
    Result base=validate(&in);
    require(base.code==CERTIFIED && base.eligible,"fixed domain admission");
    BN_CTX *ctx=BN_CTX_new();
    BIGNUM *p=word_bn(in.p), *exponent=word_bn((in.p-1)/2);
    BIGNUM *minus_one=word_bn(in.p-1), *value=BN_new(), *legendre=BN_new(), *root=BN_new();
    require(ctx!=NULL && value!=NULL && legendre!=NULL && root!=NULL,
            "target workspace allocation");
    uint64_t x=(seed*1009)%in.p;
    bool found=false;
    for (uint64_t tries=0; tries<in.p; ++tries) {
        uint64_t f=rhs(x,in.a,in.b,in.p);
        require(BN_set_word(value,(BN_ULONG)f)==1,"target field import");
        require(BN_mod_exp(legendre,value,exponent,p,ctx)==1,"Euler criterion");
        if (f==0 || BN_is_one(legendre)) {
            require(BN_mod_sqrt(root,value,p,ctx)!=NULL,"target square root");
            uint64_t y=(uint64_t)BN_get_word(root);
            require(y<in.p && mulmod(y,y,in.p)==f,"target square-root postcondition");
            if ((y&1U)!=0) y=in.p-y;
            in.qx=x; in.qy=y; found=true; break;
        }
        require(BN_cmp(legendre,minus_one)==0,"unexpected Euler classification");
        x=(x+1)%in.p;
    }
    BN_free(root); BN_free(legendre); BN_free(value); BN_free(minus_one);
    BN_free(exponent); BN_free(p); BN_CTX_free(ctx);
    require(found,"no public target within complete x-search");
    Result checked=validate(&in);
    require(checked.code==CERTIFIED && checked.eligible,"derived target admission");
    return in;
}

static void print_header(const Input *in) {
    require(printf("%s\t%d\t%" PRIu64 "\t%" PRIu64 "\t%" PRIu64
            "\t%" PRIu64 "\t%" PRIu64 "\t%" PRIu64 "\t%" PRIu64
            "\t%" PRIu64 "\t%" PRIu64 "\n",in->id,in->domain,in->seed,
            in->p,in->a,in->b,in->r,in->gx,in->gy,in->qx,in->qy)>0,
            "public header output");
}

static void scalar_check(const Input *in, uint64_t scalar) {
    if (scalar>=in->r) reject("noncanonical scalar");
    BN_CTX *ctx=BN_CTX_new();
    BIGNUM *p=word_bn(in->p),*a=word_bn(in->a),*b=word_bn(in->b);
    BIGNUM *gx=word_bn(in->gx),*gy=word_bn(in->gy),*qx=word_bn(in->qx),*qy=word_bn(in->qy);
    require(ctx!=NULL,"scalar context allocation");
    EC_GROUP *group=EC_GROUP_new_curve_GFp(p,a,b,ctx);
    require(group!=NULL,"scalar curve allocation");
    const BIGNUM *order=EC_GROUP_get0_order(group);
    require(order==NULL || BN_is_zero(order),"unexpected scalar group order");
    const EC_POINT *configured=EC_GROUP_get0_generator(group);
    require(configured==NULL || EC_POINT_is_at_infinity(group,configured)==1,
            "unexpected scalar library generator");
    EC_POINT *base=EC_POINT_new(group),*sum=EC_POINT_new(group),*target=EC_POINT_new(group);
    require(base!=NULL && sum!=NULL && target!=NULL,"scalar point workspace");
    require(EC_POINT_set_affine_coordinates(group,base,gx,gy,ctx)==1
            && EC_POINT_set_affine_coordinates(group,target,qx,qy,ctx)==1
            && EC_POINT_set_to_infinity(group,sum)==1,"scalar point import");
    while (scalar!=0) {
        if ((scalar&1U)!=0) require(EC_POINT_add(group,sum,sum,base,ctx)==1,"scalar add");
        require(EC_POINT_dbl(group,base,base,ctx)==1,"scalar double");
        scalar>>=1;
    }
    int equal=EC_POINT_cmp(group,sum,target,ctx);
    require(equal>=0,"scalar point comparison error");
    EC_POINT_free(target); EC_POINT_free(sum); EC_POINT_free(base); EC_GROUP_free(group);
    BN_free(qy); BN_free(qx); BN_free(gy); BN_free(gx); BN_free(b); BN_free(a); BN_free(p);
    BN_CTX_free(ctx);
    if (equal!=0) reject("kP != Q");
}

int main(int argc,char **argv) {
    if (argc==4 && strcmp(argv[1],"target")==0) {
        Input in=public_fixture(parse_uint(argv[2]),parse_uint(argv[3]));
        print_header(&in); return 0;
    }
    if (argc!=12) {
        fprintf(stderr,"usage: point-check target CASE SEED | CASE SEED P A B R GX GY QX QY SCALAR\n");
        return 2;
    }
    Input in=public_fixture(parse_uint(argv[1]),parse_uint(argv[2]));
    const uint64_t expected[]={in.p,in.a,in.b,in.r,in.gx,in.gy,in.qx,in.qy};
    for (unsigned k=0;k<8;++k) {
        if (parse_uint(argv[k+3])!=expected[k]) reject("public target mismatch");
    }
    scalar_check(&in,parse_uint(argv[11]));
    puts("OPENSSL VERIFIED public-target binding, nonidentity rP=O and kP=Q");
    return 0;
}
