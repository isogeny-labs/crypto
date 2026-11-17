/* Native seeded DP diagnostic and independent OpenSSL point-law checks.
 * Samples are per namespace, never per timing repetition. No timing claim.
 * Build separately for BEST2=0/1. --sample is a research run, not a unit test. */
#include "nist_rho.cuh"
#include <openssl/ec.h>
#include <openssl/obj_mac.h>
#include <openssl/bn.h>
#include <array>
#include <iostream>
#include <stdexcept>
#include <vector>
#include <string>
#include <memory>
#include <cstdint>
#include <cstdlib>

static void require(bool ok,const char* msg){if(!ok)throw std::runtime_error(msg);}
struct Oracle {
 EC_GROUP* g; BN_CTX* ctx=BN_CTX_new();
 explicit Oracle(int nid):g(EC_GROUP_new_by_curve_name(nid)){require(g&&ctx,"OpenSSL allocation");}
 ~Oracle(){EC_GROUP_free(g);BN_CTX_free(ctx);}
 template<class F> BIGNUM* bn(const typename F::elt&x){
  auto a=F::to_canonical(x); unsigned char bytes[4*F::N];
  for(int i=0;i<F::N;i++)for(int j=0;j<4;j++)bytes[4*i+j]=(unsigned char)(a.v[i]>>(8*j));
  return BN_lebin2bn(bytes,sizeof bytes,nullptr);
 }
 template<class F> EC_POINT* point(const nap<F>&p){
  EC_POINT* q=EC_POINT_new(g); require(q,"point allocation");
  if(p.inf)require(EC_POINT_set_to_infinity(g,q),"infinity");
  else {auto x=bn<F>(p.x),y=bn<F>(p.y);require(EC_POINT_set_affine_coordinates(g,q,x,y,ctx),"input on curve");BN_free(x);BN_free(y);}
  return q;
 }
 template<class F> nap<F> native(const EC_POINT*q){
  nap<F> p{};p.inf=EC_POINT_is_at_infinity(g,q);if(p.inf)return p;
  auto x=BN_new(),y=BN_new();require(EC_POINT_get_affine_coordinates(g,q,x,y,ctx),"affine coordinates");
  auto cvt=[](const BIGNUM*b){unsigned char bytes[4*F::N];uint32_t limbs[F::N]{};require(BN_bn2lebinpad(b,bytes,sizeof bytes)==(int)sizeof bytes,"BN width");
   for(int i=0;i<F::N;i++)for(int j=0;j<4;j++)limbs[i]|=(uint32_t)bytes[4*i+j]<<(8*j);
   return F::from_limbs(limbs);};
  p.x=cvt(x);p.y=cvt(y);BN_free(x);BN_free(y);return p;
 }
 template<class C> void check(const nap<typename C::Field>&p,const nap<typename C::Field>&m,
                              const nap<typename C::Field>&got,int mode,int aut,const rho_params&prm){
  using F=typename C::Field;
  auto a=point<F>(p),b=point<F>(m),plus=EC_POINT_new(g),minus=EC_POINT_new(g);
  bool dbl=mode==RHO_MODE_ESCAPE||mode==RHO_MODE_DOUBLE;
  require(dbl?EC_POINT_dbl(g,plus,a,ctx):EC_POINT_add(g,plus,a,b,ctx),"independent add");
  auto expected=native<F>(plus);int code=0;
  if(RHO_BEST2&&mode==RHO_MODE_ADD){
   require(EC_POINT_invert(g,b,ctx)&&EC_POINT_add(g,minus,a,b,ctx),"independent subtract");
   auto candidate=native<F>(minus);
   if(rho_score<F>(candidate.x)>rho_score<F>(expected.x)){expected=candidate;code=RHO_AUT_MINUS;}
  }
  code|=rho_canonical<C>(expected,prm);
  require(!expected.inf&&!got.inf&&F::eq(expected.x,got.x)&&F::eq(expected.y,got.y),"OpenSSL step differs");
  require(code==aut,"subtract/fold replay code differs");
  EC_POINT_free(a);EC_POINT_free(b);EC_POINT_free(plus);EC_POINT_free(minus);
 }
};

struct Counts {
 uint64_t attempts=0,advanced=0,detect=0,seek=0,inf=0,escape=0,dp=0;
 uint64_t pairs=0,plus_dp=0,minus_dp=0,selected_dp=0,minus=0;
 std::array<uint64_t,16> bucket{},bucket_dp{};
};
template<class C> void run(const char*name,int nid,uint64_t seed,int W,int iters,int dpbits,bool oracle,int64_t fixed_table){
 using F=typename C::Field;using Pt=nap<F>;
 Oracle ref(nid);Pt G=ref.native<F>(EC_GROUP_get0_generator(ref.g));
 uint32_t seven[F::N]{};seven[0]=7;auto Q=C::scalar_mul(G,seven,3);
 rho_params prm{};prm.r_bits=8;prm.dp_mask=(1u<<dpbits)-1;prm.fold=RHO_FOLD_NEG;
 prm.max_steps=1u<<28;prm.order_bits=F::N*32;
 // Seed IDs are reproducible pseudorandom namespaces, not a proof of independence.
 uint64_t rng=seed;prm.walk_seed=rho_splitmix64(rng);prm.table_seed=(uint32_t)rho_splitmix64(rng);
 if(fixed_table>=0)prm.table_seed=(uint32_t)fixed_table;
 std::vector<Pt> tbl(256);
 for(uint32_t j=0;j<256;j++){uint32_t a[F::N],b[F::N];rho_table_seed<F::N>(prm.table_seed,j,a,b,prm.order_bits);tbl[j]=C::double_scalar_mul(G,a,Q,b,prm.order_bits);require(!tbl[j].inf,"infinite table entry");}
 std::vector<rho_state<C>> st(W);std::vector<uint32_t> restarts(W),steps(W);
 rho_ctx<C> ctx{};ctx.G=G;ctx.Q=Q;ctx.prm=prm;ctx.restarts=restarts.data();
 for(int w=0;w<W;w++)rho_seed_walk<C>(ctx,w,st[w]);
 std::vector<uint32_t> X(F::N*W),Y(F::N*W),H(RHO_CYCLE_DEPTH*W),esc(W),bs(W),br(W);
 std::vector<rho_dp<F>> dps(oracle?W*iters:1);uint32_t dpc=0;
 rho_ctx<C> batch=ctx;batch.X=X.data();batch.Y=Y.data();batch.H=H.data();batch.esc=esc.data();batch.steps=bs.data();batch.restarts=br.data();
 batch.nthreads=1;batch.walks_per_thread=W;batch.table=tbl.data();batch.dp_out=dps.data();batch.dp_count=&dpc;batch.dp_cap=(uint32_t)dps.size();
 if(oracle)for(int w=0;w<W;w++){br[w]=restarts[w];rho_store<C>(batch,w,st[w]);}
 std::vector<typename F::elt> den(W),scratch(W);std::vector<int> mode(W);std::vector<uint32_t> j(W);
 Counts n;
 for(int it=0;it<iters;it++){
  for(int w=0;w<W;w++){mode[w]=rho_phase_a<C>(st[w],tbl.data(),prm,den[w],j[w]);if(mode[w]==RHO_MODE_INF)den[w]=F::one();}
  F::batch_inv(den.data(),W,scratch.data());
  for(int w=0;w<W;w++){
   n.attempts++;
   if(mode[w]==RHO_MODE_INF){n.inf++;restarts[w]++;steps[w]=0;rho_seed_walk<C>(ctx,w,st[w]);continue;}
   auto before=st[w].P;
   if(mode[w]==RHO_MODE_ADD){
    auto plus=C::affine_add_with_inv(before,tbl[j[w]],den[w],0);
    auto neg=tbl[j[w]];neg.y=F::neg(neg.y);
    auto minus=C::affine_add_with_inv(before,neg,den[w],0);
    auto selected=rho_score<F>(minus.x)>rho_score<F>(plus.x)?minus:plus;
    n.pairs++;n.plus_dp+=rho_is_dp<C>(plus,prm);n.minus_dp+=rho_is_dp<C>(minus,prm);
    n.selected_dp+=rho_is_dp<C>(selected,prm);
    unsigned bucket=rho_score<F>(selected.x)>>28;n.bucket[bucket]++;n.bucket_dp[bucket]+=rho_is_dp<C>(selected,prm);
   }
   int aut=0;int status=rho_phase_b<C>(st[w],tbl.data(),prm,mode[w],j[w],den[w],&aut);
   if(oracle)ref.check<C>(before,tbl[j[w]],st[w].P,mode[w],aut,prm);
   n.minus+=(aut&RHO_AUT_MINUS)!=0;n.escape+=mode[w]==RHO_MODE_ESCAPE;
   if(status<0){n.detect++;continue;}if(status==RHO_STEP_SEEK){n.seek++;continue;}
   n.advanced++;steps[w]++;
   if(rho_is_dp<C>(st[w].P,prm)){
    if(oracle){auto&d=dps[n.dp];for(int l=0;l<F::N;l++)d.x[l]=st[w].P.x.v[l];d.walk=w;d.restart=restarts[w];d.steps=steps[w];}
    n.dp++;restarts[w]++;steps[w]=0;rho_seed_walk<C>(ctx,w,st[w]);}
   else if(steps[w]>=prm.max_steps){restarts[w]++;steps[w]=0;rho_seed_walk<C>(ctx,w,st[w]);}
  }
  if(oracle){
   // Batch emitter must report the same exact DP identity, not just an on-curve x.
   std::vector<rho_dp<F>> expected(dps.begin()+dpc,dps.begin()+n.dp);
   uint32_t prior=dpc;rho_batch_step<C>(batch,0,den.data(),scratch.data());
   require(dpc==n.dp,"batch DP count differs");
   for(uint32_t d=prior;d<dpc;d++){
    const auto&e=expected[d-prior];const auto&a=dps[d];
    require(e.walk==a.walk&&e.restart==a.restart&&e.steps==a.steps,"DP replay identity differs");
    for(int l=0;l<F::N;l++)require(e.x[l]==a.x[l],"DP coordinate differs");
   }
   for(int w=0;w<W;w++){rho_state<C> b;rho_load<C>(batch,w,b);
    require(F::eq(b.P.x,st[w].P.x)&&F::eq(b.P.y,st[w].P.y)&&b.escape==st[w].escape,"batch state differs");
    require(bs[w]==steps[w]&&br[w]==restarts[w],"batch steps/restarts differ");
    for(int h=0;h<RHO_CYCLE_DEPTH;h++)require(b.h[h]==st[w].h[h],"batch history differs");
   }
  }
 }
 require(n.attempts==n.advanced+n.detect+n.seek+n.inf,"accounting partition");
 std::cout<<"{\"schema\":\"nist.seeded_dp/v1\",\"curve\":\""<<name<<"\",\"best2\":"<<RHO_BEST2
 <<",\"seed_id\":"<<seed<<",\"walk_seed\":"<<prm.walk_seed<<",\"table_seed\":"<<prm.table_seed
 <<",\"W\":"<<W<<",\"iters\":"<<iters<<",\"dp_bits\":"<<dpbits<<",\"attempts\":"<<n.attempts
 <<",\"advanced\":"<<n.advanced<<",\"cycle_detect\":"<<n.detect<<",\"seek\":"<<n.seek<<",\"inf\":"<<n.inf
 <<",\"escape_double\":"<<n.escape<<",\"emitted_dp\":"<<n.dp<<",\"pairs\":"<<n.pairs
 <<",\"plus_dp\":"<<n.plus_dp<<",\"minus_dp\":"<<n.minus_dp<<",\"selected_dp\":"<<n.selected_dp
 <<",\"minus_taken\":"<<n.minus<<",\"openssl_steps_checked\":"<<(oracle?n.attempts-n.inf:0)
 <<",\"score_buckets\":[";
 for(int b=0;b<16;b++)std::cout<<(b?",":"")<<"["<<n.bucket[b]<<","<<n.bucket_dp[b]<<"]";
 std::cout<<"]}\n";
}
int main(int argc,char**argv){try{
 bool sample=argc>1&&std::string(argv[1])=="--sample";
 require(argc==1||(sample&&(argc==7||argc==8)),"usage: test_best2 [--sample SEED COUNT W ITERS DPBITS [TABLE_SEED]]");
 uint64_t seed=sample?std::stoull(argv[2]):20261027;int count=sample?std::stoi(argv[3]):2;
 int W=sample?std::stoi(argv[4]):16,iters=sample?std::stoi(argv[5]):32,bits=sample?std::stoi(argv[6]):8;
 require(count>0&&count<=10000&&W>0&&W<=64&&iters>0&&bits>0&&bits<=24,"invalid bounds");
 int64_t fixed=argc==8?(int64_t)std::stoul(argv[7]):-1;
 for(int i=0;i<count;i++){
  run<CurveP256>("P-256",NID_X9_62_prime256v1,seed+i,W,iters,bits,!sample,fixed);
  run<CurveP384>("P-384",NID_secp384r1,seed+i,W,iters,bits,!sample,fixed);
 }
 return 0;
 }catch(const std::exception&e){std::cerr<<e.what()<<"\n";return 1;}}
