// Separately versioned quadratic-twist transport of the complete evaluator.
// Default d=1 preserves the original evaluator; auxiliary kernels stay typed.
#include <gmpxx.h>
#include <flint/flint.h>
#include <flint/fmpz.h>
#include <flint/fmpz_mod.h>
#include <flint/fmpz_mod_poly.h>
#include <flint/fmpz_poly.h>
#include <flint/fmpz_vec.h>
#undef ulong
#include <pari/pari.h>
#include <array>
#include <optional>
#include <vector>
#include <string>
#include <sstream>
#include <iostream>
#include <stdexcept>
#include <algorithm>
#include <limits>
using Z=mpz_class;
static Z p,a,b;
static fmpz_mod_ctx_t ctx;
extern "C" char *optimized(const char*,const char*,const char*,const char*,const char*,const char*);
extern "C" void optimized_free(void*);
static void require(bool x,const char*s){if(!x)throw std::runtime_error(s);}
static Z mod(Z x){mpz_mod(x.get_mpz_t(),x.get_mpz_t(),p.get_mpz_t());return x;}
static Z inverse(Z x){Z y;require(mpz_invert(y.get_mpz_t(),x.get_mpz_t(),p.get_mpz_t()),"inverse failed");return y;}
struct Point{Z x=0,y=0;bool inf=true;};
static Point add(Point u,Point v){
 if(u.inf)return v;
 if(v.inf)return u;
 if(u.x==v.x&&mod(u.y+v.y)==0)return {};
 Z m=u.x==v.x?mod((3*u.x*u.x+a)*inverse(mod(2*u.y))):mod((v.y-u.y)*inverse(mod(v.x-u.x)));
 Z x=mod(m*m-u.x-v.x);return {x,mod(m*(u.x-x)-u.y),false};
}
static Point mul(Point u,Z n){require(n>=0,"negative scalar");Point r;while(n>0){if(mpz_odd_p(n.get_mpz_t()))r=add(r,u);u=add(u,u);n>>=1;}return r;}
static bool on(Point u,Z A,Z B){return u.inf||mod(u.y*u.y-u.x*u.x*u.x-A*u.x-B)==0;}
#include "projective_walk.hpp"
struct Poly{
 fmpz_mod_poly_t z;
 Poly(){fmpz_mod_poly_init(z,ctx);}
 Poly(const Poly&q):Poly(){fmpz_mod_poly_set(z,q.z,ctx);}
 Poly(Poly&&q):Poly(){fmpz_mod_poly_swap(z,q.z,ctx);}
 Poly&operator=(const Poly&q){if(this!=&q)fmpz_mod_poly_set(z,q.z,ctx);return *this;}
 Poly&operator=(Poly&&q){if(this!=&q)fmpz_mod_poly_swap(z,q.z,ctx);return *this;}
 ~Poly(){fmpz_mod_poly_clear(z,ctx);}
};
static Poly constant(Z x){Poly r;fmpz_t t;fmpz_init(t);x=mod(x);fmpz_set_mpz(t,x.get_mpz_t());fmpz_mod_poly_set_coeff_fmpz(r.z,0,t,ctx);fmpz_clear(t);return r;}
static Poly operator+(const Poly&u,const Poly&v){Poly r;fmpz_mod_poly_add(r.z,u.z,v.z,ctx);return r;}
static Poly operator-(const Poly&u,const Poly&v){Poly r;fmpz_mod_poly_sub(r.z,u.z,v.z,ctx);return r;}
static Poly operator*(const Poly&u,const Poly&v){Poly r;fmpz_mod_poly_mul(r.z,u.z,v.z,ctx);return r;}
static Poly scale(const Poly&u,Z s){return u*constant(s);}
using Jet=std::array<Poly,4>;
#include "prepared_norm.hpp"
static Jet identity(){Jet r;r[0]=constant(1);return r;}
static Jet product(const Jet&u,const Jet&v){Jet r;for(int k=0;k<4;k++)for(int j=0;j<=k;j++)r[k]=r[k]+u[j]*v[k-j];return r;}
static void ingest(std::vector<std::optional<Jet>>&bins,Jet v){size_t k=0;while(k<bins.size()&&bins[k]){v=product(*bins[k],v);bins[k].reset();k++;}if(k==bins.size())bins.emplace_back();bins[k]=std::move(v);}
static Jet finish(std::vector<std::optional<Jet>>&bins){Jet r=identity();for(auto i=bins.rbegin();i!=bins.rend();++i)if(*i)r=product(r,**i);return r;}
static std::string encode(const Poly&u){fmpz_poly_t f;fmpz_poly_init(f);fmpz_mod_poly_get_fmpz_poly(f,u.z,ctx);char*s=fmpz_poly_get_str(f);std::string r(s);flint_free(s);fmpz_poly_clear(f);return r;}
static std::optional<std::array<Z,3>> norm(const Poly&h,const Jet&v){
 Poly g;fmpz_mod_poly_gcd(g.z,h.z,v[0].z,ctx);if(fmpz_mod_poly_degree(g.z,ctx)>0)return {};
 std::string hs=encode(h),ps=p.get_str();std::array<std::string,4> t;for(int i=0;i<4;i++)t[i]=encode(v[i]);
 char*out=optimized(ps.c_str(),hs.c_str(),t[0].c_str(),t[1].c_str(),t[2].c_str(),t[3].c_str());require(out,"native log-norm backend failed");
 std::istringstream s(out);std::array<Z,3> r;for(auto&x:r){s>>x;require(!s.fail(),"bad native output");}optimized_free(out);return r;
}
// Canonical traversal journal: reject gaps, duplicates and wrong extents.
struct Coverage{
 unsigned long B,G,block,i=0,j=0;Z covered=0,left=0,expected_left;
 Coverage(unsigned long B_,unsigned long G_,unsigned long z,Z n):B(B_),G(G_),block(z),expected_left(n-2*Z(B_)*G_){}
 void tile(unsigned long x,unsigned long nx,unsigned long y,unsigned long ny){
   require(i<B&&x==i&&y==j,"coverage gap/duplicate/order");
   require(nx==std::min(block,B-i)&&ny==std::min(block,G-j),"coverage extent");
   covered+=2*Z(nx)*ny;j+=ny;if(j==G){j=0;i+=nx;}
 }
 void leftover(Z ordinal){require(i==B&&j==0&&ordinal==left&&left<expected_left,"leftover coverage");left++;}
 void complete(Z n){require(i==B&&j==0&&left==expected_left&&covered+left==n,"incomplete coverage; assembly refused");}
};
struct Work{Z baby_products=0,baby_starts=0,baby_additions=0,giant_starts=0,giant_additions=0,leftover_starts=0,leftover_additions=0,tiles=0,norm_calls=0,prepared_builds=0,prepared_calls=0;WalkDiagnostics walks;};
struct Result{Z ca,cb;std::vector<Point> images;Z covered;unsigned long B,G;Work work;};
struct RootVector{
 fmpz* data;slong size;
 explicit RootVector(unsigned long n):data(nullptr),size(0){require(n<=static_cast<unsigned long>(std::numeric_limits<slong>::max()),"root-vector width");size=static_cast<slong>(n);data=_fmpz_vec_init(size);}
 RootVector(const RootVector&)=delete;RootVector&operator=(const RootVector&)=delete;
 ~RootVector(){_fmpz_vec_clear(data,size);}
};
static Poly baby_product(Point K,Z start,unsigned long count,Point step,Work&work,unsigned long batch){
 RootVector roots(count);Point T=mul(K,start);work.baby_starts++;
 for_each_walk_x(T,step,count,batch,work.walks,[&](unsigned long k,const Z&x){fmpz_set_mpz(roots.data+k,x.get_mpz_t());});
 if(count>0)work.baby_additions+=count-1;
 Poly h;fmpz_mod_poly_product_roots_fmpz_vec(h.z,roots.data,roots.size,ctx);work.baby_products++;return h;
}
static Result construct(Z ell,Point K,const std::vector<Point>&points,unsigned long block,unsigned long max_window=std::numeric_limits<unsigned long>::max(),Z baby_window=0,unsigned long projective_batch=0,bool prepared=false,Z twist=1){
 require(ell>=3&&mpz_odd_p(ell.get_mpz_t())&&block>0,"invalid degree/block");
 require(twist>0&&twist<p,"noncanonical or zero twist scale");
 Z untwist=inverse(twist),source_a=mod(twist*twist*a),source_b=mod(twist*twist*twist*b);
 Z n=(ell-1)/2,rad=n/2,bs;mpz_sqrt(bs.get_mpz_t(),rad.get_mpz_t());if(bs<1)bs=1;
 if(baby_window!=0){require(baby_window>0&&2*baby_window<=n,"baby window outside coverage range");bs=baby_window;}
 require(mpz_fits_ulong_p(bs.get_mpz_t()),"window index exceeds native width");unsigned long B=bs.get_ui();Z gs=n/(2*bs);require(mpz_fits_ulong_p(gs.get_mpz_t()),"giant index width");unsigned long G=gs.get_ui();
 require(G>0,"use direct path for degree below five");require(B<=max_window&&G<=max_window,"window exceeds declared limit; assembly refused");
 require(on(K,a,b)&&!K.inf&&mul(K,ell).inf,"kernel witness");Coverage coverage(B,G,block,n);Work work;
 std::vector<std::array<Z,3>> sums(points.size()+1);std::vector<bool> kernel(points.size());for(size_t k=0;k<points.size();k++){require(on(points[k],source_a,source_b),"source membership");kernel[k]=points[k].inf;}
 Poly z;fmpz_mod_poly_set_coeff_ui(z.z,1,1,ctx);
 Point baby_step=mul(K,2),giant_step=mul(K,4*Z(B));
 for(unsigned long i=0;i<B;i+=std::min(block,B-i)){
  unsigned long ni=std::min(block,B-i);Poly h=baby_product(K,2*Z(i)+1,ni,baby_step,work,projective_batch);
  std::optional<PreparedNorm> cached;
  if(prepared){cached.emplace(h);work.prepared_builds++;}
  for(unsigned long j=0;j<G;j+=std::min(block,G-j)){
   unsigned long nj=std::min(block,G-j);
   std::vector<std::vector<std::optional<Jet>>> bins(points.size()+1);
   Point T=mul(K,(4*Z(j)+2)*B);work.giant_starts++;
   for_each_walk_x(T,giant_step,nj,projective_batch,work.walks,[&](unsigned long,const Z& x){
     Poly f0=(z-constant(x))*(z-constant(x));
     Poly f1=scale((scale(z,x)+constant(a))*(z+constant(x)),-2)-constant(4*b);
     Poly f2=(scale(z,x)-constant(a))*(scale(z,x)-constant(a))-scale(z+constant(x),4*b);
     Jet v;v[0]=f0;v[1]=f1;v[2]=f2;ingest(bins[0],v);
     for(size_t q=0;q<points.size();q++)if(!kernel[q]){
       Z alpha=mod(points[q].x*untwist);Jet w;w[0]=scale(f0,alpha*alpha)+scale(f1,alpha)+f2;w[1]=scale(f0,2*alpha)+f1;w[2]=f0;ingest(bins[q+1],w);
     }
   });
   if(nj>0)work.giant_additions+=nj-1;
   for(size_t q=0;q<sums.size();q++)if(q==0||!kernel[q-1]){
     work.norm_calls++;
     Jet jet=finish(bins[q]);
     auto v=cached?cached->evaluate(jet):norm(h,jet);if(cached)work.prepared_calls++;
     if(!v){require(q>0,"infinity-jet collision");kernel[q-1]=true;}else for(int k=0;k<3;k++)sums[q][k]=mod(sums[q][k]+(*v)[k]);
   }
   coverage.tile(i,ni,j,nj);work.tiles++;
  }
 }
 std::array<Z,3> M={mod(-sums[0][0]),mod(-2*sums[0][1]),mod(-3*sums[0][2])};
 Z ordinal=0;
 Point leftover;if(4*Z(B)*G+1<ell){leftover=mul(K,4*Z(B)*G+1);work.leftover_starts++;}
 for(Z index=4*Z(B)*G+1;index<ell;index+=2){
   require(!leftover.inf,"leftover infinity");Z x=leftover.x,x2=mod(x*x);M[0]=mod(M[0]+x);M[1]=mod(M[1]+x2);M[2]=mod(M[2]+x2*x);
   for(size_t q=0;q<points.size();q++)if(!kernel[q]){
     Z alpha=mod(points[q].x*untwist);if(alpha==x){kernel[q]=true;continue;}Z r=inverse(mod(alpha-x)),r2=mod(r*r);
     sums[q+1][0]=mod(sums[q+1][0]+r);sums[q+1][1]=mod(sums[q+1][1]-r2*inverse(2));sums[q+1][2]=mod(sums[q+1][2]+r2*r*inverse(3));
   }
   coverage.leftover(ordinal++);
   if(index+2<ell){leftover=add(leftover,baby_step);work.leftover_additions++;}
 }
 coverage.complete(n); // No quotient or images may be emitted before this gate.
 Result out;out.ca=mod(twist*twist*(a-10*(3*M[1]+a*n)));out.cb=mod(twist*twist*twist*(b-14*(5*M[2]+3*a*M[0]+2*b*n)));out.covered=n;out.B=B;out.G=G;out.work=work;
 for(size_t q=0;q<points.size();q++){
   if(kernel[q]){out.images.push_back({});continue;}
   Z x=mod(points[q].x*untwist),y=points[q].y,fx=mod(x*x*x+a*x+b),df=mod(3*x*x+a);auto d=sums[q+1];
   Point H{mod(twist*(ell*x-2*M[0]-2*df*d[0]-8*fx*d[1])),mod(y*(ell-12*x*d[0]-12*df*d[1]-24*fx*d[2])),false};require(on(H,out.ca,out.cb),"destination membership");out.images.push_back(H);
 }
 return out;
}
static GEN gen(const Z&z){return strtoi(z.get_str().c_str());}
static Z integer(GEN z){char*s=GENtostr(lift(z));Z r(s);pari_free(s);return r;}
static GEN gp(Point u){return u.inf?mkvec(gen_0):mkvec2(gen(u.x),gen(u.y));}
static bool equal(Point u,GEN v){if(u.inf)return lg(v)==2;return lg(v)==3&&integer(gel(v,1))==u.x&&integer(gel(v,2))==u.y;}
static unsigned long bound(const char*s){Z n(s);require(n>0&&mpz_fits_ulong_p(n.get_mpz_t()),"invalid positive native bound");return n.get_ui();}
static void print_point(Point P){if(P.inf)std::cout<<"null";else std::cout<<"[\""<<P.x<<"\",\""<<P.y<<"\"]";}
// Explicitly supplied public cases only; no parameter search or timing arm.
static void evaluate_case(unsigned long max_window,unsigned long block,bool compare,Z baby_window,unsigned long projective_batch,bool prepared,Z twist){
 Z ell;Point K;unsigned long count;
 require(bool(std::cin>>p>>a>>b>>ell>>K.x>>K.y>>count),"case header: p a b ell Kx Ky point_count");K.inf=false;
 require(p>3&&ell>=5&&ell!=p,"invalid characteristic/degree");
 require(isprime(gen(p))&&isprime(gen(ell)),"nonprime characteristic/degree");
 require(twist>0&&twist<p,"noncanonical or zero twist scale");
 a=mod(a);b=mod(b);require(mod(4*a*a*a+27*b*b)!=0,"singular source");
 require(K.x>=0&&K.x<p&&K.y>=0&&K.y<p,"noncanonical kernel coordinates");require(count<=64,"at most 64 point inputs");
 std::vector<Point> points;points.reserve(count);
 for(unsigned long i=0;i<count;i++){
   std::string token;require(bool(std::cin>>token),"missing point input");
   if(token=="O"){points.push_back({});continue;}
   Point Q;Q.x=Z(token);Q.inf=false;require(bool(std::cin>>Q.y),"missing point ordinate");
   require(Q.x>=0&&Q.x<p&&Q.y>=0&&Q.y<p,"noncanonical point coordinates");points.push_back(Q);
 }
 std::string extra;require(!(std::cin>>extra),"extra input after declared points");
 // Independent direct reference expansion is deliberately bounded.
 require(!compare||ell<=10009,"PARI direct-reference degree cap exceeded");
 fmpz_t prime;fmpz_init(prime);fmpz_set_mpz(prime,p.get_mpz_t());fmpz_mod_ctx_init(ctx,prime);
 try{
   Result out=construct(ell,K,points,block,max_window,baby_window,projective_batch,prepared,twist);
   if(compare){
     GEN E=ellinit(mkvec5(gen_0,gen_0,gen_0,gen(a),gen(b)),gen(p),DEFAULTPREC),kernel=gp(K);
     if(twist!=1){
       GEN h=pol_1(0);Z n=(ell-1)/2;
       for(unsigned long i=1;i<=n.get_ui();i++){
         GEN R=ellmul(E,gp(K),utoi(i));
         h=gmul(h,gsub(pol_x(0),gmul(gen(twist),gel(R,1))));
       }
       kernel=h;E=ellinit(mkvec5(gen_0,gen_0,gen_0,gen(mod(twist*twist*a)),gen(mod(twist*twist*twist*b))),gen(p),DEFAULTPREC);
     }
     GEN ref=ellisogeny(E,kernel,0,0,1);
     GEN coeff=gel(ref,1),map=gel(ref,2);
     require(integer(gel(coeff,4))==out.ca&&integer(gel(coeff,5))==out.cb,"PARI codomain mismatch");
     for(size_t i=0;i<points.size();i++)require(equal(out.images[i],ellisogenyapply(map,gp(points[i]))),"PARI image mismatch");
   }
   std::cout<<"{\"status\":\"complete_evaluation\",\"p\":\""<<p<<"\",\"source\":[\""<<mod(twist*twist*a)<<"\",\""<<mod(twist*twist*twist*b)<<"\"],\"ell\":\""<<ell<<"\",\"kernel\":";
   if(twist==1)print_point(K);else{
     std::cout<<"null,\"twist_transport\":{\"version\":\"quadratic_twist_v1\",\"scale\":\""<<twist<<"\",\"auxiliary_source\":[\""<<a<<"\",\""<<b<<"\"],\"auxiliary_rational_kernel\":";print_point(K);std::cout<<"}";
   }
   std::cout<<",\"codomain\":[\""<<out.ca<<"\",\""<<out.cb<<"\"],\"baby_count\":"<<out.B<<",\"giant_count\":"<<out.G<<",\"block\":"<<block<<",\"max_window\":"<<max_window<<",\"kernel_pairs_covered\":\""<<out.covered<<"\",\"images\":[";
   for(size_t i=0;i<points.size();i++){if(i)std::cout<<",";std::cout<<"{\"input\":";print_point(points[i]);std::cout<<",\"image\":";print_point(out.images[i]);std::cout<<"}";}
   const auto&w=out.work;
   std::cout<<"],\"work_counts\":{\"baby_products\":\""<<w.baby_products<<"\",\"baby_scalar_starts\":\""<<w.baby_starts<<"\",\"baby_recurrence_additions\":\""<<w.baby_additions<<"\",\"giant_scalar_starts\":\""<<w.giant_starts<<"\",\"giant_recurrence_additions\":\""<<w.giant_additions<<"\",\"leftover_scalar_starts\":\""<<w.leftover_starts<<"\",\"leftover_recurrence_additions\":\""<<w.leftover_additions<<"\",\"rectangles\":\""<<w.tiles<<"\",\"lognorm_calls\":\""<<w.norm_calls<<"\"},\"pari_direct_reference\":"<<(compare?"true":"false")<<",\"ordinary_status\":null,\"conductor_gap\":null,\"runtime_measurements\":null";
   if(projective_batch)std::cout<<",\"walk_backend\":{\"name\":\"bounded_projective\",\"batch_cap\":"<<projective_batch<<",\"batch_inverse_calls\":\""<<w.walks.batch_inverse_calls<<"\",\"projective_additions\":\""<<w.walks.projective_additions<<"\",\"projective_points\":\""<<w.walks.projective_points<<"\",\"peak_batch_points\":"<<w.walks.peak_batch_points<<"}";
   if(prepared)std::cout<<",\"norm_backend\":{\"name\":\"prepared_flint_v1\",\"modulus_preparations\":\""<<w.prepared_builds<<"\",\"evaluations\":\""<<w.prepared_calls<<"\",\"text_conversions\":0}";
   std::cout<<"}"<<std::endl;
 }catch(...){fmpz_mod_ctx_clear(ctx);fmpz_clear(prime);throw;}
 fmpz_mod_ctx_clear(ctx);fmpz_clear(prime);
}
static void fixture(const char*ps,unsigned long degree,const char*kx,const char*ky,unsigned long q,unsigned long block){
 p=Z(ps);a=5;b=5;fmpz_t prime;fmpz_init(prime);fmpz_set_mpz(prime,p.get_mpz_t());fmpz_mod_ctx_init(ctx,prime);
 try{
 Point K{Z(kx),Z(ky),false};GEN E=ellinit(mkvec5(gen_0,gen_0,gen_0,gen(a),gen(b)),gen(p),DEFAULTPREC);Z N=integer(ellcard(E,nullptr));
 require(N%q==0,"subgroup divisor");require(mod(p+1-N)!=0,"supersingular fixture");
 require(integer(ellorder(E,gp(K),nullptr))==degree,"independent kernel order");Point base;
 for(unsigned long x=0;x<10000&&base.inf;x++){
   Z xx=x,rhs=mod(xx*xx*xx+a*xx+b),y,exp=(p+1)/4;mpz_powm(y.get_mpz_t(),rhs.get_mpz_t(),exp.get_mpz_t(),p.get_mpz_t());
   if(mod(y*y)!=rhs)continue;
   Point C{xx,y,false};Point G=mul(C,N/q);if(!G.inf&&mul(G,q).inf)base=G;
 }
 require(!base.inf,"subgroup generator missing");
 unsigned targets=std::min<unsigned long>(32,q-1);std::vector<Point> points={base,K,mul(K,2),{}};
 for(unsigned i=1;i<=targets;i++)points.push_back(mul(base,i));
 Result out=construct(degree,K,points,block);
 GEN ref=ellisogeny(E,gp(K),0,0,1);GEN coeff=gel(ref,1),map=gel(ref,2);
 require(integer(gel(coeff,4))==out.ca&&integer(gel(coeff,5))==out.cb,"PARI codomain mismatch");
 for(size_t i=0;i<points.size();i++)require(equal(out.images[i],ellisogenyapply(map,gp(points[i]))),"PARI image mismatch");
 GEN C=ellinit(coeff,gen(p),DEFAULTPREC);require(gequal(ellcard(E,nullptr),ellcard(C,nullptr)),"order changed");
 Point H=out.images[0];require(!H.inf,"transported generator zero");require(lg(ellmul(C,gp(H),utoi(q)))==2,"transported order");
 for(unsigned i=1;i<=targets;i++){
   auto Q=points[i+3],R=out.images[i+3];require(equal(R,ellmul(C,gp(H),utoi(i))),"homomorphism known-scalar check");
   Z source_answer=integer(elllog(E,gp(Q),gp(base),utoi(q))),destination_answer=integer(elllog(C,gp(R),gp(H),utoi(q)));
   require(source_answer==i&&destination_answer==i,"paired scalar recovery");
   std::cout<<"{\"type\":\"paired_known_answer_control\",\"p\":\""<<p<<"\",\"degree\":\""<<degree<<"\",\"expected_scalar\":\""<<i<<"\",\"source_answer\":\""<<source_answer<<"\",\"destination_answer\":\""<<destination_answer<<"\",\"source_target\":[\""<<Q.x<<"\",\""<<Q.y<<"\"],\"destination_target\":[\""<<R.x<<"\",\""<<R.y<<"\"],\"runtime_measurements\":null}"<<std::endl;
 }
 std::cout<<"{\"status\":\"pass\",\"scope\":\"complete ordinary-curve correctness fixture, not a conductor-gap record or timing experiment\",\"p\":\""<<p<<"\",\"degree\":\""<<degree<<"\",\"q\":\""<<q<<"\",\"baby_count\":"<<out.B<<",\"giant_count\":"<<out.G<<",\"kernel_pairs_expected\":\""<<out.covered<<"\",\"coverage_complete\":true,\"codomain\":[\""<<out.ca<<"\",\""<<out.cb<<"\"],\"pari_images_verified\":"<<points.size()<<",\"kernel_and_infinity_controls\":3,\"paired_known_scalars_recovered\":"<<targets<<",\"ecdlp_endpoint_recoveries\":"<<2*targets<<",\"runtime_measurements\":null}"<<std::endl;
 }catch(...){fmpz_mod_ctx_clear(ctx);fmpz_clear(prime);throw;}
 fmpz_mod_ctx_clear(ctx);fmpz_clear(prime);
}
static void coverage_tests(){
 unsigned passed=0;
 for(int mode=0;mode<6;mode++)try{Coverage c(2,2,1,9);if(mode==0)c.complete(9);if(mode==1)c.tile(0,1,1,1);if(mode==2)c.tile(0,2,0,1);if(mode==3){c.tile(0,1,0,1);c.tile(0,1,0,1);}if(mode>=4){c.tile(0,1,0,1);c.tile(0,1,1,1);c.tile(1,1,0,1);c.tile(1,1,1,1);if(mode==4)c.complete(9);else c.leftover(1);}throw std::runtime_error("control accepted");}catch(const std::runtime_error&e){require(std::string(e.what())!="control accepted","coverage negative control failed");passed++;}
 std::cout<<"{\"coverage_negative_controls_passed\":"<<passed<<",\"controls\":[\"incomplete\",\"gap\",\"wrong_extent\",\"duplicate\",\"missing_leftover\",\"wrong_leftover_ordinal\"]}"<<std::endl;
}
static void projective_controls(){
 Z pairs=0,double_cases=0,infinity_cases=0,inverse_cases=0,two_torsion=0,walk_samples=0,rejected_walks=0,x_checks=0;
 const unsigned long fields[][3]={{7,0,1},{11,5,5},{71,5,5}};
 for(const auto& field:fields){
   p=field[0];a=field[1];b=field[2];
   GEN E=ellinit(mkvec5(gen_0,gen_0,gen_0,gen(a),gen(b)),gen(p),DEFAULTPREC);
   std::vector<Point> group(1);
   for(unsigned long x=0;x<field[0];x++)for(unsigned long y=0;y<field[0];y++){
     Point P{Z(x),Z(y),false};if(on(P,a,b))group.push_back(P);
   }
   require(integer(ellcard(E,nullptr))==Z(group.size()),"enumerated group size mismatch");
   for(size_t i=0;i<group.size();i++){
     Point P=group[i];ProjectivePoint U=projective_lift(P);
     if(!P.inf){U={mod(4*P.x),mod(8*P.y),2};if(P.y==0)two_torsion++;}
     for(const Point&Q:group){
       require(equal(projective_affine(projective_mixed_add(U,Q)),elladd(E,gp(P),gp(Q))),"PARI mixed addition mismatch");pairs++;
       if(P.inf||Q.inf)infinity_cases++;
       if(!P.inf&&!Q.inf&&P.x==Q.x){if(mod(P.y+Q.y)==0)inverse_cases++;if(P.y==Q.y)double_cases++;}
     }
     Point step=group[(i+1)%group.size()];std::vector<Z> expected;GEN R=gp(P);bool contains_infinity=false;
     for(unsigned k=0;k<5;k++){if(lg(R)==2)contains_infinity=true;else expected.push_back(integer(gel(R,1)));R=elladd(E,R,gp(step));}
     for(unsigned long batch:{1UL,2UL,7UL}){
       WalkDiagnostics diagnostic;std::vector<Z> got;bool rejected=false;
       try{for_each_walk_x(P,step,5,batch,diagnostic,[&](unsigned long,const Z&x){got.push_back(x);});}
       catch(const std::runtime_error&e){require(std::string(e.what())=="projective walk infinity","unexpected walk rejection");rejected=true;}
       require(rejected==contains_infinity,"infinity walk rejection mismatch");
       if(rejected)rejected_walks++;else{require(got==expected,"PARI batch normalization mismatch");x_checks+=got.size();}
       walk_samples++;
     }
   }
 }
 std::cout<<"{\"scope\":\"small native arithmetic development controls; no timing\",\"fields\":[7,11,71],\"mixed_add_pari_pairs\":\""<<pairs<<"\",\"doubling_cases\":\""<<double_cases<<"\",\"infinity_cases\":\""<<infinity_cases<<"\",\"inverse_pair_cases\":\""<<inverse_cases<<"\",\"two_torsion_inputs\":\""<<two_torsion<<"\",\"walk_samples\":\""<<walk_samples<<"\",\"walk_infinity_rejections\":\""<<rejected_walks<<"\",\"normalization_x_checks\":\""<<x_checks<<"\",\"runtime_measurements\":null}"<<std::endl;
}
int main(int argc,char**argv){try{
 if(argc==2&&std::string(argv[1])=="--projective-controls"){
   pari_init(256000000,2000000);projective_controls();pari_close();flint_cleanup();return 0;
 }
 if(argc>=4&&std::string(argv[1])=="--evaluate"){
   bool compare=false,prepared=false,has_twist=false;Z baby_window=0,twist=1;unsigned long projective_batch=0;
   for(int i=4;i<argc;i++){
     std::string option=argv[i];
     if(option=="--pari-reference"&&!compare)compare=true;
     else if(option=="--prepared-norm"&&!prepared)prepared=true;
     else if(option=="--twist-scale"&&!has_twist&&i+1<argc){twist=Z(argv[++i]);has_twist=true;}
     else if(option=="--baby-window"&&baby_window==0&&i+1<argc){baby_window=Z(argv[++i]);require(baby_window>0,"baby window must be positive");}
     else if(option=="--projective-batch"&&projective_batch==0&&i+1<argc){projective_batch=bound(argv[++i]);require(projective_batch<=65536,"projective batch cap exceeds 65536");}
     else throw std::runtime_error("unknown/duplicate evaluation option");
   }
   unsigned long max_window=bound(argv[2]),block=bound(argv[3]);
   pari_init(256000000,2000000);evaluate_case(max_window,block,compare,baby_window,projective_batch,prepared,twist);pari_close();flint_cleanup();return 0;
 }
 if(argc==2&&std::string(argv[1])=="--target80-check"){
   Z ell("729845429276318409687041"),n=(ell-1)/2,B;Z rad=n/2;mpz_sqrt(B.get_mpz_t(),rad.get_mpz_t());Z G=n/(2*B);
   std::cout<<"{\"status\":\"incomplete_target_coverage\",\"ell\":\""<<ell<<"\",\"baby_roots_expected\":\""<<B<<"\",\"baby_roots_in_current_fixture\":4096,\"giant_count_expected\":\""<<G<<"\",\"kernel_pairs_expected\":\""<<n<<"\",\"quotient\":null,\"point_images\":null,\"endpoint_timings\":null,\"reason\":\"stored prefix does not cover complete native schedule; assembly refused\"}"<<std::endl;return 3;
 }
 require(argc==2&&std::string(argv[1])=="--fixtures","usage: complete_pipeline --fixtures | --target80-check");
 pari_init(256000000,2000000);coverage_tests();fixture("71",19,"4","35",3,2);fixture("6666679",10009,"9970","1003793",37,17);pari_close();flint_cleanup();return 0;
}catch(const std::exception&e){std::cerr<<"complete_pipeline: "<<e.what()<<"\n";return 2;}}
