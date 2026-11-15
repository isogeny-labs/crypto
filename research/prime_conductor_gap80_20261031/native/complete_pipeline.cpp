// Complete native square-root Velu wiring and independent fixture replay.
// Fixtures are controls, not an 80-bit conductor crossing or timing result.
#include <gmpxx.h>
#include <flint/flint.h>
#include <flint/fmpz.h>
#include <flint/fmpz_mod.h>
#include <flint/fmpz_mod_poly.h>
#include <flint/fmpz_poly.h>
#include <pari/pari.h>
#include <array>
#include <optional>
#include <vector>
#include <string>
#include <sstream>
#include <iostream>
#include <stdexcept>
#include <algorithm>
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
struct Result{Z ca,cb;std::vector<Point> images;Z covered;unsigned long B,G;};
static Result construct(Z ell,Point K,const std::vector<Point>&points,unsigned long block){
 require(ell>=3&&mpz_odd_p(ell.get_mpz_t())&&block>0,"invalid degree/block");require(on(K,a,b)&&!K.inf&&mul(K,ell).inf,"kernel witness");
 Z n=(ell-1)/2,rad=n/2,bs;mpz_sqrt(bs.get_mpz_t(),rad.get_mpz_t());if(bs<1)bs=1;
 require(mpz_fits_ulong_p(bs.get_mpz_t()),"window index exceeds native width");unsigned long B=bs.get_ui();Z gs=n/(2*bs);require(mpz_fits_ulong_p(gs.get_mpz_t()),"giant index width");unsigned long G=gs.get_ui();
 require(G>0,"use direct path for degree below five");Coverage coverage(B,G,block,n);
 std::vector<std::array<Z,3>> sums(points.size()+1);std::vector<bool> kernel(points.size());for(size_t k=0;k<points.size();k++){require(on(points[k],a,b),"source membership");kernel[k]=points[k].inf;}
 Poly z;fmpz_mod_poly_set_coeff_ui(z.z,1,1,ctx);
 for(unsigned long i=0;i<B;i+=std::min(block,B-i))for(unsigned long j=0;j<G;j+=std::min(block,G-j)){
   unsigned long ni=std::min(block,B-i),nj=std::min(block,G-j);Poly h=constant(1);
   for(unsigned long k=0;k<ni;k++){Point T=mul(K,2*(Z(i)+k)+1);require(!T.inf,"baby infinity");h=h*(z-constant(T.x));}
   std::vector<std::vector<std::optional<Jet>>> bins(points.size()+1);
   for(unsigned long k=0;k<nj;k++){
     Point T=mul(K,(4*(Z(j)+k)+2)*B);require(!T.inf,"giant infinity");Z x=T.x;
     Poly f0=(z-constant(x))*(z-constant(x));
     Poly f1=scale((scale(z,x)+constant(a))*(z+constant(x)),-2)-constant(4*b);
     Poly f2=(scale(z,x)-constant(a))*(scale(z,x)-constant(a))-scale(z+constant(x),4*b);
     Jet v;v[0]=f0;v[1]=f1;v[2]=f2;ingest(bins[0],v);
     for(size_t q=0;q<points.size();q++)if(!kernel[q]){
       Z alpha=points[q].x;Jet w;w[0]=scale(f0,alpha*alpha)+scale(f1,alpha)+f2;w[1]=scale(f0,2*alpha)+f1;w[2]=f0;ingest(bins[q+1],w);
     }
   }
   for(size_t q=0;q<sums.size();q++)if(q==0||!kernel[q-1]){
     auto v=norm(h,finish(bins[q]));if(!v){require(q>0,"infinity-jet collision");kernel[q-1]=true;}else for(int k=0;k<3;k++)sums[q][k]=mod(sums[q][k]+(*v)[k]);
   }
   coverage.tile(i,ni,j,nj);
 }
 std::array<Z,3> M={mod(-sums[0][0]),mod(-2*sums[0][1]),mod(-3*sums[0][2])};
 Z ordinal=0;
 for(Z index=4*Z(B)*G+1;index<ell;index+=2){
   Point T=mul(K,index);require(!T.inf,"leftover infinity");Z x=T.x,x2=mod(x*x);M[0]=mod(M[0]+x);M[1]=mod(M[1]+x2);M[2]=mod(M[2]+x2*x);
   for(size_t q=0;q<points.size();q++)if(!kernel[q]){
     if(points[q].x==x){kernel[q]=true;continue;}Z r=inverse(mod(points[q].x-x)),r2=mod(r*r);
     sums[q+1][0]=mod(sums[q+1][0]+r);sums[q+1][1]=mod(sums[q+1][1]-r2*inverse(2));sums[q+1][2]=mod(sums[q+1][2]+r2*r*inverse(3));
   }
   coverage.leftover(ordinal++);
 }
 coverage.complete(n); // No quotient or images may be emitted before this gate.
 Result out;out.ca=mod(a-10*(3*M[1]+a*n));out.cb=mod(b-14*(5*M[2]+3*a*M[0]+2*b*n));out.covered=n;out.B=B;out.G=G;
 for(size_t q=0;q<points.size();q++){
   if(kernel[q]){out.images.push_back({});continue;}
   Z x=points[q].x,y=points[q].y,fx=mod(x*x*x+a*x+b),df=mod(3*x*x+a);auto d=sums[q+1];
   Point H{mod(ell*x-2*M[0]-2*df*d[0]-8*fx*d[1]),mod(y*(ell-12*x*d[0]-12*df*d[1]-24*fx*d[2])),false};require(on(H,out.ca,out.cb),"destination membership");out.images.push_back(H);
 }
 return out;
}
static GEN gen(const Z&z){return strtoi(z.get_str().c_str());}
static Z integer(GEN z){char*s=GENtostr(lift(z));Z r(s);pari_free(s);return r;}
static GEN gp(Point u){return u.inf?mkvec(gen_0):mkvec2(gen(u.x),gen(u.y));}
static bool equal(Point u,GEN v){if(u.inf)return lg(v)==2;return lg(v)==3&&integer(gel(v,1))==u.x&&integer(gel(v,2))==u.y;}
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
int main(int argc,char**argv){try{
 if(argc==2&&std::string(argv[1])=="--target80-check"){
   Z ell("729845429276318409687041"),n=(ell-1)/2,B;Z rad=n/2;mpz_sqrt(B.get_mpz_t(),rad.get_mpz_t());Z G=n/(2*B);
   std::cout<<"{\"status\":\"incomplete_target_coverage\",\"ell\":\""<<ell<<"\",\"baby_roots_expected\":\""<<B<<"\",\"baby_roots_in_current_fixture\":4096,\"giant_count_expected\":\""<<G<<"\",\"kernel_pairs_expected\":\""<<n<<"\",\"quotient\":null,\"point_images\":null,\"endpoint_timings\":null,\"reason\":\"stored prefix does not cover complete native schedule; assembly refused\"}"<<std::endl;return 3;
 }
 require(argc==2&&std::string(argv[1])=="--fixtures","usage: complete_pipeline --fixtures | --target80-check");
 pari_init(256000000,2000000);coverage_tests();fixture("71",19,"4","35",3,2);fixture("6666679",10009,"9970","1003793",37,17);pari_close();flint_cleanup();return 0;
}catch(const std::exception&e){std::cerr<<"complete_pipeline: "<<e.what()<<"\n";return 2;}}
